use std::{env, fs};
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
        Self { search_idx: Self::build_search_index() }
    }

    pub fn capture_input(&self) -> Result<String> {
        let mut stdout = io::stdout().into_raw_mode().context("Failed to enter raw mode")?;
        let mut input = String::new();
        let mut buffer = [0; 1];
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut double_tab = false;

        loop {
            handle.read_exact(&mut buffer).context("Failed to read input")?;
            let c = buffer[0] as char;

            match c {
                TAB => {
                    let mut suggestions = self.search_idx.search(&input);
                    suggestions.sort();
                    match suggestions.len() {
                        0 => {
                            write!(stdout, "\x07")?;
                        }
                        1 => {
                            write!(stdout, "\r$ {} ", suggestions[0])?;
                            input = format!("{} ", suggestions[0]);
                        }
                        _ => {
                            match double_tab {
                                true => write!(stdout, "\r\n{}\r\n$ {}", suggestions.join("  "), input)?,
                                false => write!(stdout, "\x07")?
                            }
                            double_tab = !double_tab;
                        }
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

    fn build_search_index() -> Trie {
        let mut search_idx = Trie::new();
        for executable in Self::get_executables() {
            search_idx.insert(&executable);
        }
        for builtin in Builtin::variants_as_lowercase() {
            search_idx.insert(&builtin);
        }
        search_idx
    }

    pub fn get_executables() -> Vec<String> {
        let path = env::var("PATH").unwrap();
        let path_directories = env::split_paths(&path);

        path_directories
            .filter_map(|path| fs::read_dir(path).ok())
            .flat_map(|directory| directory.filter_map(|entry| entry.ok()))
            .filter(|entry| entry.metadata().map(|m| m.is_file()).unwrap_or(false))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect()
    }
}