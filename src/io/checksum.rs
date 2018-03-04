use crate::error::Error;

pub struct Checksum<I: Iterator<Item = Result<u8, Error>>> {
    source: I,
    row: u32,
    sum: u8,
}

impl<I: Iterator<Item = Result<u8, Error>>> From<I> for Checksum<I> {
    fn from(source: I) -> Self {
        Self {
            source: source,
            row: 1,
            sum: 0,
        }
    }
}

impl<I: Iterator<Item = Result<u8, Error>>> Iterator for Checksum<I> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            item @ Some(Ok(b'\n')) => {
                self.sum = 0;
                self.row += 1;
                item
            }
            item @ Some(Ok(b'\r')) => {
                self.sum = 0;
                item
            }
            Some(Ok(b'*')) => {
                let mut checksum = 0;
                loop {
                    let number = match self.source.next() {
                        Some(Ok(b'0')) => 0,
                        Some(Ok(b'1')) => 1,
                        Some(Ok(b'2')) => 2,
                        Some(Ok(b'3')) => 3,
                        Some(Ok(b'4')) => 4,
                        Some(Ok(b'5')) => 5,
                        Some(Ok(b'6')) => 6,
                        Some(Ok(b'7')) => 7,
                        Some(Ok(b'8')) => 8,
                        Some(Ok(b'9')) => 9,
                        Some(Ok(_)) if self.sum != checksum => {
                            break Some(Err(Error::Checksum(self.sum, checksum, self.row)))
                        }
                        item => break item,
                    };
                    if checksum != 0 {
                        checksum *= 10;
                    }
                    checksum += number;
                }
            }
            Some(Ok(code)) => {
                self.sum ^= code;
                Some(Ok(code))
            }
            item => item,
        }
    }
}
