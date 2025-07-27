pub fn change_to_current() -> String {
    let mut path = String::new();
    if cfg!(target_os = "windows") {
        if let Ok(user) = std::env::var("USERPROFILE") {
            path = user;
        }
    } else {
        if let Ok(home) = std::env::var("HOME") {
            path = home;
        }
    }
    println!("{}", path.clone());
    path
}