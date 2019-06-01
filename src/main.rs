pub mod json;
pub mod lexer;
pub mod parser;
pub mod token;

fn try_read_from_stdin<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.parse()
}

fn main() {
    let input = match try_read_from_stdin::<String>() {
        Ok(x) => x,
        _ => std::process::exit(1),
    };

    let mut tokens = lexer::Lexer::new(&input).into_iter();
    match parser::Parser::new(&mut tokens).parse() {
        Ok(j) => {
            println!("{:?}", j);
        }
        Err(e) => println!("{}", e),
    }
}
