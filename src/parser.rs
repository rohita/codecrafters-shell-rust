pub struct Parser;

enum State {
    Delimiter,
    Unquoted,
    SingleQuoted,
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
                Unquoted => match next {
                    None => {
                        words.push(std::mem::replace(&mut word, String::new()));
                        break;
                    }
                    Some('\'') => SingleQuoted,
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