#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit 0" => exit(0),
            &_ => {
                println!("{}: command not found", input);
            }
        }

    }
}
