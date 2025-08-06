use std::{fs, path::PathBuf};

use crossterm::style::Stylize;

use crate::handlers::errors::{CliErr, ErrKind};

pub fn list_aliases() -> Result<(), CliErr> {
    let home = dirs::home_dir()
        .ok_or_else(|| CliErr::set_err("No home dir", ErrKind::DirMissing))?;

    let mut path = PathBuf::from(home);
    path.push(".cdputils/cdpaliases.txt");

    if !path.exists() {
        if cfg!(windows) {
            return Err(CliErr::set_err("path `%USERPROFILE%/.cdputils/cdpaliases.txt` doesn't even exist.", ErrKind::FileMissing));
        } else {
            return Err(CliErr::set_err("path `~/.cdputils/cdpaliases.txt` doesn't even exist.", ErrKind::FileMissing));
        }
    } else {
        let content = fs::read_to_string(&path)
            .map_err(|e| {
                CliErr::set_err(&format!("Error opening file: {e}"), ErrKind::IoError)
            })?;

        for parts in content.lines() {
            let components = parts.split(';').collect::<Vec<_>>();
            println!("{};{}", components[0].green().bold(), components[1].magenta().italic().bold())
        }
        println!("{}: {}", "Path".yellow(), path.display().to_string().replace(r"\", "/").blue());
    }

    Ok(())
}