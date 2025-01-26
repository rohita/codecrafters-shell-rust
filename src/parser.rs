use std::mem::replace;

pub struct Parser;

enum State {
    Backslash,
    Delimiter,
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
    DoubleQuotedBackslash,
}

impl Parser {
    pub fn parse(source: &str) -> Vec<String> {
        use State::*;
        let mut words = Vec::new();
        let mut word = String::new();
        let mut chars = source.chars();
        let mut state = Delimiter;
        loop {
            let next = chars.next();
            state = match state {
                Backslash => match next {
                    None => {
                        word.push('\\');
                        words.push(replace(&mut word, String::new()));
                        break;
                    }
                    Some(c) => {
                        word.push(c);
                        Unquoted
                    }
                },
                Delimiter => match next {
                    None => break,
                    Some('\'') => SingleQuoted,
                    Some('\"') => DoubleQuoted,
                    Some('\\') => Backslash,
                    Some(' ') => Delimiter,
                    Some(c) => {
                        word.push(c);
                        Unquoted
                    }
                },
                SingleQuoted => match next {
                    None => panic!("parse error"),
                    Some('\'') => Unquoted,
                    Some(c) => {
                        word.push(c);
                        SingleQuoted
                    }
                },
                DoubleQuoted => match next {
                    None => panic!("parse error"),
                    Some('\"') => Unquoted,
                    Some('\\') => DoubleQuotedBackslash,
                    Some(c) => {
                        word.push(c);
                        DoubleQuoted
                    }
                },
                DoubleQuotedBackslash => match next {
                    None => panic!("parse error"),
                    Some(c @ '$') | Some(c @ '\n') | Some(c @ '"') | Some(c @ '\\') => {
                        word.push(c);
                        DoubleQuoted
                    }
                    Some(c) => {
                        word.push('\\');
                        word.push(c);
                        DoubleQuoted
                    }
                },
                Unquoted => match next {
                    None => {
                        words.push(replace(&mut word, String::new()));
                        break;
                    }
                    Some('\'') => SingleQuoted,
                    Some('\"') => DoubleQuoted,
                    Some('\\') => Backslash,
                    Some(' ') => {
                        words.push(replace(&mut word, String::new()));
                        Delimiter
                    }
                    Some(c) => {
                        word.push(c);
                        Unquoted
                    }
                },
            }
        }
        words
    }
}