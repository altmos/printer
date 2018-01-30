mod buf_read;
mod char;

use std::io::{Read, Error as IOError};

pub use self::char::Char;

pub type Error = IOError;

pub fn buf_read<R: Read>(r: R) -> impl Iterator<Item=Result<Char, Error>> {
    char::Chars::from(buf_read::BufRead::from(r))
}
