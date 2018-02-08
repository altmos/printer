use crate::lexer::Token;
use crate::error::Error;

#[derive(Debug)]
pub struct Word {
    letter: char,
    value: f32,
    row: u32,
    start: u16,
    end: u16,
}

pub fn words<I: Iterator<Item=Result<Token, Error>>>(iter: I) -> impl Iterator<Item=Result<Word, Error>> {
    Words::from(iter)
}

pub struct Words<I> where I: Iterator<Item=Result<Token, Error>> {
    source: I,
}

impl<I: Iterator<Item=Result<Token, Error>>> From<I> for Words<I> {
    fn from(source: I) -> Self {
        Self {
            source: source,
        }
    }
}

impl<I: Iterator<Item=Result<Token, Error>>> Iterator for Words<I> {
    type Item = Result<Word, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(Ok(Token::Letter {value: letter, start, row, ..})) => {
                match self.source.next() {
                    Some(Ok(Token::Number {value, end, ..})) => {
                        Some(Ok(Word { letter, value, row, start, end }))
                    }
                    Some(Ok(token)) => {
                        Some(Err(Error::Token(token)))
                    }
                    Some(Err(err)) => Some(Err(err)),
                    None => None
                }
            }
            Some(Ok(token)) => {
                Some(Err(Error::Token(token)))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None
        }
    }
}
