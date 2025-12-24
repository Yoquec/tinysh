#[allow(unused_imports)]
use std::io::{self, Write};

const BUILTINS: [&str; 3] = ["echo", "type", "exit"];

fn main() {
    'mainloop: loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        match &buf.trim().split_whitespace().collect::<Vec<_>>()[..] {
            [command, arguments @ ..] => match *command {
                "echo" => echo(arguments),
                "type" => type_(arguments),
                "exit" => break 'mainloop,
                _ => print_not_found(command),
            },
            _ => continue,
        }
    }
}

fn print_not_found(command: &str) {
    println!("{}: command not found", command.trim())
}

fn type_(arguments: &[&str]) {
    for arg in arguments {
        if BUILTINS.contains(arg) {
            println!("{} is a shell builtin", *arg)
        } else {
            println!("{}: not found", *arg)
        }
    }
}

fn echo(arguments: &[&str]) {
    println!("{}", arguments.join(" "))
}
