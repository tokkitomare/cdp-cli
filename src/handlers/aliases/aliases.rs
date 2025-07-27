use std::{fs, io::Write};

use crate::utils::create_cdputils::create_cdputils;

pub fn aliases(path: String, alias: String) -> Result<(), Box<dyn std::error::Error>> {
    let dir = create_cdputils()?;
    let alias_path = format!("{}/cdpaliases.txt", dir);

    let content = fs::read_to_string(&alias_path)?
        .lines()
        .find(|line| line.starts_with(&format!("{};", alias)))
        .is_none();

    if content {
        let mut file = fs::File::options()
                .create(true)
                .append(true)
                .open(alias_path)?;
        writeln!(file, "{};{}", alias, path)?;
    } else {
        println!("Alias ({}) already exists.", alias);
    }
    
    Ok(())
}