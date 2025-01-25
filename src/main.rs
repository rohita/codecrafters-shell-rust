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

        stdin.read_line(&mut input).unwrap();
        let split_input = input.trim().split_whitespace().collect::<Vec<&str>>();
        let command = split_input[0];
        let args = split_input[1..].to_vec();

        match command {
            "echo"  => println!("{}", args.join(" ")),
            "exit"  => exit(args[0].parse().unwrap()),
            _       => println!("{}: command not found", command)
        }
    }
}
