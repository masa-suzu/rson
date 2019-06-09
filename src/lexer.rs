use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    fn next_token(&mut self) -> Token {
        self.debug();

        self.skip_whitespace();

        let token = match self.ch {
            b'0'...b'9' => return self.consume_number(),
            b'-' => return self.consume_number(),

            b'\"' => return self.consume_string(),
            b':' => Token::Colon,
            b',' => Token::Comma,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b'[' => Token::LeftBracket,
            b']' => Token::RightBracket,

            b'\\' => self.found_illegal(),
            0 => Token::Eof,
            ch => {
                if ch.is_ascii_alphabetic() {
                    return self.consume_keyword();
                } else {
                    return self.found_illegal();
                }
            }
        };

        self.read_char();

        token
    }

    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;
        if self.ch == b'-' {
            self.read_char();
        }
        loop {
            match self.ch {
                b'0'...b'9' => self.read_char(),
                b'.' => self.read_char(),
                _ => break,
            }
        }

        let consumed = match self.ch {
            0 => &self.input[start_pos..=self.pos],
            _ => &self.input[start_pos..self.pos],
        };

        match consumed.parse::<f64>().ok() {
            Some(n) => Token::Number(n),
            None => self.found_illegal(),
        }
    }

    fn consume_string(&mut self) -> Token {
        let start_pos = self.next_pos;

        loop {
            self.read_char();
            match self.ch {
                b'\"' => {
                    self.read_char();
                    break;
                }
                b'\\' => {
                    self.read_char();
                    match self.ch {
                        b'\"' => {}
                        b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {}
                        _ => return self.found_illegal(),
                    }
                }
                0 => return Token::Illegal(self.pos),
                _ => {}
            }
        }

        let consumed = match self.ch {
            0 => &self.input[start_pos..self.pos],
            _ => &self.input[start_pos..self.pos - 1],
        };
        Token::String(consumed.to_string())
    }

    fn consume_keyword(&mut self) -> Token {
        let start_pos = self.pos;

        while self.ch.is_ascii_alphanumeric() {
            self.read_char()
        }

        let consumed = match self.ch {
            0 => &self.input[start_pos..=self.pos],
            _ => &self.input[start_pos..self.pos],
        };

        match &*consumed {
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => self.found_illegal(),
        }
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
            return;
        }

        self.ch = self.input.as_bytes()[self.next_pos];
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => self.read_char(),
                _ => break,
            }
        }
    }

    fn found_illegal(&self) -> Token {
        self.debug();

        Token::Illegal(self.pos)
    }
    fn debug(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        print!("{}", self.input);

        if !self.input.ends_with('\n') {
            println!()
        }

        for _ in 0..self.pos {
            print!(" ");
        }
        println!("^ <- current = {}", self.ch);

        println!();
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        match self.next_token() {
            Token::Eof => None,
            x => Some(x),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn next_token() {
        let input = r##"
{
    "number" : -1.1,
    "string" : "x",
    "array" : [ 1.0, 2.0, 3.1],
    "true" : true,
    "false" : false,
    "null" : null,
    "escape_sequence" : "\\\/\b\f\n\r\t"
}
"##;
        let want = vec![
            Token::LeftBrace,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(-1.1),
            Token::Comma,
            Token::String("string".to_string()),
            Token::Colon,
            Token::String("x".to_string()),
            Token::Comma,
            Token::String("array".to_string()),
            Token::Colon,
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.1),
            Token::RightBracket,
            Token::Comma,
            Token::String("true".to_string()),
            Token::Colon,
            Token::True,
            Token::Comma,
            Token::String("false".to_string()),
            Token::Colon,
            Token::False,
            Token::Comma,
            Token::String("null".to_string()),
            Token::Colon,
            Token::Null,
            Token::Comma,
            Token::String("escape_sequence".to_string()),
            Token::Colon,
            Token::String("\\\\\\/\\b\\f\\n\\r\\t".to_string()),
            Token::RightBrace,
        ];

        let got: Vec<Token> = Lexer::new(input).map(|x| x).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn next_token_keywords() {
        let input = "true false null";
        let want = vec![Token::True, Token::False, Token::Null];

        let got: Vec<Token> = Lexer::new(input).map(|x| x).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn next_token_with_illegal() {
        let input = "\\123 x 123";
        let want = vec![
            Token::Illegal(0),
            Token::Number(123.0),
            Token::Illegal(6),
            Token::Number(123.0),
        ];

        let got: Vec<Token> = Lexer::new(input).map(|x| x).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn next_token_with_escape_sequence() {
        let input = "\" \\\" \\\\ \\\" \\/ \\b \\f \\n \\r \\t \"";
        let want = vec![Token::String(
            " \\\" \\\\ \\\" \\/ \\b \\f \\n \\r \\t ".to_string(),
        )];

        let got: Vec<Token> = Lexer::new(input).map(|x| x).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn next_token_string_not_terminated() {
        let input = "1\"xxx";
        let want = vec![Token::Number(1.0), Token::Illegal(4)];

        let got: Vec<Token> = Lexer::new(input).map(|x| x).collect();
        assert_eq!(want, got);
    }
}
