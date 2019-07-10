use std::collections::HashMap;

extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    multi::{many1, separated_list},
    sequence::delimited,
    IResult,
};

use crate::json::Value;

pub fn parse_value(s: &str) -> IResult<&str, Value> {
    alt((
        parse_boolean,
        parse_number,
        parse_string,
        parse_array,
        parse_object,
    ))(s)
}

fn parse_boolean(s: &str) -> IResult<&str, Value> {
    match alt((tag("true"), tag("false")))(s)? {
        (s, "true") => Ok((s, Value::Boolean(true))),
        (s, _) => Ok((s, Value::Boolean(false))),
    }
}

fn parse_number(s: &str) -> IResult<&str, Value> {
    match many1(digit1)(s)? {
        (s, v) => Ok((
            s,
            Value::Number(v.iter().cloned().collect::<String>().parse().unwrap()),
        )),
    }
}

fn parse_string(s: &str) -> IResult<&str, Value> {
    match delimited(tag("\""), take_until("\""), tag("\""))(s)? {
        (s, v) => Ok((s, Value::String(v.to_string()))),
    }
}

fn parse_array(s: &str) -> IResult<&str, Value> {
    let (s, _) = multispace0(s)?;

    let (s, _) = tag("(")(s)?;

    let (s, _) = multispace0(s)?;

    let (s, v) = match separated_list(tag(","), parse_value)(s) {
        Ok((s, x)) => (s, Some(x)),
        _ => (s, None),
    };

    let (s, _) = multispace0(s)?;

    let (s, _) = tag(")")(s)?;

    match v {
        Some(v) => Ok((s, Value::Array(v))),
        None => Ok((s, Value::Array(vec![]))),
    }
}

fn parse_kvp(s: &str) -> IResult<&str, (String, Value)> {
    let (s, _) = multispace0(s)?;

    let (s, k) = parse_string(s)?;

    let (s, _) = multispace0(s)?;
    let (s, _) = tag(":")(s)?;

    let (s, _) = multispace0(s)?;
    let (s, v) = parse_value(s)?;

    match k {
        Value::String(k) => Ok((s, (k, v))),
        _ => unreachable!(),
    }
}

fn parse_object(s: &str) -> IResult<&str, Value> {
    let (s, _) = multispace0(s)?;

    let (s, _) = tag("{")(s)?;

    let (s, _) = multispace0(s)?;

    let (s, kvs) = match separated_list(tag(","), parse_kvp)(s) {
        Ok((s, x)) => (s, Some(x)),
        _ => (s, None),
    };

    let (s, _) = multispace0(s)?;

    let (s, _) = tag("}")(s)?;

    let mut map = HashMap::new();
    if let Some(x) = kvs {
        for (k, v) in x {
            map.insert(k, v);
        }
    }
    Ok((s, Value::Object(map)))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::combinator::parse_value;
    use crate::json::Value::{Array, Boolean, Number, Object, String};
    use nom::{error::ErrorKind, Err};

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
    fn boolean() {
        assert_eq!(parse_value("true"), Ok(("", Boolean(true))));
        assert_eq!(parse_value("false-m"), Ok(("-m", Boolean(false))));
        assert_eq!(parse_value("xxx"), Err(Err::Error(("xxx", ErrorKind::Tag))));
    }
    #[test]
    fn number() {
        assert_eq!(parse_value("1234567"), Ok(("", Number(1234567.0))));
        assert_eq!(parse_value("123-4567"), Ok(("-4567", Number(123.0))));
        assert_eq!(parse_value("x"), Err(Err::Error(("x", ErrorKind::Tag))));
    }

    #[test]
    fn string() {
        assert_eq!(parse_value("\"x\""), Ok(("", String("x".to_string()))));
        assert_eq!(
            parse_value("\"true\""),
            Ok(("", String("true".to_string())))
        );
        assert_eq!(parse_value("\"\""), Ok(("", String("".to_string()))));
        assert_eq!(parse_value("\"x"), Err(Err::Error(("\"x", ErrorKind::Tag))));
        assert_eq!(parse_value("x"), Err(Err::Error(("x", ErrorKind::Tag))));
    }
    #[test]
    fn array() {
        assert_eq!(
            parse_value("(\"string\")"),
            Ok(("", Array(vec![String("string".to_string())])))
        );
        assert_eq!(parse_value("(true)"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(parse_value("(1)"), Ok(("", Array(vec![Number(1.0)]))));
        assert_eq!(parse_value("()"), Ok(("", Array(vec![]))));
        assert_eq!(parse_value("(())"), Ok(("", Array(vec![Array(vec![])]))));
        assert_eq!(
            parse_value("(true,false)"),
            Ok(("", Array(vec![Boolean(true), Boolean(false)])))
        );
        assert_eq!(parse_value(" (true)"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(parse_value("( true)"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(parse_value("(true )"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(
            parse_value("(true) "),
            Ok((" ", Array(vec![Boolean(true)])))
        );
    }
    #[test]
    fn object() {
        assert_eq!(
            parse_value("{\"x\":\"y\"}"),
            Ok((
                "",
                Object(hash![("x".to_string(), String("y".to_string()))])
            ))
        );
        assert_eq!(
            parse_value("{\"x\":\"y\",\"z\":\"w\"}"),
            Ok((
                "",
                Object(hash![
                    ("x".to_string(), String("y".to_string())),
                    ("z".to_string(), String("w".to_string()))
                ])
            ))
        );
        assert_eq!(
            parse_value("{\"a\":{\"b\":\"c\"}}"),
            Ok((
                "",
                Object(hash![(
                    "a".to_string(),
                    Object(hash![("b".to_string(), String("c".to_string()))])
                )])
            ))
        );
    }
}
