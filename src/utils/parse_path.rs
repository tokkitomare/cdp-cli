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
    } else if path.contains(":") {
        let components = split_alias_path(path);

        let (index_path, path_alias) = {
            let f = components
                .iter()
                .enumerate()
                .find_map(|(i, f)| {
                    if f.starts_with(":") {
                        Some((i, f))
                    } else {
                        None
                    }
                })
                .ok_or(CliErr::set_err("No path included", ErrKind::InvalidData))?;

            (f.0, f.1)
        };

        let prefix = format!("{};", &path_alias[1..]);

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
            .ok_or(
                CliErr::set_err("No path related to this alias", ErrKind::InvalidData)
            )?;

        let normalized = real_path.replace(r"\", "/");

        let mut result: String;
        
        if components.len() == 1 {
            result = normalized
        } else {
            std::env::set_current_dir(real_path)
                .map_err(|e| CliErr::set_err(e.to_string(), ErrKind::IoError))?;

            match components.len() {
                2 => {
                    let other_idx = if index_path == 0 { 1 } else { 0 };
                    result = fs::canonicalize(&components[other_idx])
                        .map_err(|e| CliErr::set_err(e.to_string(), ErrKind::InvalidData))?
                        .display()
                        .to_string();
                    #[cfg(windows)]
                    {
                        result = result.strip_prefix(r"\\?\")
                            .ok_or(CliErr::set_err(r"No prefix '\\?\' found", ErrKind::NotFound))?
                            .to_string();
                    }
                },
                _ => {
                    let (before, after) = (&components[index_path-1], &components[index_path+1]);
                    result = fs::canonicalize(after)
                        .map_err(|e| CliErr::set_err(e.to_string(), ErrKind::InvalidData))?
                        .display()
                        .to_string();
                    #[cfg(windows)]
                    {
                        result = result.strip_prefix(r"\\?\")
                            .ok_or(CliErr::set_err(r"No prefix '\\?\' found", ErrKind::NotFound))?
                            .to_string();
                    }

                    std::env::set_current_dir(result)
                        .map_err(|e| CliErr::set_err(e.to_string(), ErrKind::IoError))?;

                    result = fs::canonicalize(before)
                        .map_err(|e| CliErr::set_err(e.to_string(), ErrKind::InvalidData))?
                        .display()
                        .to_string();
                    #[cfg(windows)]
                    {
                        result = result.strip_prefix(r"\\?\")
                            .ok_or(CliErr::set_err(r"No prefix '\\?\' found", ErrKind::NotFound))?
                            .to_string();
                    }
                }
            }
        }

        Ok(result)
    } else {
        Ok(path)
    }
}

fn split_alias_path(s: impl AsRef<str>) -> Vec<String> {
    let s = s.as_ref();
    if let Some(colon_idx) = s.find(":") {
        let before = if colon_idx > 0 && &s[..2] != "./" {
            &s[..colon_idx-1]
        } else {
            ""
        };
        let rest = &s[colon_idx..];

        if let Some(end) = rest.find(|c| c == '\\' || c == '/') {
            let middle = &rest[..end];
            let end = &rest[end+1..];

            return vec![before.to_string(), middle.to_string(), end.to_string()]
                .iter()
                .filter(|f| !f.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
        }

        return vec![before.to_string(), rest.to_string()]
            .iter()
            .filter(|f| !f.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
    }

    vec![s.to_string()]
}