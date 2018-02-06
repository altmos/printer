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
            Some(Ok(Char { code: b'A', row, col })) => map_to_token(TokenKind::A, row, col),
            Some(Ok(Char { code: b'B', row, col })) => map_to_token(TokenKind::B, row, col),
            Some(Ok(Char { code: b'C', row, col })) => map_to_token(TokenKind::C, row, col),
            Some(Ok(Char { code: b'D', row, col })) => map_to_token(TokenKind::D, row, col),
            Some(Ok(Char { code: b'E', row, col })) => map_to_token(TokenKind::E, row, col),
            Some(Ok(Char { code: b'F', row, col })) => map_to_token(TokenKind::F, row, col),
            Some(Ok(Char { code: b'G', row, col })) => map_to_token(TokenKind::G, row, col),
            Some(Ok(Char { code: b'H', row, col })) => map_to_token(TokenKind::H, row, col),
            Some(Ok(Char { code: b'I', row, col })) => map_to_token(TokenKind::I, row, col),
            Some(Ok(Char { code: b'J', row, col })) => map_to_token(TokenKind::J, row, col),
            Some(Ok(Char { code: b'K', row, col })) => map_to_token(TokenKind::K, row, col),
            Some(Ok(Char { code: b'L', row, col })) => map_to_token(TokenKind::L, row, col),
            Some(Ok(Char { code: b'M', row, col })) => map_to_token(TokenKind::M, row, col),
            Some(Ok(Char { code: b'N', row, col })) => map_to_token(TokenKind::N, row, col),
            Some(Ok(Char { code: b'O', row, col })) => map_to_token(TokenKind::O, row, col),
            Some(Ok(Char { code: b'P', row, col })) => map_to_token(TokenKind::P, row, col),
            Some(Ok(Char { code: b'Q', row, col })) => map_to_token(TokenKind::Q, row, col),
            Some(Ok(Char { code: b'R', row, col })) => map_to_token(TokenKind::R, row, col),
            Some(Ok(Char { code: b'S', row, col })) => map_to_token(TokenKind::S, row, col),
            Some(Ok(Char { code: b'T', row, col })) => map_to_token(TokenKind::T, row, col),
            Some(Ok(Char { code: b'U', row, col })) => map_to_token(TokenKind::U, row, col),
            Some(Ok(Char { code: b'V', row, col })) => map_to_token(TokenKind::V, row, col),
            Some(Ok(Char { code: b'W', row, col })) => map_to_token(TokenKind::W, row, col),
            Some(Ok(Char { code: b'X', row, col })) => map_to_token(TokenKind::X, row, col),
            Some(Ok(Char { code: b'Y', row, col })) => map_to_token(TokenKind::Y, row, col),
            Some(Ok(Char { code: b'Z', row, col })) => map_to_token(TokenKind::Z, row, col),
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

        number.push(first_char.code);

        loop {
            match self.source.next() {
                Some(Ok(ref char)) if (char.code >= b'0' && char.code <= b'9') => {
                    number.push(char.code);
                    end = char.col;
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

fn map_to_token(kind: TokenKind, row: u32, col: u16) -> Option<Result<Token, Error>> {
    Some(Ok(Token {
        kind: kind,
        row: row,
        start: col,
        end: col
    }))
}
