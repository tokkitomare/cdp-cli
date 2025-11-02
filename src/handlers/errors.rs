#![allow(unused)]

use std::{
    error::Error as StdError,
    fmt::Display,
};

use crossterm::style::Stylize;

#[derive(Debug)]
pub enum ErrKind {
    FileMissing,
    DirMissing,
    PermissionDenied,
    InvalidData,
    NotFound,
    IoError,
    Other(String),
}

impl Display for ErrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String;
        match self {
            Self::FileMissing => display = "The file is missing".to_string(),
            Self::DirMissing => display = "The directory is missing".to_string(),
            Self::PermissionDenied => display = "Permission denied".to_string(),
            Self::InvalidData => display = "Invalid data".to_string(),
            Self::NotFound => display = "Not found".to_string(),
            Self::IoError => display = "Input/Output error".to_string(),
            Self::Other(msg) => display = msg.clone(),
        }
        
        writeln!(f, "{}", display)
    }
}

#[derive(Debug)]
pub struct CliErr {
    message: String,
    kind: ErrKind,
}

impl Display for CliErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:\n\tmessage: {}\n\tkind: {}", "CDP Error".red().bold(), self.message.clone().red(), self.kind)
    }
}

impl StdError for CliErr {}

impl CliErr {
    pub fn set_err(msg: impl AsRef<str>, kind: ErrKind) -> Self {
        Self {
            message: msg.as_ref().to_owned(),
            kind,
        }
    }
}