use crate::{handlers::errors::{CliErr, ErrKind}, types::Editors};

pub fn open_editor(editor: Editors, path: String) -> Result<(), CliErr> {
    if cfg!(windows) {
        match editor {
            Editors::Vsc => {
                println!("Opening VS Code...");
                std::process::Command::new("cmd")
                    .current_dir(path)
                    .args(&["/C", "code", "."])
                    .spawn()
                    .map_err(|e| {
                        CliErr::set_err(&format!("Can't open VS Code: {e}"), ErrKind::Other("Process".to_string()))
                    })?;
            },
        }
    } else {
        todo!("Make to Unix")
    }

    Ok(())
}