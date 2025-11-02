use std::fs;
use crate::handlers::errors::{CliErr, ErrKind};
use crate::utils::setup::create_cdputils::create_cdputils;

pub fn create_cdp_cmdaliases() -> Result<String, CliErr> {
    let mut file = create_cdputils()?;
    file.push_str("/cdp_cmdaliases.txt");

    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .truncate(false)
        .open(&file)
        .map_err(|e| {
            CliErr::set_err(e.to_string(), ErrKind::IoError)
        })?;

    Ok(file)
}