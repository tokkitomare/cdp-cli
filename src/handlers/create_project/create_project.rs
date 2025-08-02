use crate::handlers::aliases::aliases::aliases;
use crate::handlers::errors::{CliErr, ErrKind};
use crate::types::Langs;
use crate::utils::create_cdputils::create_cdputils;
use std::process::Command;
use std::{fs, io::Write, path::PathBuf};

macro_rules! create_project_default {
    ($dir:expr, $name:expr, $extension:literal, $write:expr, $alias:expr) => {{
        let mut path = PathBuf::from($dir);
        path.push($name);
        fs::create_dir_all(&path)
            .map_err(|e| {
                CliErr::set_err(&e.to_string(), ErrKind::IoError)
            })?;

        path.push(format!("main.{}", $extension));
        let mut f = fs::File::create(&path)
            .map_err(|e| {
                CliErr::set_err(&e.to_string(), ErrKind::IoError)
            })?;

        writeln!(f, "{}", $write).unwrap();

        if let Some(alias) = $alias {
            aliases(format!("{}\\{}", $dir, $name), alias.clone())?;
            println!("Success! You can now type `cdp general --vsc \":{}\"` to open it on Visual Studio Code.", alias);
        } else {
            println!("Success! You can now type `cdp general --vsc \"{}\\{}\"` to open it on Visual Studio Code.", $dir, $name);
        }
    }};
}

macro_rules! response {
    ($alias:expr, $dir:expr, $name:expr) => {
        if let Some(alias) = $alias {
            aliases(format!("{}\\{}", $dir.display(), $name), alias.clone())?;
            println!("Success! You can now type `cdp general --vsc \":{}\"` to open it on Visual Studio Code.", alias);
        } else {
            println!("Success! You can now type `cdp general --vsc \"{}\\{}\"` to open it on Visual Studio Code.", $dir.display(), $name);
        }
    };
}

pub fn create_project(lang: Langs, name: String, alias: Option<String>) -> Result<(), CliErr> {
    let dir = [create_cdputils()?, "projects".to_string()].iter().collect::<PathBuf>();
    fs::create_dir_all(&dir)
        .map_err(|e| {
            CliErr::set_err(&e.to_string(), ErrKind::IoError)
        })?;

    if cfg!(windows) {
        match lang {
            Langs::Rs => {
                let status = Command::new("cmd")
                    .args(&[
                        "/C",
                        "cargo", "new", 
                        &format!("{}", name), "--bin",
                    ])
                    .current_dir(&dir)
                    .status()
                    .expect("Can not create the project");
                
                if status.success() {
                    response!(alias, dir, name);
                } else {
                    eprintln!("Can't create. {:?}", status.code())
                }
            },
            Langs::C => {
                create_project_default!(
                    &dir.display().to_string(), 
                    &name, 
                    "c", 
                    "#include <stdio.h>\n\nint main() {\n\tprintf(\"Created by cdp CLI\");\n\treturn 0;\n}", 
                    &alias
                );
            },
            Langs::Cpp => {
                create_project_default!(
                    &dir.display().to_string(), 
                    &name, 
                    "cpp", 
                    "#include <iostream>\n\nint main() {\n\tstd::cout << \"Created by cdp CLI\" << std::endl;\n\treturn 0;\n}", 
                    &alias
                );
            },
            Langs::Py => {
                create_project_default!(
                    &dir.display().to_string(), 
                    &name, 
                    "py", 
                    "def main() -> None:\n\tprint(\"Created by cdp CLI\")\n\nif __name__ == '__main__':\n\tmain()",
                    &alias
                );
            },
            Langs::Js => {
                let mut folder = PathBuf::from(&dir);
                folder.push(&name);
                fs::create_dir_all(&folder)
                    .map_err(|e| {
                        CliErr::set_err(&e.to_string(), ErrKind::IoError)
                    })?;

                let status = Command::new("cmd")
                    .args(&[
                        "/C",
                        "npm", "init", "-y"
                    ])
                    .current_dir(&folder)
                    .status()
                    .unwrap();

                let file = [&folder.to_str().unwrap(), "app.js"].iter().collect::<PathBuf>();
                let mut f = fs::File::create(file)
                    .map_err(|e| {
                        CliErr::set_err(&e.to_string(), ErrKind::IoError)
                    })?;
                writeln!(f, "console.log(\"Created by cdp CLI\");")
                    .map_err(|e| {
                        CliErr::set_err(&format!("Can't write to file: {e}"), ErrKind::IoError)
                    })?;

                if status.success() {
                    response!(alias, dir, name);
                } else {
                    eprintln!("Can't create. {:?}", status.code())
                }
            },
            Langs::Ts => {
                let mut folder = PathBuf::from(&dir);
                folder.push(&name);
                fs::create_dir_all(&folder)
                    .map_err(|e| {
                        CliErr::set_err(&e.to_string(), ErrKind::IoError)
                    })?;
                let mut src = PathBuf::from(&folder);
                src.push("src");
                fs::create_dir(&src)
                    .map_err(|e| {
                        CliErr::set_err(&e.to_string(), ErrKind::IoError)
                    })?;

                let process1 = Command::new("cmd")
                    .args(&[
                        "/C",
                        "npm", "init", "-y"
                    ])
                    .current_dir(&folder)
                    .status()
                    .unwrap();

                let process2 = Command::new("cmd")
                    .args(&[
                        "/C",
                        "npm", "install", "typescript", "--save-dev",
                    ])
                    .current_dir(&folder)
                    .status()
                    .unwrap();

                let process3 = Command::new("cmd")
                    .args(&[
                        "/C",
                        "npx", "tsc", "--init",
                    ])
                    .current_dir(&folder)
                    .status()
                    .unwrap();

                let file = [&src.to_str().unwrap(), "app.ts"].iter().collect::<PathBuf>();
                let mut f = fs::File::create(file)
                    .map_err(|e| {
                        CliErr::set_err(&e.to_string(), ErrKind::IoError)
                    })?;

                writeln!(f, "const message: string = \"Created by cdp CLI\";\nconsole.log(message);")
                    .map_err(|e| {
                        CliErr::set_err(&format!("Can't write to file: {e}"), ErrKind::IoError)
                    })?;

                if process1.success() && process2.success() && process3.success() {
                    response!(alias, dir, name);
                } else {
                    eprintln!("Can't create. Process 1:\n{:?}\nProcess 2:\n{:?}\nProcess 3:\n{:?}", process1.code(), process2.code(), process3.code())
                }
            },
        }
    } else {
        todo!("CreateProject Handler for unix.");
    }

    Ok(())
}