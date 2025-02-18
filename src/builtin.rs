use crate::builtin::Builtin::*;
use anyhow::Result;
use pathsearch::find_executable_in_path;
use std::env;
use std::path::Path;
use std::process::exit;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Builtin {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd,
}

impl Builtin {
    pub(crate) fn variants_as_lowercase() -> Vec<String> {
        Builtin::iter().map(|variant| format!("{:?}", variant).to_lowercase()).collect()
    }

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

    pub fn exec(&self, args: &[String]) -> Result<Vec<u8>> {
        let mut return_val = match self {
            Echo => args.join(" "),
            Exit => {
                match args.is_empty() {
                    false => exit(args[0].parse().unwrap_or(0)),
                    true => exit(0)
                }
            },
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
            return_val.push('\n');
        }

        Ok(return_val.into_bytes())
    }
}
