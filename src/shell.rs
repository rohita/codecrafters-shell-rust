use crate::command::Command;
use crate::parser::Parser;
use anyhow::Result;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

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
        if args[i] == ">" || args[i] == "1>" || args[i] == "2>" {
            // Ensure the redirection file is specified
            if i + 1 >= args.len() {
                return Err("Missing redirection file".to_string());
            }

            let redir_path = Path::new(&args[i + 1]);
            // Ensure the parent directory exists
            if let Some(parent) = redir_path.parent() {
                if !parent.exists() {
                    return Err("Redirection file parent directory does not exist".to_string());
                }
            }
            let redir_file =
                File::create(redir_path).map_err(|_| "Failed to create redirection file")?;
            if args[i] == ">" || args[i] == "1>" {
                // Redirect stdout
                *stdout = Box::new(redir_file);
            } else if args[i] == "2>" {
                // Redirect stderr
                *stderr = Box::new(redir_file);
            }
            // Remove redirection tokens and the file name
            args.drain(i..=i + 1);
            break;
        }
    }
    Ok(())
}
