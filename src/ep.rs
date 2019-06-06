use crate::lexer::Lexer;
use crate::parser::Parser;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(s: String) {
    let mut tokens = Lexer::new(&s).into_iter();
    match Parser::new(&mut tokens).parse() {
        Ok(j) => {
            println!("{:?}", j);
        }
        Err(e) => println!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::json::Object;
    use crate::json::Value;
    use crate::lexer::Lexer;
    use crate::parser::Error;
    use crate::parser::Parser;
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
    fn parse_full() {
        let input = r##"
        {
            "info": {
                "version":"0.1.0"
            },
            "はろー" : null,
            "dev": true,
            "escape_support" : false,
            "keywords": ["json","parser","rust"]
        }
        "##
        .to_string();

        let want = hash![
            (
                "info".to_string(),
                Value::Object(hash![(
                    "version".to_string(),
                    Value::String("0.1.0".to_string())
                )])
            ),
            ("はろー".to_string(), Value::Null),
            ("dev".to_string(), Value::Boolean(true)),
            ("escape_support".to_string(), Value::Boolean(false)),
            (
                "keywords".to_string(),
                Value::Array(
                    [
                        Value::String("json".to_string()),
                        Value::String("parser".to_string()),
                        Value::String("rust".to_string())
                    ]
                    .to_vec()
                )
            )
        ];

        let mut tokens = Lexer::new(&input).into_iter();
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
