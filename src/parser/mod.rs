mod token;

use crate::error::Error;
use crate::io::Char;
use self::token::Tokens;
pub use self::token::Token;

pub fn tokens<I: Iterator<Item = Result<Char, Error>>>(
    iter: I,
) -> impl Iterator<Item = Result<Token, Error>> {
    Tokens::from(iter)
}
