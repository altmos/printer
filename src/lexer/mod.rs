use crate::io::Char;
use crate::error::Error;

#[derive(Debug)]
pub enum Token {
    Letter {
        value: char,
        row: u32,
        start: u16,
        end: u16
    },
    Number {
        value: f32,
        row: u32,
        start: u16,
        end: u16
    },
}

pub fn tokens<I: Iterator<Item=Result<Char, Error>>>(iter: I) -> impl Iterator<Item=Result<Token, Error>> {
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
                Some(Ok(Token::Letter {
                    value: char.code as char,
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
        let mut is_float = false;
        let mut end = first_char.col;
        let mut dot_pos = 1.;

        let mut number = match first_char.code {
            b'.' => {
                is_float = true;
                0.
            }
            b'+' | b'-' => 0.,
            b'0' => 0.,
            b'1' => 1.,
            b'2' => 2.,
            b'3' => 3.,
            b'4' => 4.,
            b'5' => 5.,
            b'6' => 6.,
            b'7' => 7.,
            b'8' => 8.,
            b'9' => 9.,
            _ => unreachable!()
        };

        loop {
            match self.source.next() {
                Some(Ok(ref char)) if char.code == b'0' => {
                    end = char.col;
                    number = number * 10.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'1' => {
                    end = char.col;
                    number = number * 10. + 1.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'2' => {
                    end = char.col;
                    number = number * 10. + 2.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'3' => {
                    end = char.col;
                    number = number * 10. + 3.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'4' => {
                    end = char.col;
                    number = number * 10. + 4.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'5' => {
                    end = char.col;
                    number = number * 10. + 5.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'6' => {
                    end = char.col;
                    number = number * 10. + 6.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'7' => {
                    end = char.col;
                    number = number * 10. + 7.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'8' => {
                    end = char.col;
                    number = number * 10. + 8.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(ref char)) if char.code == b'9' => {
                    end = char.col;
                    number = number * 10. + 9.;
                    if is_float { dot_pos *= 10.; }
                }
                Some(Ok(Char { code: b'.', row: _, col })) if !is_float => {
                    end = col;
                    is_float = true;
                }
                item => {
                    self.peeked = Some(item);
                    number /= dot_pos;

                    if first_char.code == b'-' {
                        number *= -1.;
                    }

                    break Some(Ok(Token::Number {
                        value: number,
                        row: first_char.row,
                        start: first_char.col,
                        end: end
                    }))
                }
            }
        }
    }
}
