use std::{fs, io::Write};

use crate::{handlers::errors::{CliErr, ErrKind}, utils::setup::create_cdp_cmdaliases::create_cdp_cmdaliases};

pub fn cmd_aliases(cmd: String, alias: String) -> Result<(), CliErr> {
    if cmd.is_empty() || alias.is_empty() {
        return Err(CliErr::set_err("Command/alias cannot be empty", ErrKind::InvalidData));
    }

    let alias_path = create_cdp_cmdaliases()?;

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
        writeln!(file, "{};{}", alias, cmd)
            .map_err(|e| {
                CliErr::set_err(&format!("Can't write to file: {e}"), ErrKind::IoError)
            })?;

        println!("Alias ({}) written.", alias);
    } else {
        println!("Alias ({}) already exists.", alias);
    }

    Ok(())
}