mod buf_read;
mod checksum;
mod char;

use std::io::Read;
use crate::error::Error;

pub use self::char::Char;

pub fn buf_read<R: Read>(r: R) -> impl Iterator<Item = Result<Char, Error>> {
    char::Chars::from(checksum::Checksum::from(buf_read::BufRead::from(r)))
}
