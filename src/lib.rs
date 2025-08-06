mod handlers;
mod types;
mod utils;
mod constants;

use clap::CommandFactory;
use crossterm::style::Stylize;
use types::*;
use handlers::{
    general::*, aliases::*, create_project::*,
};

use crate::{handlers::errors::CliErr, utils::parse_path};

pub fn cli_run() -> Result<(), CliErr> {
    let args = Cli::parse();

    match args.command {
        Some(Commands::General(cmd)) => {
            let path = match parse_path::parse_path(cmd.path.clone()) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{e}");
                    cmd.path
                }
            };

            if cmd.current_user {
                let home_path = if cfg!(windows) {
                    "%USER/"
                } else {
                    "~/"
                };
                println!(
                    "{}: the {} (or {}) flag is no longer necessary.\nYou can specify whether the directory is in user/home using: {}your_dir", 
                    "Deprecated".yellow(), 
                    "--current-user".red(), 
                    "-C".red(), 
                    home_path.green()
                );
            }
            if cmd.alias {
                println!(
                    "{}: the {} (or {}) flag is no longer necessary.\nYou can specify whether is an alias using {}. (e.g. {})", 
                    "Deprecated".yellow(), 
                    "--alias".red(), 
                    "--al".red(),
                    ":".green(),
                    ":cdpproject/src/parse_path.rs".green()
                );
            }
            if cmd.ls {
                if let Err(e) = ls::ls(path.clone()) {
                    eprintln!("{e}");
                }
            }
            if cmd.vsc {
                println!(
                    "{}: the {} flag is no longer necessary. Use {} (or {}) instead. (e.g. {})", 
                    "Deprecated".yellow(), 
                    "--vsc".red(),
                    "--editor".green(),
                    "-E".green(),
                    r#"cdp general ":cdptests/assets" -E vsc"#.green()
                );
            }
            if cmd.editor.is_some() {
                if let Err(e) = editors::open_editor(Editors::Vsc, path.clone()) {
                    eprintln!("{e}");
                }
            }
        },
        Some(Commands::Aliases(cmd)) => {
            if let (Some(path), Some(alias)) = (cmd.path, cmd.alias) {
                    if let Err(e) = aliases::aliases(path, alias) {
                        eprintln!("{e}");
                    }
            }
            if let Some(edit) = cmd.edit {
                    if let Err(e) = edit::parse_response(edit) {
                        eprintln!("{e}");
                    }
            }
            if let Some(remove) = cmd.remove {
                    if let Err(e) = remove::remove_alias(remove) {
                        eprintln!("{e}");
                    }
            }
            if cmd.list {
                if let Err(e) = list::list_aliases() {
                    eprintln!("{e}");
                }
            }
        },
        Some(Commands::CreateProject(cmd)) => {
            if let Err(e) = create_project::create_project(
                cmd.lang,
                cmd.name,
                cmd.alias,
                cmd.path,
            ) {
                eprintln!("{e}");
            }
        },
        None => { Cli::command().print_help().unwrap(); std::process::exit(1); }
    }

    Ok(())
}