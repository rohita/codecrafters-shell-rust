use pathsearch::find_executable_in_path;
use std::process::exit;
use crate::builtin::Builtin::*;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

pub enum Builtin {
    Echo,
    Exit,
    Type,
    NotFound(String),
}

impl Builtin {
    pub fn from(command: &str) -> Builtin {
        match command {
            "echo" => Echo,
            "exit" => Exit,
            "type" => Type,
            _ => NotFound(command.to_string()),
        }
    }

    pub fn handle(&self, args: Vec<&str>) {
        match self {
            Echo => println!("{}", args.join(" ")),
            Exit => exit(args[0].parse().unwrap()),
            NotFound(command) => println!("{}: command not found", command),
            Type => self.handle_type(args[0]),
        }
    }

    fn handle_type(&self, command: &str) {
        if BUILTINS.contains(&command) {
            println!("{command} is a shell builtin");
        } else if let Some(executable) = find_executable_in_path(command) {
            println!("{} is {}", command, executable.display());
        } else {
            println!("{command}: not found");
        }
    }
}