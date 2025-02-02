use std::mem::take;

pub struct Parser;

const SPACE: char = ' ';
const BACKSLASH: char = '\\';
const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';
const ESCAPABLE_CHARS: [char; 4] = [BACKSLASH, '$', '\n', DOUBLE_QUOTE];

impl Parser {
    pub fn parse(source: &str) -> Vec<String> {
        let mut arguments = Vec::new();
        let mut current_argument = String::new();
        let mut chars = source.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                SPACE => if !current_argument.is_empty() {
                    arguments.push(take(&mut current_argument));
                },
                BACKSLASH => match chars.next() {
                    Some(next) => current_argument.push(next),
                    None => {
                        current_argument.push(BACKSLASH);
                        break;
                    }
                },
                SINGLE_QUOTE => {
                    loop {
                        match chars.next() {
                            None => panic!("Unmatched single quote in input"),
                            Some(SINGLE_QUOTE) => break,
                            Some(quoted) => current_argument.push(quoted),
                        }
                    }
                },
                DOUBLE_QUOTE => {
                    loop {
                        match chars.next() {
                            None => panic!("Unmatched double quote in input"),
                            Some(DOUBLE_QUOTE) => break,
                            Some(BACKSLASH) => match chars.peek() {
                                Some(next) if ESCAPABLE_CHARS.contains(next) => {
                                    current_argument.push(chars.next().unwrap())
                                }
                                _ => current_argument.push(BACKSLASH),
                            },
                            Some(quoted) => current_argument.push(quoted),
                        }
                    }
                }
                _ => current_argument.push(c),
            }
        }

        if !current_argument.is_empty() {
            arguments.push(current_argument);
        }
        arguments
    }
}
