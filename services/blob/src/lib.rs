use std::collections::HashMap;
use std::io::{Error, Read};

struct ConstantSizeChunker<R> {
    inner: R,
    inner_consumed: bool,
    chunk_size: usize,
    counter: usize,
}

impl<R: Read> ConstantSizeChunker<R> {
    fn new(inner: R, chunk_size: usize) -> Self {
        Self {
            inner,
            inner_consumed: false,
            chunk_size,
            counter: 0,
        }
    }
}

impl<R: Read> Iterator for ConstantSizeChunker<R> {
    type Item = std::io::Result<Vec<u8>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner_consumed {
            None
        } else {
            let mut next_chunk = Vec::<u8>::new();
            let mut buffer = vec![0u8; 1024]; // FIXME: use a sensible buffer size
            loop {
                match self.inner.read(buffer.as_mut_slice()) {
                    Ok(0) => break Some(Ok(next_chunk)),
                    Ok(length) => {
                        next_chunk.extend_from_slice(&buffer[..length]);
                    }
                    Err(e) => break Some(std::io::Result::Err(e)),
                }
            }
        }
    }
}

trait ChunkStore {
    fn store(&mut self, content: Vec<u8>) -> std::io::Result<String>;
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
        self.chunk_store.store(chunk)
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
        fn store(&mut self, content: Vec<u8>) -> std::io::Result<String> {
            let key = "foo".to_string();
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
