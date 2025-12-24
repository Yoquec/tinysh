#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    fs::{self, DirEntry},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

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

fn type_(commands: &[&str]) {
    for cmd in commands {
        if BUILTINS.contains(cmd) {
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
