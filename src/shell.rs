use crate::command::Command;
use crate::completion::Completer;
use crate::parser::parse;
use crate::redirect::Redirect;
use anyhow::Result;
use std::io;
use std::io::Write;

pub struct Shell {
    completer: Completer,
}

impl Shell {
    pub fn new() -> Shell {
        Self {
            completer: Completer::new()
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            // Reset stdout and stderr to console
            let mut stdout: Box<dyn Write> = Box::new(io::stdout());
            let mut stderr: Box<dyn Write> = Box::new(io::stderr());

            write!(stdout, "$ ")?;
            stdout.flush()?;

            let input = self.completer.capture_input()?;
            let parsed_input = parse(input.trim());
            let command_name = parsed_input[0].clone();
            let mut args = parsed_input[1..].to_vec();
            if let Some(redirect) =  Redirect::is_redirect(&mut args) {
                match redirect.is_stdout {
                    true => stdout = Box::new(redirect.file),
                    false => stderr = Box::new(redirect.file),
                }
            }

            let result = Command::new(command_name, args).exec()?;
            stdout.write_all(&result.stdout)?;
            stderr.write_all(&result.stderr)?;
            stdout.flush()?;
            stderr.flush()?;
        }
    }
}
