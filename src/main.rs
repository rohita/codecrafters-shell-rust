mod builtin;
mod command;
mod shell;
mod parser;
mod redirect;
mod trie;
mod completion;

use crate::shell::Shell;
use anyhow::Result;

fn main() -> Result<()> {
    let mut shell = Shell::new();
    shell.run()
}