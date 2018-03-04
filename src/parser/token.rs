use std::str;
use std::fmt;
use crate::io::Char;
use crate::error::Error;

pub enum Token {
    Letter {
        value: u8,
        row: u32,
        start: u16,
        end: u16,
    },
    Number {
        value: [u8; 20],
        len: usize,
        row: u32,
        start: u16,
        end: u16,
    },
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::Letter {
                value,
                row,
                start,
                end,
            } => write!(
                f,
                "Letter: {:?}, Row: {:?}, Start: {:?}, End: {:?}",
                value as char, row, start, end
            ),
            &Token::Number {
                value,
                len,
                row,
                start,
                end,
            } => write!(
                f,
                "Number: {:?}, Row: {:?}, Start: {:?}, End: {:?}",
                unsafe { str::from_utf8_unchecked(&value[0..len]) },
                row,
                start,
                end
            ),
        }
    }
}

pub struct Tokens<I: Iterator<Item = Result<Char, Error>>> {
    source: I,
    peeked: Option<Option<Result<Char, Error>>>,
}

impl<I: Iterator<Item = Result<Char, Error>>> From<I> for Tokens<I> {
    fn from(source: I) -> Self {
        Self {
            source: source,
            peeked: None,
        }
    }
}
impl<I: Iterator<Item = Result<Char, Error>>> Iterator for Tokens<I> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.peeked.take() {
            Some(v) => v,
            None => self.source.next(),
        };

        match item {
            Some(Ok(ref char)) if char.code >= b'A' && char.code <= b'Z' => {
                Some(Ok(Token::Letter {
                    value: char.code,
                    row: char.row,
                    start: char.col,
                    end: char.col,
                }))
            }
            Some(Ok(Char { code, col, row, .. }))
                if (code >= b'0' && code <= b'9') || code == b'.' || code == b'-'
                    || code == b'+' =>
            {
                let mut is_float = code == b'.';
                let mut end = col;
                let mut pos = 1;
                let mut number = [code; 20];

                loop {
                    match self.source.next() {
                        Some(Ok(Char { code, col, .. })) if code >= b'0' && code <= b'9' => {
                            end = col;
                            number[pos] = code;
                            pos += 1;
                        }
                        Some(Ok(Char {
                            code: b'.', col, ..
                        })) if !is_float =>
                        {
                            end = col;
                            number[pos] = b'.';
                            pos += 1;
                            is_float = true;
                        }
                        Some(Ok(char)) => {
                            self.peeked = Some(Some(Ok(char)));

                            break Some(Ok(Token::Number {
                                value: number,
                                len: pos,
                                row: row,
                                start: col,
                                end: end,
                            }));
                        }
                        Some(Err(err)) => break Some(Err(err)),
                        None => break None,
                    }
                }
            }
            Some(Ok(char)) => Some(Err(Error::Char(char))),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}
