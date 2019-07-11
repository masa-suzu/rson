use crate::combinator::parse;
use crate::json::Root;

pub fn run(s: String) {
    match parse(&s) {
        Ok(Root::Object(o)) => {
            println!("{:?}", o);
        }
        Ok(Root::Array(a)) => {
            println!("{:?}", a);
        }
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::combinator::{parse, Error};
    use crate::json::Object;
    use crate::json::Root;
    use crate::json::Value;

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

        let got = parse(&input);

        assert_object(want, got)
    }

    fn assert_object(want: Object, got: Result<Root, Error>) {
        match got {
            Ok(Root::Object(o)) => assert_eq!(want, o),
            Ok(Root::Array(a)) => unreachable!(),
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "Want Object, got Error {:?}", e)
            }
        }
    }
}
