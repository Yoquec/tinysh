#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        rep();
    }
}

fn rep() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    println!("{}: command not found", buf.trim_end());
}
