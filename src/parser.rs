pub struct Parser;

enum State {
    Delimiter,
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
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
                Delimiter => match next {
                    None => break,
                    Some('\'') => SingleQuoted,
                    Some('\"') => DoubleQuoted,
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
                    Some(c) => {
                        word.push(c);
                        DoubleQuoted
                    }
                },
                Unquoted => match next {
                    None => {
                        words.push(std::mem::replace(&mut word, String::new()));
                        break;
                    }
                    Some('\'') => SingleQuoted,
                    Some('\"') => DoubleQuoted,
                    Some(' ') => {
                        words.push(std::mem::replace(&mut word, String::new()));
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