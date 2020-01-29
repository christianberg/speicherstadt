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

impl<R: Read> Read for ConstantSizeChunker<R> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if self.counter == self.chunk_size {
            return Ok(0);
        }
        let bytes_to_read = std::cmp::min(buffer.len(), self.chunk_size - self.counter);
        let buffer_slice = &mut buffer[..bytes_to_read];
        let result = self.inner.read(buffer_slice);
        if let Ok(bytes_read) = result {
            self.counter += bytes_read;
            if bytes_read == 0 {
                self.inner_consumed = true;
            }
        }
        result
    }
}

impl<R: Read> Iterator for ConstantSizeChunker<R> {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        if self.inner_consumed {
            None
        } else {
            self.counter = 0;
            Some(())
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
    fn constant_size_larger_than_input() {
        let input = "Short data";
        let source = input.as_bytes();
        let mut csc = ConstantSizeChunker::new(source, 2000);
        assert_eq!(csc.next(), Some(()));
        let mut output = String::new();
        csc.read_to_string(&mut output).unwrap();
        assert_eq!(input, output);
        assert_eq!(csc.next(), None);
    }

    #[test]
    fn constant_size_is_one() {
        let input = "abc";
        let source = input.as_bytes();
        let mut csc = ConstantSizeChunker::new(source, 1);

        for expected_chunk in ["a", "b", "c", ""].iter() {
            assert_eq!(csc.next(), Some(()));
            let mut output = String::new();
            csc.read_to_string(&mut output).unwrap();
            assert_eq!(output, *expected_chunk);
            assert_eq!(csc.read(&mut [0, 0, 0]).unwrap(), 0);
        }

        assert_eq!(csc.next(), None);
    }

    #[test]
    fn constant_size_is_two() {
        let input = "abcde";
        let source = input.as_bytes();
        let mut csc = ConstantSizeChunker::new(source, 2);

        for expected_chunk in ["ab", "cd", "e"].iter() {
            assert_eq!(csc.next(), Some(()));
            let mut output = String::new();
            csc.read_to_string(&mut output).unwrap();
            assert_eq!(output, *expected_chunk);
            assert_eq!(csc.read(&mut [0, 0, 0]).unwrap(), 0);
        }

        assert_eq!(csc.next(), None);
    }

    #[test]
    fn constant_size_is_three() {
        let input = "abcde";
        let source = input.as_bytes();
        let mut csc = ConstantSizeChunker::new(source, 3);

        for expected_chunk in ["abc", "de"].iter() {
            assert_eq!(csc.next(), Some(()));
            let mut output = String::new();
            csc.read_to_string(&mut output).unwrap();
            assert_eq!(output, *expected_chunk);
            assert_eq!(csc.read(&mut [0, 0, 0]).unwrap(), 0);
        }

        assert_eq!(csc.next(), None);
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
