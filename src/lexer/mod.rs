mod ascii;
mod skip;
mod token;
mod read;

pub use self::read::BufRead;
pub use self::skip::Comment as SkipComment;
pub use self::skip::Space as SkipSpace;

#[derive(Debug)]
pub enum Error {
    IO
}
