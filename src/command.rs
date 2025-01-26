use crate::builtin::Builtin;
use anyhow::Result;
use pathsearch::find_executable_in_path;

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(input: String) -> Self {
        let split_input: Vec<_> = input
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Self {
            name: split_input[0].clone(),
            args: split_input[1..].to_vec(),
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