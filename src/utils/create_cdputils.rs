use crate::handlers::errors::{CliErr, ErrKind};

pub fn create_cdputils() -> Result<String, CliErr> {
    let home = dirs::home_dir()
        .ok_or_else(|| CliErr::set_err("No home dir", ErrKind::DirMissing))?;

    let dir = [home.to_str().unwrap(), ".cdputils"].iter().collect::<std::path::PathBuf>();
    
    std::fs::create_dir_all(&dir)
        .map_err(|e| {
            CliErr::set_err(&e.to_string(), ErrKind::IoError)
        })?;

    Ok(dir.to_str().unwrap().to_string())
}