use std::fs::{File, OpenOptions};
use std::path::Path;

enum RedirectType {
    Stdout, Stderr, StdoutAppend, StderrAppend,
}

impl RedirectType {
    pub fn from_str(token: &str) -> Option<RedirectType> {
        use RedirectType::*;
        match token {
            ">" | "1>" => Some(Stdout),
            ">>" | "1>>" => Some(StdoutAppend),
            "2>" => Some(Stderr),
            "2>>" => Some(StderrAppend),
            _ => None,
        }
    }

    pub fn is_append(&self) -> bool {
        match self {
            RedirectType::Stdout | RedirectType::Stderr => false,
            RedirectType::StdoutAppend | RedirectType::StderrAppend => true
        }
    }

    pub fn is_stdout(&self) -> bool {
        match self {
            RedirectType::Stdout | RedirectType::StdoutAppend => true,
            RedirectType::Stderr | RedirectType::StderrAppend => false
        }
    }
}

pub struct Redirect {
    pub is_stdout: bool,
    pub file: File,
}

impl Redirect {
    pub fn is_redirect(args: &mut Vec<String>) -> Option<Redirect> {
        for i in 0..args.len() {
            if let Some(redirect_type) = RedirectType::from_str(&args[i]) {
                if let Ok(file) = get_file(args.get(i + 1), redirect_type.is_append()) {
                    args.drain(i..=i + 1); // Remove redirection tokens and the file name
                    return Some(Redirect { is_stdout: redirect_type.is_stdout(), file })
                }
            }
        }

        None
    }
}

fn get_file(file: Option<&String>, is_append: bool) -> Result<File, String> {
    // Ensure the redirection file is specified
    let redir_path = match file {
        Some(f) => Path::new(f),
        None => return Err("Missing redirection file".into()),
    };

    // Ensure the parent directory exists
    if redir_path.parent().is_none() {
        return Err("Redirection file parent directory does not exist".into());
    }

    OpenOptions::new()
        .write(true)
        .create(true)
        .append(is_append)
        .open(redir_path)
        .map_err(|e| format!("Failed to create redirection file: {e}"))
}