#![allow(unused)]
// TODO: Make the Errors Kind
// TODO: Display for CliErr and ErrKind

use std::{
    error::Error as StdError,
    fmt::Display,
};

#[derive(Debug)]
pub enum ErrKind {

}

#[derive(Debug)]
pub struct CliErr {
    message: String,
    kind: ErrKind,
}

impl Display for CliErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Make the display for CliErr struct.");
    }
}

impl StdError for CliErr {}

impl CliErr {
    pub fn set_err(msg: &str, kind: ErrKind) -> Self {
        Self {
            message: msg.to_owned(),
            kind,
        }
    }
}