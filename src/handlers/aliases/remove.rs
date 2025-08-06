use std::{fs, io::Write};

use crossterm::style::Stylize;

use crate::{constants, handlers::errors::{CliErr, ErrKind}};

pub fn remove_alias(identifier: String) -> Result<(), CliErr> {
    let path = std::path::Path::new(constants::CDPALIASES.as_ref().unwrap());
    let content = fs::read_to_string(path)
        .map_err(|e| CliErr::set_err(&format!("Can't read: {e}"), ErrKind::FileMissing))?;
    let format = format!("{};", identifier);
    let exist = content
        .lines()
        .find(|l| l.starts_with(&format))
        .is_some();

    if !exist {
        return Err(
            CliErr::set_err("This alias doesn't exist", ErrKind::NotFound)
        );
    }

    let mut changed = String::new();
    for line in content.lines() {
        if !line.starts_with(&format) {
            changed.push_str(line);
            changed.push_str("\n");
        }
    }

    let tmp = path.with_extension("tmp");
    {
        let mut tmp = fs::File::create(&tmp)
            .map_err(|e| CliErr::set_err(&e.to_string(), ErrKind::IoError))?;
        tmp.write_all(changed.as_bytes())
            .map_err(|e| CliErr::set_err(&e.to_string(), ErrKind::IoError))?;
        tmp.flush()
            .map_err(|e| CliErr::set_err(&e.to_string(), ErrKind::IoError))?;
    }
    fs::rename(tmp, path)
        .map_err(|e| CliErr::set_err(&format!("Can't rename the file: {e}"), ErrKind::IoError))?;
    println!("{} removed successfully!", identifier.red().bold());

    Ok(())
}