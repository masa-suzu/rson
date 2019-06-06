extern crate rson;

fn main() {
    let input = match try_read_from_stdin::<String>() {
        Ok(x) => x,
        _ => std::process::exit(1),
    };
    rson::ep::run(input);
}

fn try_read_from_stdin<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.parse()
}
