use super::{Char, Error};
use crate::ascii;
use std::io::{Read, ErrorKind as IOErrorKind};

const BUF_SIZE: usize = 1024 * 4;

pub struct BufRead<Read> {
    buf: [u8; BUF_SIZE],
    pos: usize,
    size: usize,
    source: Read,
    row: u32,
    col: u16,
    single_comment: bool,
    multi_comment: bool,
}

impl<R: Read> From<R> for BufRead<R> {
    fn from(read: R) -> Self {
        Self {
            buf: [0; BUF_SIZE],
            pos: BUF_SIZE,
            size: BUF_SIZE,
            source: read,
            row: 1,
            col: 0,
            single_comment: false,
            multi_comment: false,
        }
    }
}

impl<R: Read> Iterator for BufRead<R> {
    type Item = Result<Char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos >= self.size {
                self.pos = 0;

                self.size = match self.source.read(&mut self.buf) {
                    Ok(0) => return None,
                    Ok(size) => size,
                    Err(e) => return Some(Err(e))
                };
            }

            match self.buf[self.pos] {
                ascii::LINEFEED => {
                    self.pos += 1;
                    self.row += 1;
                    self.col = 0;

                    self.single_comment = false;
                }
                ascii::CARRIAGE_RETURN => {
                    self.pos += 1;
                    self.col = 0;
                }
                ascii::LEFT_PARENTHESIS if !self.single_comment && !self.multi_comment => {
                    self.pos += 1;
                    self.col += 1;

                    self.multi_comment = true;
                }
                ascii::RIGHT_PARENTHESIS if self.multi_comment => {
                    self.pos += 1;
                    self.col += 1;

                    self.multi_comment = false;
                }
                ascii::SEMICOLON if !self.single_comment && !self.multi_comment => {
                    self.pos += 1;

                    self.single_comment = true;
                }
                code => {
                    self.pos += 1;
                    self.col += 1;

                    if code != ascii::SPACE && !self.single_comment && !self.multi_comment {
                        return Some(Ok(Char {
                            code: code,
                            row: self.row,
                            col: self.col,
                        }))
                    }
                }
            };
        }
    }
}
