extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    multi::{many1, separated_list},
    sequence::delimited,
    IResult,
};

use crate::json::Value;

pub fn parse_value(s: &str) -> IResult<&str, Value> {
    alt((parse_boolean, parse_number, parse_string, parse_array))(s)
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
    let (s, _) = tag("(")(s)?;

    let (s, v) = match separated_list(tag(","), parse_value)(s) {
        Ok((s, x)) => (s, Some(x)),
        _ => (s, None),
    };
    let (s, _) = tag(")")(s)?;

    match v {
        Some(v) => Ok((s, Value::Array(v))),
        None => Ok((s, Value::Array(vec![]))),
    }
}

#[cfg(test)]
mod tests {
    use crate::combinator::parse_value;
    use crate::json::Value::{Array, Boolean, Number, String};
    use nom::{error::ErrorKind, Err};
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
    }
}
