use std::env;
use std::path::Path;
use crate::builtin::Builtin::*;
use anyhow::Result;
use pathsearch::find_executable_in_path;
use std::process::exit;

pub enum Builtin {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd,
}

impl Builtin {
    pub fn is_builtin(name: &str) -> Option<Builtin> {
        match name {
            "echo" => Some(Echo),
            "exit" => Some(Exit),
            "type" => Some(Type),
            "pwd" => Some(Pwd),
            "cd" => Some(Cd),
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
            Pwd => env::current_dir()?.display().to_string(),
            Cd => {
                let dir = args[0].replace("~", env::var("HOME")?.as_str());
                let path = Path::new(&dir);
                match env::set_current_dir(path) {
                    Ok(()) => String::new(),
                    Err(_) => format!("cd: {}: No such file or directory", args[0])
                }
            },
        };

        if !return_val.is_empty() {
            return_val.push_str("\n");
        }

        Ok(return_val.into_bytes())
    }
}
