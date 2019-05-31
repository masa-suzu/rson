pub mod lexer;
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

    let mut lexer = lexer::Lexer::new(&input);

    loop {
        match &lexer.next_token() {
            token::Token::Eof => {
                println!("{:?}", token::Token::Eof);
                break;
            }
            x => println!("{:?}", x),
        }
    }
}