use crate::error::Error;

#[derive(Debug)]
pub struct Char {
    pub code: u8,
    pub row: u32,
    pub col: u16,
}

pub struct Chars<I> where I: Iterator<Item=Result<u8, Error>> {
    source: I,
    row: u32,
    col: u16,
    single_comment: bool,
    multi_comment: bool,
}

impl<I: Iterator<Item=Result<u8, Error>>> From<I> for Chars<I> {
    fn from(source: I) -> Self {
        Self {
            source: source,
            row: 1,
            col: 0,
            single_comment: false,
            multi_comment: false,
        }
    }
}

impl<I: Iterator<Item=Result<u8, Error>>> Iterator for Chars<I> {
    type Item = Result<Char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.source.next() {
                Some(Ok(b'\n')) => {
                    self.row += 1;
                    self.col = 0;

                    self.single_comment = false;
                }
                Some(Ok(b'\r')) => {
                    self.col = 0;
                }
                Some(Ok(b'(')) if !self.single_comment && !self.multi_comment => {
                    self.col += 1;

                    self.multi_comment = true;
                }
                Some(Ok(b')')) if self.multi_comment => {
                    self.col += 1;

                    self.multi_comment = false;
                }
                Some(Ok(b';')) if !self.single_comment && !self.multi_comment => {

                    self.single_comment = true;
                }
                Some(Ok(code)) => {
                    self.col += 1;

                    if code != b' ' && !self.single_comment && !self.multi_comment {
                        return Some(Ok(Char {
                            code: code,
                            row: self.row,
                            col: self.col,
                        }))
                    }
                }
                Some(Err(err)) => break Some(Err(err)),
                None => break None
            };
        }
    }
}
