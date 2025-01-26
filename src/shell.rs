use crate::command::Command;
use crate::parser::Parser;
use crate::redirect::Redirect;
use anyhow::Result;
use std::io;
use std::io::Write;

pub struct Shell;

impl Shell {
    pub fn new() -> Shell {
        Self {}
    }

    pub fn run(&mut self) -> Result<()> {
        let stdin = io::stdin();
        loop {
            // Reset stdout and stderr to console
            let mut stdout: Box<dyn Write> = Box::new(io::stdout());
            let mut stderr: Box<dyn Write> = Box::new(io::stderr());

            write!(stdout, "$ ")?;
            stdout.flush()?;

            let mut input = String::new();
            stdin.read_line(&mut input)?;
            let parsed_input = Parser::parse(&input.trim());
            let command_name = parsed_input[0].clone();
            let mut args = parsed_input[1..].to_vec();
            match handle_redir(&mut args, &mut stdout, &mut stderr) {
                Ok(()) => {}
                Err(err) => {
                    writeln!(stderr, "Failed to handle redirection: {}", err)?;
                    continue;
                }
            }

            let result = Command::new(command_name, args).exec()?;
            stdout.write(&result.stdout)?;
            stderr.write(&result.stderr)?;
            stdout.flush()?;
            stderr.flush()?;
        }
    }
}

fn handle_redir(
    args: &mut Vec<String>,
    stdout: &mut Box<dyn Write>,
    stderr: &mut Box<dyn Write>,
) -> Result<(), String> {
    for i in 0..args.len() {
        match Redirect::is_redirect(args[i].as_str()) {
            None => {}
            Some(redirect) => {
                let redir_file = redirect.get_file(args.get(i + 1))?;
                match redirect.is_stdout() {
                    true => *stdout = Box::new(redir_file),
                    false => *stderr = Box::new(redir_file),
                }

                // Remove redirection tokens and the file name
                args.drain(i..=i + 1);
                break;
            }
        }
    }
    Ok(())
}
