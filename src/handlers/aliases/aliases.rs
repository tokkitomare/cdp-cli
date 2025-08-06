use std::{fs, io::Write};

use crate::{handlers::errors::{CliErr, ErrKind}, utils::create_cdputils::create_cdputils};

pub fn aliases(path: String, alias: String) -> Result<(), CliErr> {
    let dir = create_cdputils()?;
    let format = &format!("{}/cdpaliases.txt", dir);
    let alias_path = std::path::Path::new(format);

    if !alias_path.exists() {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(alias_path)
            .map_err(|e| CliErr::set_err(&format!("Can't create the file: {e}"), ErrKind::IoError))?;
    }

    let content = fs::read_to_string(&alias_path)
        .map_err(|e| CliErr::set_err(&format!("Can't read to string: {e}"), ErrKind::IoError))?
        .lines()
        .find(|line| line.starts_with(&format!("{};", alias)))
        .is_none();

    if content {
        let mut file = fs::File::options()
                .create(true)
                .append(true)
                .open(alias_path)
                .map_err(|e| {
                    CliErr::set_err(&format!("Can't open: {e}"), ErrKind::IoError)
                })?;
        writeln!(file, "{};{}", alias, path)
            .map_err(|e| {
                CliErr::set_err(&format!("Can't write to file: {e}"), ErrKind::IoError)
            })?;

        println!("Alias ({}) written.", alias);
    } else {
        println!("Alias ({}) already exists.", alias);
    }
    
    Ok(())
}