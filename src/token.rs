#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(usize),
    Eof,

    True,
    False,
    Null,

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
