use crate::builtin::Builtin;
use crate::trie::Trie;
use anyhow::{Result, Context};
use std::io::{self, Read, Write};
use termion::raw::IntoRawMode;

const TAB: char = '\t';
const NEWLINE: char = '\n';

pub struct Completer {
    search_idx: Trie
}

impl Completer {
    pub fn new() -> Self {
        let mut search_idx = Trie::new();
        Builtin::variants_as_lowercase()
            .iter()
            .for_each(|word| search_idx.insert(word));
        Self { search_idx }
    }

    pub fn capture_input(&self) -> Result<String> {
        let mut stdout = io::stdout().into_raw_mode().context("Failed to enter raw mode")?;
        let mut input = String::new();
        let mut buffer = [0; 1];
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        loop {
            handle.read_exact(&mut buffer).context("Failed to read input")?;
            let c = buffer[0] as char;

            match c {
                TAB => {
                    if input.is_empty() {
                        continue;
                    }
                    let suggestions = self.search_idx.search(&input);
                    if !suggestions.is_empty() {
                        write!(stdout, "\r$ {} ", suggestions[0])?;
                        input = format!("{} ", suggestions[0]);
                    }
                }
                NEWLINE => {
                    write!(stdout, "\r\n")?;
                    break;
                }
                _ => {
                    write!(stdout, "{}", c)?;
                    input.push(c);
                }
            }
            stdout.flush()?;
        }

        Ok(input)
    }
}