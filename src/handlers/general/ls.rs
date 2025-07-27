pub fn ls(path: String) {
    let dir = std::fs::read_dir(path).unwrap();
    for content in dir {
        println!("{}", content.unwrap().path().display());
    }
}