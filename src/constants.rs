use lazy_static::lazy_static;
use crate::handlers::errors::{CliErr, ErrKind};

lazy_static! {
    pub static ref CDPALIASES: Result<String, CliErr> = set_cdpaliases();
    pub static ref CDPPROJECTS: Result<String, CliErr> = set_cdpprojects();
}

fn set_cdpaliases() -> Result<String, CliErr> {
    let home = dirs::home_dir().ok_or_else(|| CliErr::set_err("No home dir", ErrKind::DirMissing))?;
    let path = format!("{}/.cdputils/cdpaliases.txt", home.display());
    let path = std::path::Path::new(&path);
    Ok(path.display().to_string())
}

fn set_cdpprojects() -> Result<String, CliErr> {
    let home = dirs::home_dir().ok_or_else(|| CliErr::set_err("No home dir", ErrKind::DirMissing))?;
    let path = format!("{}/.cdputils/projects", home.display());
    let path = std::path::Path::new(&path);
    Ok(path.display().to_string())
}