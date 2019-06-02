use crate::json::Value;
use crate::token::Token;

use crate::json::Object;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Parser<'a> {
    tokens: &'a mut Iterator<Item = Token>,
    current_token: Token,
    next_token: Token,
}

#[derive(Debug)]
pub enum Error {
    FoundIllegalToken(usize),
    FoundUnExpectedToken(Token, Token),
    FoundUnTerminatedBrace,
    FailedParseValue(Token),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::FoundIllegalToken(at) => write!(f, "Found Illegal Token at {}.", at),
            Error::FoundUnExpectedToken(want, got) => {
                write!(f, "Found Token {:?}, want Token {:?}", got, want)
            }
            Error::FoundUnTerminatedBrace => write!(f, "Not terminated brace."),
            Error::FailedParseValue(t) => write!(f, "Failed to parse value from Token {:?}.", t),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Iterator<Item = Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_token: Token::Eof,
            next_token: Token::Eof,
        };

        parser.advance_token();
        parser.advance_token();

        parser
    }

    pub fn parse(&mut self) -> Result<Object, Error> {
        match self.current_token.to_owned() {
            Token::LeftBrace => match self.parse_object() {
                Ok(v) => match v {
                    Value::Object(o) => {
                        Parser::dump(&Value::Object(o.clone()));
                        Ok(o)
                    }
                    _ => panic!("parse_object must return Ok(Value::Object) or Err(Error)"),
                },
                Err(e) => return Err(e),
            },
            t => Err(Error::FoundUnExpectedToken(Token::LeftBrace, t)),
        }
    }

    fn parse_object(&mut self) -> Result<Value, Error> {
        let mut kvs: HashMap<String, Value> = HashMap::new();

        loop {
            match self.parse_key_value_pair() {
                Ok((k, v)) => {
                    kvs.insert(k, v);
                }
                Err(e) => return Err(e),
            }

            match self.next_token {
                Token::RightBrace => {
                    return Ok(Value::Object(kvs));
                }
                Token::Eof => return Err(Error::FoundUnTerminatedBrace),
                Token::Comma => {
                    self.advance_token();
                }
                _ => {}
            }
        }
    }

    fn parse_key_value_pair(&mut self) -> Result<(String, Value), Error> {
        let k = match self.next_token.to_owned() {
            Token::String(s) => s,
            t => {
                return Err(Error::FoundUnExpectedToken(
                    Token::String("any_key".to_string()),
                    t,
                ))
            }
        };

        self.advance_token();

        if Token::Colon != self.next_token {
            return Err(Error::FoundUnExpectedToken(
                Token::Colon,
                self.next_token.to_owned(),
            ));
        }

        self.advance_token();

        let v = match self.parse_value() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok((k, v))
    }

    fn parse_value(&mut self) -> Result<Value, Error> {
        let v = match self.next_token.to_owned() {
            Token::Number(i) => Value::Number(i),
            Token::String(s) => Value::String(s),
            Token::True => Value::Boolean(true),
            Token::False => Value::Boolean(false),
            Token::Null => Value::Null,
            _ => {
                self.advance_token();
                match self.parse() {
                    Ok(o) => Value::Object(o),
                    Err(e) => return Err(e),
                }
            }
        };

        self.advance_token();

        Ok(v)
    }

    fn advance_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = match self.tokens.next() {
            Some(t) => t,
            None => Token::Eof,
        }
    }

    fn debug(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("current: {:?}", self.current_token);
        println!("next   : {:?}", self.next_token);
        println!()
    }

    fn dump(v: &Value) {
        if !cfg!(debug_assertions) {
            return;
        }

        if let Value::Object(_) = v {
            println!("{:?}", v);
            println!("{}", v);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::json::Object;
    use crate::json::Value;
    use crate::parser::Error;
    use crate::parser::Parser;
    use crate::token::Token;
    use std::collections::HashMap;

    macro_rules! hash {
        ( $( $t:expr),* ) => {
            {
                let mut temp_hash = HashMap::new();
                $(
                    temp_hash.insert($t.0, $t.1);
                )*
                temp_hash
            }
        };
    }

    #[test]
    fn parse_object_composite() {
        let mut tokens = vec![
            Token::LeftBrace,
            Token::String("x".to_string()),
            Token::Colon,
            Token::LeftBrace,
            Token::String("y".to_string()),
            Token::Colon,
            Token::Number(1.0),
            Token::RightBrace,
            Token::RightBrace,
        ]
        .into_iter();

        let want = hash![(
            "x".to_string(),
            Value::Object(hash![("y".to_string(), Value::Number(1.0))])
        )];
        let got = Parser::new(&mut tokens).parse();

        assert_object(want, got)
    }
    #[test]
    fn parse_object_string() {
        let mut tokens = vec![
            Token::LeftBrace,
            Token::String("x".to_string()),
            Token::Colon,
            Token::String("y".to_string()),
            Token::RightBrace,
        ]
        .into_iter();

        let want = hash![("x".to_string(), Value::String("y".to_string()))];
        let got = Parser::new(&mut tokens).parse();

        assert_object(want, got)
    }
    #[test]
    fn parse_object_boolean() {
        let mut tokens = vec![
            Token::LeftBrace,
            Token::String("x".to_string()),
            Token::Colon,
            Token::True,
            Token::Comma,
            Token::String("y".to_string()),
            Token::Colon,
            Token::False,
            Token::RightBrace,
        ]
        .into_iter();

        let want = hash![
            ("x".to_string(), Value::Boolean(true)),
            ("y".to_string(), Value::Boolean(false))
        ];
        let got = Parser::new(&mut tokens).parse();

        assert_object(want, got)
    }
    #[test]
    fn parse_object_null() {
        let mut tokens = vec![
            Token::LeftBrace,
            Token::String("null".to_string()),
            Token::Colon,
            Token::Null,
            Token::RightBrace,
        ]
        .into_iter();

        let want = hash![("null".to_string(), Value::Null)];
        let got = Parser::new(&mut tokens).parse();

        assert_object(want, got)
    }
    #[test]
    fn parse_object_multiple() {
        let mut tokens = vec![
            Token::LeftBrace,
            Token::String("x".to_string()),
            Token::Colon,
            Token::String("y".to_string()),
            Token::Comma,
            Token::String("z".to_string()),
            Token::Colon,
            Token::String("w".to_string()),
            Token::RightBrace,
        ]
        .into_iter();

        let want = hash![
            ("x".to_string(), Value::String("y".to_string())),
            ("z".to_string(), Value::String("w".to_string()))
        ];
        let got = Parser::new(&mut tokens).parse();

        assert_object(want, got)
    }

    fn assert_object(want: Object, got: Result<Object, Error>) {
        match got {
            Ok(j) => assert_eq!(want, j),
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "Want Object, got Error {}", e)
            }
        }
    }
}
