use std::error::Error;
use enigo::{Enigo, Keyboard, Mouse};
use serde::{Deserialize, Serialize};
use crate::steps::program::Program;

pub mod program;

/// Defines the set of actions that may be performed as a step within a macro script
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Step {
    //Start with those supported by Enigo:

    /// See [`enigo::agent::Token::Text`]
    Text(String),
    /// See [`enigo::agent::Token::Key`]
    Key(enigo::Key, enigo::Direction),
    /// See [`enigo::agent::Token::Raw`]
    Raw(u16, enigo::Direction),
    /// See [`enigo::agent::Token::Button`]
    Button(enigo::Button, enigo::Direction),
    /// See [`enigo::agent::Token::MoveMouse`]
    MoveMouse(i32, i32, enigo::Coordinate),
    /// See [`enigo::agent::Token::Scroll`]
    Scroll(i32, enigo::Axis),

    //Then add our custom steps:

    /// Executes a program (with or without arguments)
    Run(Program),
}

impl Step {
    pub fn execute(&self, enigo: &mut Enigo) -> Result<(), Box<dyn Error>> {
        match self {
            Step::Key(key, dir) => enigo.key(*key, *dir).map_err(Box::from),
            Step::Raw(key, dir) => enigo.raw(*key, *dir).map_err(Box::from),
            Step::Text(text) => enigo.text(text).map_err(Box::from),
            Step::Button(button, dir) => enigo.button(*button, *dir).map_err(Box::from),
            Step::MoveMouse(x, y, coordinate) => enigo.move_mouse(*x, *y, *coordinate).map_err(Box::from),
            Step::Scroll(magnitude, axis) => enigo.scroll(*magnitude, *axis).map_err(Box::from),
            
            Step::Run(program) => program.execute(),
        }
    }
}
