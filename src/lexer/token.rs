#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub row: u32,
    pub start: u16,
    pub end: u16
}

#[derive(Debug)]
pub enum TokenKind {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Number(Vec<u8>)
}
