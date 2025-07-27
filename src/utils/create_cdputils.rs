pub fn create_cdputils() -> Result<String, Box<dyn std::error::Error>> {
    let home = if cfg!(windows) {
        std::env::var("USERPROFILE")?
    } else {
        std::env::var("HOME")?
    };
    let dir = [home.as_str(), ".cdputils"].iter().collect::<std::path::PathBuf>();
    
    std::fs::create_dir_all(&dir)?;

    Ok(dir.to_str().unwrap().to_string())
}