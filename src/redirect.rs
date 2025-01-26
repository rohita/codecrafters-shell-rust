use std::fs::File;
use std::path::Path;

pub enum Redirect {
    Stdout, Stderr, StdoutAppend, StderrAppend,
}

impl Redirect {
    pub fn is_redirect(token: &str) -> Option<Redirect> {
        match token {
            ">" | "1>" => Some(Redirect::Stdout),
            ">>" | "1>>" => Some(Redirect::StdoutAppend),
            "2>" => Some(Redirect::Stderr),
            "2>>" => Some(Redirect::StderrAppend),
            _ => None,
        }
    }

    pub fn get_file(&self, file: Option<&String>) -> Result<File, String> {
        // Ensure the redirection file is specified
        let redir_path = match file {
            Some(f) => Path::new(f),
            None => return Err("Missing redirection file".to_string()),
        };

        // Ensure the parent directory exists
        if redir_path.parent().is_none() {
            return Err("Redirection file parent directory does not exist".to_string());
        }

        match self.is_append() {
            true => File::options().create(true).append(true).open(redir_path),
            false => File::create(redir_path)
        }.map_err(|_| "Failed to create redirection file".to_string())
    }

    pub fn is_append(&self) -> bool {
        match self {
            Redirect::Stdout | Redirect::Stderr => false,
            Redirect::StdoutAppend | Redirect::StderrAppend => true
        }
    }

    pub fn is_stdout(&self) -> bool {
        match self {
            Redirect::Stdout | Redirect::StdoutAppend => true,
            Redirect::Stderr | Redirect::StderrAppend => false
        }
    }

}