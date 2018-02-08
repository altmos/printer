use std::io::Error as IOError;
use crate::io::Char;
use crate::lexer::Token;

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    Checksum(u8, u8, u32),
    Char(Char),
    Token(Token),
}
