#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{self, current_dir, set_current_dir},
    fs::{self, DirEntry},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};

enum Builtin {
    Pwd,
    Echo,
    Cd,
    Type,
    Exit,
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let splits = buf.trim().split_whitespace().collect::<Vec<_>>();

        match &splits[..] {
            [command, arguments @ ..] => match parse(command) {
                Some(Builtin::Echo) => echo(arguments),
                Some(Builtin::Type) => type_(arguments),
                Some(Builtin::Cd) => cd(arguments),
                Some(Builtin::Pwd) => pwd(),
                Some(Builtin::Exit) => return,
                _ => match find_path(command) {
                    Some(_) => run(*command, arguments),
                    None => print_not_found(command),
                },
            },
            // No output introduced or just empty lines, break to a new input line
            _ => continue,
        }
    }
}

/// Determines if the current command is a builtin
fn parse(command: &str) -> Option<Builtin> {
    match command {
        "echo" => Some(Builtin::Echo),
        "type" => Some(Builtin::Type),
        "cd" => Some(Builtin::Cd),
        "pwd" => Some(Builtin::Pwd),
        "exit" => Some(Builtin::Exit),
        _ => None,
    }
}

fn print_not_found(command: &str) {
    println!("{}: command not found", command.trim())
}

fn type_(commands: &[&str]) {
    for cmd in commands {
        if let Some(_) = parse(cmd) {
            println!("{} is a shell builtin", *cmd);
            continue;
        }

        match find_path(*cmd) {
            Some(entry) => println!("{} is {}", *cmd, entry.to_string_lossy()),
            _ => println!("{}: not found", *cmd),
        }
    }
}

fn echo(arguments: &[&str]) {
    println!("{}", arguments.join(" "))
}

fn pwd() {
    let wd = current_dir().unwrap();
    println!("{}", wd.as_path().to_string_lossy())
}

fn cd(arguments: &[&str]) {
    if arguments.len() > 1 {
        println!("cd: too many arguments");
        return;
    }

    let mut target = arguments.get(0).unwrap().to_string();

    if target.starts_with("~") {
        let home = env::var_os("HOME").unwrap();
        target = target.replace("~", home.to_str().unwrap());
    }

    if let Err(_) = set_current_dir(&target) {
        println!("cd: {target}: No such file or directory");
    }
}

fn run(command: &str, arguments: &[&str]) {
    let mut command = Command::new(command);
    let output = command.args(arguments).output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

/// Finds an executable binary in the path
fn find_path(command: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    let mut entries = env::split_paths(&path)
        .filter_map(|path| fs::read_dir(path.to_str()?).ok())
        .flat_map(|read_dir| read_dir.into_iter())
        .filter_map(|e| e.ok());

    return entries
        .find(|entry| entry.file_name().eq(command) && is_executable(entry))
        .map(|e| e.path());
}

/// An entry being executable in a UNIX operating system will be defined
/// as being a file or symlink having any of its executable bits set.
fn is_executable(entry: &DirEntry) -> bool {
    entry.metadata().map_or_else(
        |_| false,
        |metadata| {
            let mode = metadata.permissions().mode();
            (metadata.is_file() || metadata.is_symlink()) && (mode & 0o111) != 0
        },
    )
}
