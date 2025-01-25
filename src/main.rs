mod builtin;

use crate::builtin::Builtin;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut input).unwrap();
        let split_input: Vec<_> = input.trim().split_whitespace().collect();
        let command = split_input[0];
        let args = split_input[1..].to_vec();
        Builtin::from(command).handle(args);
    }
}