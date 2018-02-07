mod token;

use crate::io::Char;
use crate::error::Error;

use self::token::{Token, TokenKind};


pub fn tokenize<I: Iterator<Item=Result<Char, Error>>>(iter: I) -> impl Iterator<Item=Result<Token, Error>> {
    Tokens::from(iter)
}

pub struct Tokens<I> where I: Iterator<Item=Result<Char, Error>> {
    source: I,
    peeked: Option<Option<Result<Char, Error>>>
}

impl<I: Iterator<Item=Result<Char, Error>>> From<I> for Tokens<I> {
    fn from(source: I) -> Self {
        Self {
            source: source,
            peeked: None
        }
    }
}

impl<I: Iterator<Item=Result<Char, Error>>> Iterator for Tokens<I> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.peeked.take() {
            Some(v) => v,
            None => self.source.next(),
        };

        match item {
            Some(Ok(ref char)) if char.code >= b'A' && char.code <= b'Z' => {
                Some(Ok(Token {
                    kind: TokenKind::Code(char.code),
                    row: char.row,
                    start: char.col,
                    end: char.col
                }))
            }
            Some(Ok(ref char)) if (char.code >= b'0' && char.code <= b'9') || char.code == b'.' || char.code == b'+' || char.code == b'-' => {
                self.get_number(char)
            }
            Some(Ok(char)) => Some(Err(Error::Char(char))),
            Some(Err(err)) => Some(Err(err)),
            None => None
        }
    }
}

impl<I: Iterator<Item=Result<Char, Error>>> Tokens<I> {
    fn get_number(&mut self, first_char: &Char) -> Option<<Self as Iterator>::Item> {
        let mut number = Vec::with_capacity(16);
        let mut end = first_char.col;
        let mut is_float = first_char.code == b'.';

        number.push(first_char.code);

        loop {
            match self.source.next() {
                Some(Ok(ref char)) if (char.code >= b'0' && char.code <= b'9') => {
                    number.push(char.code);
                    end = char.col;
                }
                Some(Ok(Char { code: b'.', row: _, col })) if is_float == false => {
                    is_float = true;
                    number.push(b'.');
                    end = col;
                }
                item => {
                    self.peeked = Some(item);

                    break Some(Ok(Token {
                        kind: TokenKind::Number(number),
                        row: first_char.row,
                        start: first_char.col,
                        end: end
                    }))
                }
            }
        }
    }
}
