use crate::builtin::Builtin;
use anyhow::Result;
use pathsearch::find_executable_in_path;
use std::process::Output;

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self {
            name,
            args,
        }
    }

    pub fn exec(&mut self) -> Result<Output> {
        if let Some(builtin) = Builtin::is_builtin(&self.name) {
            let output = Output { status: Default::default(), stdout: builtin.exec(&self.args)?, stderr: vec![] };
            return Ok(output)
        }

        if find_executable_in_path(&self.name).is_some() {
            let proc = std::process::Command::new(&self.name)
                .args(&self.args)
                .output()?;
            return Ok(proc)
        }

        let not_found = format!("{}: command not found\n", self.name).into_bytes();
        let output = Output { status: Default::default(), stdout: not_found, stderr: vec![] };
        Ok(output)
    }
}