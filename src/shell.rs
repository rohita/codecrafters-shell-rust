use crate::command::Command;
use anyhow::Result;
use std::io;
use std::io::Write;

pub struct Shell {
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl Shell {
    pub fn new() -> Shell {
        Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            write!(self.stdout, "$ ")?;
            self.stdout.flush()?;

            let mut input = String::new();
            self.stdin.read_line(&mut input)?;

            let result = Command::new(input).exec()?;
            self.stdout.write(&result)?;
            self.stdout.flush()?;
        }
    }
}
