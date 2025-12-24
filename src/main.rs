#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        match buf.trim_end() {
            "exit" => break,
            _ => println!("{}: command not found", buf.trim_end()),
        }
    }
}
