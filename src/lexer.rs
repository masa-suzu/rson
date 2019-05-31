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

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
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

            0 => Token::Eof,
            _ => {
                self.debug();
                Token::Illegal(self.pos)
            }
        };

        self.read_char();

        return token;
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
            0 => &self.input[start_pos..self.pos + 1],
            _ => &self.input[start_pos..self.pos],
        };

        match consumed.parse::<f64>().ok() {
            Some(n) => Token::Number(n),
            None => Token::Illegal(self.pos),
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

    fn debug(&mut self) {
        if !cfg!(debug_assertions) {
            return;
        }
        print!("{}", self.input);

        if !self.input.ends_with("\n") {
            println!()
        }

        for _ in 0..self.pos {
            print!(" ");
        }
        println!("^ <- self.ch = {}", self.ch);
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
    "number" : -1.0,
    "string" : "x",
    "array" : [ 1, 2, 3],
}
"##;
        let tests = vec![
            Token::LeftBrace,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(-1.0),
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
            Token::Number(3.0),
            Token::RightBracket,
            Token::Comma,
            Token::RightBrace,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for want in tests {
            let got = lexer.next_token();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn next_token_with_illegal() {
        let input = "123 x 123";
        let tests = vec![
            Token::Number(123.0),
            Token::Illegal(4),
            Token::Number(123.0),
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for want in tests {
            let got = lexer.next_token();
            assert_eq!(want, got);
        }
    }
    #[test]
    fn next_token_string_not_terminated() {
        let input = "1\"xxx";
        let tests = vec![Token::Number(1.0), Token::Illegal(4), Token::Eof];

        let mut lexer = Lexer::new(input);

        for want in tests {
            let got = lexer.next_token();
            assert_eq!(want, got);
        }
    }
}
