pub fn open_editor(editor: &str, path: String) {
    match editor {
        "vsc" => {
            std::process::Command::new("cmd")
                .current_dir(path)
                .args(&["/C", "code", "."])
                .spawn()
                .expect("Cant open Visual Studio Code");
        },
        _ => panic!("Invalid editor")
    }
}