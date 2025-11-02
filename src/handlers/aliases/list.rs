use std::fs;
use crossterm::style::Stylize;

use crate::{handlers::errors::{CliErr, ErrKind}, utils::setup::create_cdpaliases::create_cdpaliases};

pub fn list_aliases() -> Result<(), CliErr> {
    let path = create_cdpaliases()?;

    let content = fs::read_to_string(&path)
        .map_err(|e| {
            CliErr::set_err(&format!("Error opening file: {e}"), ErrKind::IoError)
        })?;

    for parts in content.lines() {
        let components = parts.split(';').collect::<Vec<_>>();
        println!("{}: {}", components[0].green().bold(), components[1].magenta().italic().bold())
    }
    println!("{}: {}", "Path".yellow(), path.replace(r"\", "/").blue());

    Ok(())
}