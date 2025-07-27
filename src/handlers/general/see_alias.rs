use std::{env::var, fs};

pub fn see_alias(alias: String) -> Result<String, Box<dyn std::error::Error>> {
    let path: String;
    if cfg!(windows) {
        path = format!("{}/.cdputils/cdpaliases.txt", var("USERPROFILE").expect("No envVar USERPROFILE found."));
    } else {
        path = format!("{}/.cdputils/cdpaliases.txt", var("HOME").expect("No envVar HOME found."));
    }

    let content = fs::read_to_string(path)?
        .lines()
        .find(|line| line.starts_with(&format!("{};", alias)))
        .and_then(|line| line.split(';').nth(1))
        .map(|s| s.to_string())
        .ok_or_else(|| "Alias not found")?;

    Ok(content)
}