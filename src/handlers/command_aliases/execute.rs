use std::fs;

use crossterm::style::Stylize;

use crate::{handlers::errors::{CliErr, ErrKind}, utils::setup::create_cdp_cmdaliases::create_cdp_cmdaliases};

pub fn execute_alias(alias: String) -> Result<(), CliErr> {
    if !cfg!(windows) {
        return Err(
            CliErr::set_err(format!("Sorry. Execute command aliases is {}.", "Windows-only".bold().on_red()), ErrKind::Other("Plataform Not Supported".to_owned()))
        );
    }

    let alias_path = create_cdp_cmdaliases()?;

    let file = fs::read_to_string(alias_path)
        .map_err(|_| {
            CliErr::set_err("Use 'cdp command-aliases <COMMAND> <ALIAS>' to create the file.", ErrKind::FileMissing)
        })?;

    let prefix = format!("{};", &alias);
    let line = file
        .lines()
        .find(|q| q.starts_with(&prefix))
        .ok_or_else(|| {
            CliErr::set_err("Alias not found", ErrKind::NotFound)
        })?;

    let mut parts = line.split(';');
    let _ = parts.next();
    let real_command = parts.next()
        .ok_or_else(|| {
            CliErr::set_err("No command related to this alias", ErrKind::InvalidData)
        })?;

    // It's necessary to use regex in this part so that commands with spaces between ' or " can be identified without errors
    let regex = regex::Regex::new(r#"'([^']*)'|"([^"]*)"|(\S+)"#)
        .map_err(|e| CliErr::set_err(e.to_string(), ErrKind::Other("Regex Error".to_owned())))?;

    let real_command_parts = regex.captures_iter(real_command)
        .map(|cap| {
            cap.get(1)
                .or_else(|| cap.get(2))
                .or_else(|| cap.get(3))
                .ok_or_else(||
                    CliErr::set_err("Can't get any capture", ErrKind::Other("Regex Error".to_owned()))
                )
                .map(|x| x.as_str().to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    std::process::Command::new("cmd")
        .arg("/C")
        .args(&real_command_parts)
        .current_dir(".")
        .spawn()
        .map_err(|e| CliErr::set_err(e.to_string(), ErrKind::Other("Execution Error".to_string())))?;

    Ok(())
}