use crossterm::style::Stylize;

use crate::handlers::errors::{CliErr, ErrKind};

pub fn ls(path: String) -> Result<(), CliErr> {
    let dir = std::fs::read_dir(path)
        .map_err(|e| {
            CliErr::set_err(&e.to_string(), ErrKind::DirMissing)
        })?;

    for content in dir {
        println!("-- {}", content.unwrap().path().display().to_string().replace("\\", "/").green());
    }

    Ok(())
}