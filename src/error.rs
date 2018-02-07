use std::io::Error as IOError;
use crate::io::Char;

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    Checksum(u8, u8, u32),
    Char(Char),
    NumberToLong(u32, u16),
}
