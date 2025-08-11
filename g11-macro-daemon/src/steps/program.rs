use std::{
    process::{Command, Stdio},
    thread,
};
use std::error::Error;
use serde::{Deserialize, Serialize};

/// The first value is the program, and the second value is the arguments to be passed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program(String, #[serde(default)] Vec<String>);

impl Program {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        Command::new(&self.0)
            .args(&self.1)
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map(|mut child| thread::spawn(move || child.wait()))
            .map(|_| ())
            .map_err(Box::from)
    }
}
