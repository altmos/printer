#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub row: u32,
    pub start: u16,
    pub end: u16
}

#[derive(Debug)]
pub enum TokenKind {
    Letter(char),
    Number(f32)
}
