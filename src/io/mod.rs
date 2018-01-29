mod buf_read;

use std::io::{Read, Error as IOError};

pub type Error = IOError;

#[derive(Debug)]
pub struct Char {
    pub code: u8,
    pub row: u32,
    pub col: u16,
}

pub fn buf_read<R: Read>(r: R) -> impl Iterator<Item=Result<Char, Error>> {
    buf_read::BufRead::from(r)
}
