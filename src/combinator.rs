use std::collections::HashMap;

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FoundUnTerminatedError,
    ParseError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::FoundUnTerminatedError => write!(f, "Found unterminated json"),
            Error::ParseError => write!(f, "Failed to parse value"),
        }
    }
}

extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    error::ErrorKind,
    multi::separated_list,
    sequence::delimited,
    IResult,
};

use crate::json::{Root, Value};

pub fn parse(s: &str) -> Result<Root, Error> {
    if let Ok((s, j)) = parse_json(s) {
        if let Ok((s, _)) = multispace0::<&str, (&str, ErrorKind)>(s) {
            if s != "" {
                return Err(Error::FoundUnTerminatedError);
            }
        }
        Ok(j)
    } else {
        return Err(Error::ParseError);
    }
}
fn parse_json(s: &str) -> IResult<&str, Root> {
    let (s, v) = alt((parse_array, parse_object))(s)?;

    match v {
        Value::Object(o) => Ok((s, Root::Object(o))),
        Value::Array(a) => Ok((s, Root::Array(a))),
        _ => unreachable!(),
    }
}

fn parse_value(s: &str) -> IResult<&str, Value> {
    let (s, _) = multispace0(s)?;
    alt((
        parse_null,
        parse_boolean,
        parse_number,
        parse_string,
        parse_array,
        parse_object,
    ))(s)
}

fn parse_null(s: &str) -> IResult<&str, Value> {
    let (s, _) = tag("null")(s)?;
    Ok((s, Value::Null))
}
fn parse_boolean(s: &str) -> IResult<&str, Value> {
    match alt((tag("true"), tag("false")))(s)? {
        (s, "true") => Ok((s, Value::Boolean(true))),
        (s, _) => Ok((s, Value::Boolean(false))),
    }
}

fn parse_digits_with_sign(s: &str) -> IResult<&str, String> {
    let (s, suffix) = alt((tag("+"), tag("-"), tag("")))(s)?;

    let (s, v1) = digit1(s)?;
    Ok((s, format!("{}{}", suffix, v1)))
}

fn parse_number_with_sign(s: &str) -> IResult<&str, String> {
    let (s, v1) = parse_digits_with_sign(s)?;

    match tag::<&str, &str, (&str, ErrorKind)>(".")(s) {
        Ok((s, _)) => {
            let (s, v2) = digit1(s)?;
            Ok((s, format!("{}.{}", v1, v2)))
        }
        _ => Ok((s, v1.to_string())),
    }
}

fn parse_number(s: &str) -> IResult<&str, Value> {
    let (s, v1) = parse_number_with_sign(s)?;
    if let Ok((x, e)) = alt((
        tag::<&str, &str, (&str, ErrorKind)>("e"),
        tag::<&str, &str, (&str, ErrorKind)>("E"),
    ))(s)
    {
        let (x, v2) = parse_digits_with_sign(x)?;
        return Ok((
            x,
            Value::Number(format!("{}{}{}", v1, e, v2).parse().unwrap()),
        ));
    }
    Ok((s, Value::Number(v1.parse().unwrap())))
}

fn parse_string(s: &str) -> IResult<&str, Value> {
    let (s, _) = multispace0(s)?;

    match delimited(tag("\""), take_until("\""), tag("\""))(s)? {
        (s, v) => Ok((s, Value::String(v.to_string()))),
    }
}

fn parse_array(s: &str) -> IResult<&str, Value> {
    let (s, _) = multispace0(s)?;

    let (s, _) = tag("[")(s)?;

    let (s, _) = multispace0(s)?;

    let (s, v) = match separated_list(tag(","), parse_value)(s) {
        Ok((s, x)) => (s, Some(x)),
        _ => (s, None),
    };

    let (s, _) = multispace0(s)?;

    let (s, _) = tag("]")(s)?;

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

    use crate::combinator::{parse, parse_value, Error};
    use crate::json::Root;
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
        assert_eq!(parse_value("123.4567"), Ok(("", Number(123.4567))));
        assert_eq!(parse_value("-123.4567"), Ok(("", Number(-123.4567))));
        assert_eq!(parse_value("10e0"), Ok(("", Number(10.0))));
        assert_eq!(parse_value("10e-10"), Ok(("", Number(0.000000001))));
        assert_eq!(parse_value("-1.2e-10"), Ok(("", Number(-0.00000000012))));
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
            parse_value("[\"string\"]"),
            Ok(("", Array(vec![String("string".to_string())])))
        );
        assert_eq!(parse_value("[true]"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(parse_value("[1]"), Ok(("", Array(vec![Number(1.0)]))));
        assert_eq!(parse_value("[]"), Ok(("", Array(vec![]))));
        assert_eq!(parse_value("[[]]"), Ok(("", Array(vec![Array(vec![])]))));
        assert_eq!(
            parse_value("[true,false]"),
            Ok(("", Array(vec![Boolean(true), Boolean(false)])))
        );
        assert_eq!(parse_value(" [true]"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(parse_value("[ true]"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(parse_value("[true ]"), Ok(("", Array(vec![Boolean(true)]))));
        assert_eq!(
            parse_value("[true] "),
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
    #[test]
    fn root() {
        assert_eq!(
            parse("[true,false]"),
            Ok(Root::Array(vec![Boolean(true), Boolean(false)]))
        );
        assert_eq!(
            parse("{\"a\":{\"b\":\"c\"}}"),
            Ok(Root::Object(hash![(
                "a".to_string(),
                Object(hash![("b".to_string(), String("c".to_string()))])
            )]))
        );
        assert_eq!(parse("[true,false]1"), Err(Error::FoundUnTerminatedError));
        assert_eq!(parse("[true,falsex"), Err(Error::ParseError));
    }

}
