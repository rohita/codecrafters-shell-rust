use crate::builtin::Builtin;
use anyhow::Result;
use pathsearch::find_executable_in_path;
use crate::parser::Parser;

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(input: String) -> Self {
        let parsed_input = Parser::parse(&input.trim());

        Self {
            name: parsed_input[0].clone(),
            args: parsed_input[1..].to_vec(),
        }
    }

    pub fn exec(&mut self) -> Result<Vec<u8>> {
        if let Some(builtin) = Builtin::is_builtin(&self.name) {
            return builtin.exec(&self.args)
        }

        if find_executable_in_path(&self.name).is_some() {
            let proc = std::process::Command::new(&self.name)
                .args(&self.args)
                .output()?;
            return Ok(proc.stdout)
        }

        let not_found = format!("{}: command not found\n", self.name).into_bytes();
        Ok(not_found)
    }
}