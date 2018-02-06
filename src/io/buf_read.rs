use crate::error::Error;
use std::io::Read;

const BUF_SIZE: usize = 1024 * 4;

pub struct BufRead<Read> {
    buf: [u8; BUF_SIZE],
    buf_pos: usize,
    buf_size: usize,
    source: Read,
}

impl<R: Read> From<R> for BufRead<R> {
    fn from(read: R) -> Self {
        Self {
            buf: [0; BUF_SIZE],
            buf_pos: BUF_SIZE,
            buf_size: BUF_SIZE,
            source: read
        }
    }
}

impl<R: Read> Iterator for BufRead<R> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.buf_pos >= self.buf_size {
                self.buf_pos = 0;

                self.buf_size = match self.source.read(&mut self.buf) {
                    Ok(0) => return None,
                    Ok(buf_size) => buf_size,
                    Err(err) => return Some(Err(Error::IO(err)))
                };
            }
            let code = self.buf[self.buf_pos];

            self.buf_pos += 1;

            break Some(Ok(code));
        }
    }
}
