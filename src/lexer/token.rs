pub const NUMBER_LENGTH: usize = 16;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub row: u32,
    pub start: u16,
    pub end: u16
}

#[derive(Debug)]
pub enum TokenKind {
    Code(u8),
    Number([u8; NUMBER_LENGTH], usize)
}
