use std::{fs, io::Write, path::Path};

use crossterm::style::Stylize;

use crate::{handlers::errors::{CliErr, ErrKind}, utils::setup::create_cdpaliases::create_cdpaliases};

struct Update {
    new_alias: Option<String>,
    new_path: Option<String>,
}

pub fn parse_response(arg: String) -> Result<(), CliErr> {
    let arg = arg.trim();

    if arg.starts_with("(") && arg.ends_with(")") {
        let path = create_cdpaliases()?;
        let content = fs::read_to_string(&path)
            .map_err(|e| CliErr::set_err(&format!("Can't read file: {}", e), ErrKind::FileMissing))?;

        let parts = split_first(arg);
        let parts = parts
            .iter()
            .filter(|&s| !s.is_empty())
            .map(|s| s.as_str())
            .collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(
                CliErr::set_err(
                    &format!(
                        "Not enough arguments supplied. The correct format is: {}", 
                        "(<ID: current ALIAS>: <NEW_ALIAS: type the new alias or just type '.' to leave unchanged>;<NEW_PATH: type the new path or . to leave unchanged>)".green()
                    ), 
                    ErrKind::InvalidData
                )
            );
        }

        if content
            .lines()
            .find(|l| l.starts_with(&format!("{};", parts[0])))
            .is_none()
        {
            return Err(
                CliErr::set_err(&format!("No alias {}", parts[0].red()), ErrKind::InvalidData)
            );
        }

        let mut iter = parts.iter();
        let identifier = iter.next().unwrap();
        let new_alias = match iter.next() {
            Some(&s) if s != "." => Some(s.to_owned()),
            _ => None,
        };
        let new_path = match iter.next() {
            Some(&s) if s != "." => Some(s.to_owned()),
            _ => None,
        };

        upd_line(Path::new(&path), &identifier, Update { new_alias, new_path })?;
        println!("{}", "Changes:".on_green().white());
        if parts[1] != "." {
            println!("{} to {}", parts[0].yellow(), parts[1].green());
        }
        if parts[2] != "." {
            let content = content
                .lines()
                .find(|x| x.starts_with(&format!("{};", parts[0])))
                .unwrap();
            let old_path = content.split(';').collect::<Vec<_>>()[1];
            println!("{} to {}", old_path.yellow(), parts[2].green());
        }
        return Ok(());
    }

    Err(CliErr::set_err(&format!("You {} provide parentheses around the argument. Example: {}", "must".bold().yellow(), "(<ID>: <NEW_ALIAS>;<NEW_PATH>)".green()), ErrKind::InvalidData))
}

fn split_first(s: impl AsRef<str>) -> Vec<String> {
    let s = s.as_ref().trim_matches(['(', ')']);
    if let Some(colon_idx) = s.find(":") {
        let before = &s[..colon_idx].trim();
        let after_c = &s[colon_idx + 1..].trim();
        if let Some(semi_idx) = after_c.find(";") {
            let middle = &after_c[..semi_idx].trim();
            let after_s = &after_c[semi_idx + 1..].trim();
            vec![
                before.to_string(),
                middle.to_string(),
                after_s.to_string(),
            ]
        } else {
            vec![
                before.to_string(),
                after_c.to_string(),
            ]
        }
    } else {
        vec![s.to_string()]
    }
}

fn upd_line(path: impl AsRef<Path>, search: &str, update: Update) -> Result<(), CliErr> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .map_err(|e| CliErr::set_err(&format!("Can't read: {e}"), ErrKind::FileMissing))?;

    let part = content
        .lines()
        .filter(|&x| x.starts_with(&format!("{};", search)))
        .collect::<String>();

    let mut components = part.split(';')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    if let Some(update) = update.new_alias {
        components[0] = update;
    }

    if let Some(update) = update.new_path {
        components[1] = update;
    }

    let full_line = components.join(";");

    let content = content.replace(&part, &full_line);

    let tmp = path.with_extension("tmp");
    {
        let mut tmp = fs::File::create(&tmp).unwrap();
        tmp.write_all(content.as_bytes()).unwrap();
        tmp.flush().unwrap();
    }
    fs::rename(tmp, path).unwrap();

    Ok(())
}