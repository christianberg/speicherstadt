use multihash::{encode, Hash, Multihash};
use std::collections::HashMap;
use std::io::{Error, Read};

struct ChunkRef {
    length: usize,
    hash: Multihash,
}

impl ChunkRef {
    fn from_bytes(content: &[u8]) -> Self {
        Self {
            length: content.len(),
            hash: encode(Hash::SHA2256, content).unwrap(),
        }
    }

    fn id(&self) -> String {
        multibase::encode(multibase::Base::Base58btc, self.hash.as_bytes())
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
    type Item = std::io::Result<Vec<u8>>;
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
                        break Some(Ok(next_chunk));
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
    fn store(&mut self, key: &str, content: Vec<u8>) -> std::io::Result<String>;
}

struct BlobStore<C> {
    chunk_store: C,
}

impl<C: ChunkStore> BlobStore<C> {
    fn new(chunk_store: C) -> Self {
        Self { chunk_store }
    }

    fn store(&mut self, mut r: impl Read) -> std::io::Result<String> {
        let mut chunk: Vec<u8> = vec![];
        std::io::copy(&mut r, &mut chunk)?;
        self.chunk_store.store("foo", chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_input_fits_in_one_chunk() {
        let input = vec![1, 2, 3];
        let mut csc = ConstantSizeChunker::new(input.as_slice(), 4);
        let chunk = csc.next().unwrap().unwrap();
        assert_eq!(chunk, input);
        assert!(csc.next().is_none());
    }

    #[test]
    fn input_is_split_into_two_chunks() {
        let input = vec![1, 2, 3];
        let mut csc = ConstantSizeChunker::new(input.as_slice(), 2);
        let chunk1 = csc.next().unwrap().unwrap();
        assert_eq!(chunk1, vec![1, 2]);
        let chunk2 = csc.next().unwrap().unwrap();
        assert_eq!(chunk2, vec![3]);
        assert!(csc.next().is_none());
    }

    #[test]
    fn input_size_equals_chunk_size() {
        let input = vec![1, 2, 3];
        let mut csc = ConstantSizeChunker::new(input.as_slice(), 3);
        let chunk = csc.next().unwrap().unwrap();
        assert_eq!(chunk, input);
        assert!(csc.next().is_none());
    }

    #[test]
    fn input_can_be_reassembled_from_chunks() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let mut csc = ConstantSizeChunker::new(input.as_slice(), 3);
        let mut output = vec![];
        for chunk in csc {
            output.extend(chunk.unwrap());
        }
        assert_eq!(input, output);
    }

    #[test]
    fn chunk_ref_shows_length() {
        let cr = ChunkRef::from_bytes("hello world".as_bytes());
        assert_eq!(cr.length, 11);
    }

    #[test]
    fn chunk_ref_shows_hash() {
        let cr = ChunkRef::from_bytes("hello world".as_bytes());
        assert_eq!(
            cr.hash,
            Multihash::from_bytes(vec![
                18, 32, 185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250,
                196, 132, 239, 227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233
            ])
            .unwrap()
        );
    }

    #[test]
    fn chunk_string_id() {
        let cr = ChunkRef::from_bytes("hello world".as_bytes());
        assert_eq!(cr.id(), "zQmaozNR7DZHQK1ZcU9p7QdrshMvXqWK6gpu5rmrkPdT3L4");
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
        fn store(&mut self, key: &str, content: Vec<u8>) -> std::io::Result<String> {
            let key = key.to_string();
            self.chunks.insert(key.clone(), content);
            Ok(key)
        }
    }

    #[test]
    fn store_blob() {
        let input = "This is my important payload";
        let reader = input.as_bytes();

        let mut store = BlobStore::new(ChunkStoreFake::new());
        let result = store.store(reader).unwrap();

        assert_eq!(result, "foo");
        assert!(store.chunk_store.chunks.contains_key("foo"));
    }
}
