const SPACE: char = ' ';
const BACKSLASH: char = '\\';
const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';
const ESCAPABLE_CHARS: [char; 4] = [BACKSLASH, '$', '\n', DOUBLE_QUOTE];

/// Parses the command line input, taking into account the backslash
/// and escaped characters inside double quotes.
pub fn parse(source: &str) -> Vec<String> {
    let mut arguments = Vec::new();
    let mut current_argument = String::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            SPACE => match current_argument.is_empty() {
                false => arguments.push(std::mem::take(&mut current_argument)),
                true => continue,
            },
            BACKSLASH => match chars.next() {
                Some(next) => current_argument.push(next),
                None => current_argument.push(BACKSLASH),
            },
            SINGLE_QUOTE => loop {
                match chars.next() {
                    None => panic!("Unmatched single quote in input"),
                    Some(SINGLE_QUOTE) => break,
                    Some(quoted) => current_argument.push(quoted),
                }
            },
            DOUBLE_QUOTE => loop {
                match chars.next() {
                    None => panic!("Unmatched double quote in input"),
                    Some(DOUBLE_QUOTE) => break,
                    Some(BACKSLASH) => match chars.peek() {
                        Some(n) if ESCAPABLE_CHARS.contains(n) => {
                            current_argument.push(chars.next().unwrap())
                        }
                        _ => current_argument.push(BACKSLASH),
                    },
                    Some(quoted) => current_argument.push(quoted),
                }
            },
            _ => current_argument.push(c),
        }
    }

    if !current_argument.is_empty() {
        arguments.push(current_argument);
    }
    arguments
}

