mod builtin;
mod command;
mod shell;
use crate::shell::Shell;
use anyhow::Result;

fn main() -> Result<()> {
    let mut shell = Shell::new();
    shell.run()
}