use std::{fs, path::{Path, PathBuf}};

use crate::handlers::errors::{CliErr, ErrKind};

pub fn parse_path(path: String) -> Result<String, CliErr> {

    if path.starts_with("%USER/") || path.starts_with("~/") {
        let home = dirs::home_dir().ok_or_else(|| CliErr::set_err("No home dir", ErrKind::DirMissing))?;

        let path = path.splitn(2, |c| c == '\\' || c == '/')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let result = Path::new(&format!("{}/{}", home.to_str().unwrap(), path.iter().nth(1).unwrap()))
            .to_str()
            .unwrap()
            .to_string()
            .replace("\\", "/");

        Ok(result)
    } else if path.starts_with(":") {
        let mut components = path.split(|c| c == '\\' || c == '/')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let first = components.get(0).ok_or_else(|| {
            CliErr::set_err("Empty path", ErrKind::InvalidData)
        })?;
        let prefix = format!("{};", &first[1..]);

        let mut alias_path = PathBuf::from(
            dirs::home_dir()
                .ok_or_else(|| CliErr::set_err("No home dir", ErrKind::DirMissing))?
        );
        alias_path.push(".cdputils/cdpaliases.txt");

        let file = fs::read_to_string(alias_path)
            .map_err(|e| {
                CliErr::set_err(&e.to_string(), ErrKind::IoError)
            })?;

        let line = file
            .lines()
            .find(|line| line.starts_with(&prefix))
            .ok_or_else(|| {
                CliErr::set_err("Alias not found", ErrKind::NotFound)
            })?;

        let mut parts = line.split(';');
        let _ = parts.next();
        let real_path = parts.next()
            .ok_or_else(|| {
                CliErr::set_err("No path related to this alias", ErrKind::InvalidData)
            })?;

        let normalized = real_path.replace(r"\", "/");
        components[0] = normalized;

        let result = components.join("/");

        Ok(result)
    } else {
        Ok(path)
    }
}