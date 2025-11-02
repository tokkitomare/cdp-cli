use std::fs;
use crate::handlers::errors::{CliErr, ErrKind};
use crate::utils::setup::create_cdputils::create_cdputils;

pub fn create_projects() -> Result<String, CliErr> {
    let mut dir = create_cdputils()?;
    dir.push_str("/projects");

    fs::create_dir_all(&dir)
        .map_err(|e| {
            CliErr::set_err(e.to_string(), ErrKind::IoError)
        })?;

    Ok(dir)
}