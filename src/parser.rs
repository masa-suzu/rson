use crate::json::Value;
use crate::token::Token;

use crate::json::Root;
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
    FoundUnTerminatedBracket,
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
            Error::FoundUnTerminatedBracket => write!(f, "Not terminated bracket."),
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

    pub fn parse(&mut self) -> Result<Root, Error> {
        self.debug();
        match self.current_token.to_owned() {
            Token::LeftBrace => match self.parse_object() {
                Ok(v) => match v {
                    Value::Object(o) => Ok(Root::Object(o)),
                    _ => panic!("parse_object must return Ok(Value::Object) or Err(Error)"),
                },
                Err(e) => Err(e),
            },
            Token::LeftBracket => match self.parse_array() {
                Ok(v) => match v {
                    Value::Array(o) => Ok(Root::Array(o)),
                    _ => panic!("parse_array must return Ok(Value::Array) or Err(Error)"),
                },
                Err(e) => Err(e),
            },
            t => Err(Error::FoundUnExpectedToken(Token::LeftBrace, t)),
        }
    }

    fn parse_object(&mut self) -> Result<Value, Error> {
        let mut kvs: HashMap<String, Value> = HashMap::new();
        if self.next_token == Token::RightBrace {
            return Ok(Value::Object(kvs));
        }
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

    fn parse_array(&mut self) -> Result<Value, Error> {
        let mut array: Vec<Value> = Vec::new();
        if self.next_token == Token::RightBracket {
            return Ok(Value::Array(array));
        }
        loop {
            match self.parse_value() {
                Ok(v) => {
                    array.push(v);
                }
                Err(e) => return Err(e),
            }

            match self.next_token {
                Token::RightBracket => {
                    return Ok(Value::Array(array));
                }
                Token::Eof => return Err(Error::FoundUnTerminatedBracket),
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
                    Ok(Root::Object(o)) => Value::Object(o),
                    Ok(Root::Array(o)) => Value::Array(o),
                    Err(e) => return Err(e),
                }
            }
        };

        self.advance_token();

        Ok(v)
    }

    fn advance_token(&mut self) {
        self.debug();

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
        println!("current token : {:?}", self.current_token);
        println!("next token    : {:?}", self.next_token);
        println!()
    }
}

#[cfg(test)]
mod tests {
    use crate::json::Root;
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

        assert_root(Root::Object(want), got)
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

        assert_root(Root::Object(want), got)
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

        assert_root(Root::Object(want), got)
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

        assert_root(Root::Object(want), got)
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

        assert_root(Root::Object(want), got)
    }

    #[test]
    fn parse_object_array() {
        let mut tokens = vec![
            Token::LeftBrace,
            Token::String("x".to_string()),
            Token::Colon,
            Token::LeftBracket,
            Token::String("y".to_string()),
            Token::Comma,
            Token::True,
            Token::Comma,
            Token::Null,
            Token::RightBracket,
            Token::RightBrace,
        ]
        .into_iter();

        let want = hash![(
            "x".to_string(),
            Value::Array(vec!(
                Value::String("y".to_string()),
                Value::Boolean(true),
                Value::Null
            ))
        )];
        let got = Parser::new(&mut tokens).parse();

        assert_root(Root::Object(want), got)
    }

    #[test]
    fn parse_root_array() {
        let mut tokens = vec![
            Token::LeftBracket,
            Token::String("x".to_string()),
            Token::Comma,
            Token::True,
            Token::RightBracket,
        ]
        .into_iter();

        let want = vec![Value::String("x".to_string()), Value::Boolean(true)];
        let got = Parser::new(&mut tokens).parse();

        assert_root(Root::Array(want), got)
    }

    #[test]
    fn parse_empty_object() {
        let mut tokens = vec![Token::LeftBrace, Token::RightBrace].into_iter();

        let want = hash![];
        let got = Parser::new(&mut tokens).parse();

        assert_root(Root::Object(want), got)
    }

    #[test]
    fn parse_empty_array() {
        let mut tokens = vec![Token::LeftBracket, Token::RightBracket].into_iter();

        let want = vec![];
        let got = Parser::new(&mut tokens).parse();

        assert_root(Root::Array(want), got)
    }

    #[test]
    fn parse_empty_values() {
        let mut tokens = vec![
            Token::LeftBracket,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::LeftBracket,
            Token::RightBracket,
            Token::RightBracket,
        ]
        .into_iter();

        let want = vec![Value::Object(hash!()), Value::Array(vec![])];
        let got = Parser::new(&mut tokens).parse();

        assert_root(Root::Array(want), got)
    }

    fn assert_root(want: Root, got: Result<Root, Error>) {
        match got {
            Ok(x) => assert_eq!(want, x),
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "Want Object, got Error {}", e)
            }
        }
    }
}
