#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(usize),
    Eof,

    Number(f64),
    String(String),

    Colon,
    Comma,
    DoubleQuote,

    // {}
    LeftBrace,
    RightBrace,

    // []
    LeftBracket,
    RightBracket,
}
