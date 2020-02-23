use iron::prelude::*;
use iron::status;
use router::Router;
use serde::{Serialize, Serializer};
use std::io::Read;

const HASH_ALGORITHM: multihash::Hash = multihash::Hash::SHA2256;
const ENCODING: multibase::Base = multibase::Base::Base58btc;

struct ChunkHash(multihash::Multihash);

impl ChunkHash {
    fn new(content: &Vec<u8>) -> Self {
        Self(multihash::encode(HASH_ALGORITHM, content).unwrap())
    }
}

impl std::fmt::Display for ChunkHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", multibase::encode(ENCODING, self.0.as_bytes()))
    }
}

impl Serialize for ChunkHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct Chunk {
    hash: ChunkHash,
    content: Vec<u8>,
}

impl Chunk {
    fn new(content: Vec<u8>) -> Self {
        Self {
            hash: ChunkHash::new(&content),
            content,
        }
    }
}

#[derive(Serialize)]
struct ChunkRef {
    hash: ChunkHash,
    length: usize,
}

impl ChunkRef {
    fn from_chunk(chunk: Chunk) -> Self {
        Self {
            hash: chunk.hash,
            length: chunk.content.len(),
        }
    }
}

#[derive(Serialize)]
struct Blob {
    chunks: Vec<ChunkRef>,
}

impl Blob {
    fn new() -> Self {
        Self { chunks: vec![] }
    }
}

struct ConstantSizeChunker<R> {
    inner: R,
    chunk_size: usize,
}

impl<R: Read> ConstantSizeChunker<R> {
    fn new(inner: R, chunk_size: usize) -> Self {
        Self { inner, chunk_size }
    }
}

impl<R: Read> Iterator for ConstantSizeChunker<R> {
    type Item = std::io::Result<Chunk>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next_chunk = Vec::<u8>::new();
        // An 8kB read buffer is used (same as the default buffer size
        // of BufReader), unless the chunk size is smaller
        let mut buffer = vec![0u8; std::cmp::min(self.chunk_size, 8192)];
        let mut bytes_remaining = self.chunk_size;
        let mut slice = buffer.as_mut_slice();
        loop {
            if bytes_remaining < slice.len() {
                slice = &mut slice[..bytes_remaining];
            }
            match self.inner.read(slice) {
                Ok(0) => {
                    if next_chunk.is_empty() {
                        break None;
                    } else {
                        break Some(Ok(Chunk::new(next_chunk)));
                    }
                }
                Ok(length) => {
                    next_chunk.extend_from_slice(&slice[..length]);
                    bytes_remaining -= length;
                }
                Err(e) => break Some(std::io::Result::Err(e)),
            }
        }
    }
}

trait ChunkStore {
    fn store(&mut self, chunk: &Chunk) -> std::io::Result<()>;
}

struct BlobStore<C> {
    chunk_store: C,
}

impl<C: ChunkStore> BlobStore<C> {
    fn new(chunk_store: C) -> Self {
        Self { chunk_store }
    }

    fn store(&mut self, r: impl Read) -> std::io::Result<String> {
        let mut blob = Blob::new();
        let chunks = ConstantSizeChunker::new(r, 4);
        for chunk in chunks {
            let chunk = chunk?;
            self.chunk_store.store(&chunk)?;
            let cr = ChunkRef::from_chunk(chunk);
            blob.chunks.push(cr);
        }
        let meta_chunk = Chunk::new(serde_json::to_vec(&blob)?);
        self.chunk_store.store(&meta_chunk)?;
        Ok(meta_chunk.hash.to_string())
    }
}

struct HttpChunkStore {
    base_url: reqwest::Url,
    client: reqwest::Client,
}

impl ChunkStore for HttpChunkStore {
    fn store(&mut self, chunk: &Chunk) -> std::io::Result<()> {
        let url = format!("{}/{}", self.base_url, chunk.hash);
        // TODO can we do without clone() here?
        // TODO error handling
        self.client.post(&url).body(chunk.content.clone()).send();
        Ok(())
    }
}

fn handle_post(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Created))
}

pub fn start_server(port: u16) -> std::io::Result<()> {
    let mut router = Router::new();
    router.post("/blobs", handle_post, "blobs_post");
    let chain = Chain::new(router);
    Iron::new(chain)
        .http(("localhost", port))
        .expect("Unable to start server");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn whole_input_fits_in_one_chunk() {
        let input = vec![1, 2, 3];
        let mut csc = ConstantSizeChunker::new(input.as_slice(), 4);
        let chunk = csc.next().unwrap().unwrap();
        assert_eq!(chunk.content, input);
        assert!(csc.next().is_none());
    }

    #[test]
    fn input_is_split_into_two_chunks() {
        let input = vec![1, 2, 3];
        let mut csc = ConstantSizeChunker::new(input.as_slice(), 2);
        let chunk1 = csc.next().unwrap().unwrap();
        assert_eq!(chunk1.content, vec![1, 2]);
        let chunk2 = csc.next().unwrap().unwrap();
        assert_eq!(chunk2.content, vec![3]);
        assert!(csc.next().is_none());
    }

    #[test]
    fn input_size_equals_chunk_size() {
        let input = vec![1, 2, 3];
        let mut csc = ConstantSizeChunker::new(input.as_slice(), 3);
        let chunk = csc.next().unwrap().unwrap();
        assert_eq!(chunk.content, input);
        assert!(csc.next().is_none());
    }

    #[test]
    fn input_can_be_reassembled_from_chunks() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let csc = ConstantSizeChunker::new(input.as_slice(), 3);
        let mut output = vec![];
        for chunk in csc {
            output.extend(chunk.unwrap().content);
        }
        assert_eq!(input, output);
    }

    #[test]
    fn chunk_shows_hash() {
        let cr = Chunk::new("hello world".as_bytes().to_vec());
        assert_eq!(
            cr.hash.0,
            multihash::Multihash::from_bytes(vec![
                18, 32, 185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250,
                196, 132, 239, 227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233
            ])
            .unwrap()
        );
    }

    #[test]
    fn chunk_string_hash() {
        let cr = Chunk::new("hello world".as_bytes().to_vec());
        assert_eq!(
            cr.hash.to_string(),
            "zQmaozNR7DZHQK1ZcU9p7QdrshMvXqWK6gpu5rmrkPdT3L4"
        );
    }

    struct ChunkStoreFake {
        chunks: HashMap<String, Vec<u8>>,
    }

    impl ChunkStoreFake {
        fn new() -> Self {
            Self {
                chunks: HashMap::new(),
            }
        }
    }

    impl ChunkStore for ChunkStoreFake {
        fn store(&mut self, chunk: &Chunk) -> std::io::Result<()> {
            let key = chunk.hash.to_string();
            self.chunks.insert(key.clone(), chunk.content.clone());
            Ok(())
        }
    }

    #[test]
    fn blob_store_calls_chunk_store() {
        let mut store = BlobStore::new(ChunkStoreFake::new());
        store
            .store("This is my important payload".as_bytes())
            .unwrap();

        assert!(store.chunk_store.chunks.keys().len() > 1);
    }

    #[test]
    fn blob_store_returns_blob_id() {
        let mut store = BlobStore::new(ChunkStoreFake::new());
        let result = store
            .store("This is my important payload".as_bytes())
            .unwrap();

        assert_eq!(result, "zQmbMf6UzYixw32VSmPi2WDNFFizmVAaANqHAQoXuZMWAjc");
    }
}