use std::process::exit;
use crate::builtin::Builtin::*;

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
            Type => match args[0] {
                "exit" | "echo" | "type" => println!("{} is a shell builtin", args[0]),
                _ => println!("{}: not found", args[0]),
            },
        }
    }
}