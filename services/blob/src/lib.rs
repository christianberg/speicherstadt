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
}
