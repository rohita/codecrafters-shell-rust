use crate::builtin::Builtin::*;
use anyhow::Result;
use pathsearch::find_executable_in_path;
use std::process::exit;

pub enum Builtin {
    Echo,
    Exit,
    Type,
}

impl Builtin {
    pub fn is_builtin(name: &str) -> Option<Builtin> {
        match name {
        "echo" => Some(Echo),
        "exit" => Some(Exit),
        "type" => Some(Type),
        _ => None,
        }
    }

    pub fn exec(&self, args: &Vec<String>) -> Result<Vec<u8>> {
        let mut return_val = match self {
            Echo => args.join(" "),
            Exit => exit(args[0].parse()?),
            Type => {
                let type_arg = &args[0];
                if Self::is_builtin(type_arg).is_some() {
                    format!("{type_arg} is a shell builtin")
                } else if let Some(executable) = find_executable_in_path(type_arg) {
                    format!("{} is {}", type_arg, executable.display())
                } else {
                    format!("{type_arg}: not found")
                }
            },
        };

        return_val.push_str("\n");
        Ok(return_val.into_bytes())
    }
}