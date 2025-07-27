mod handlers;
mod types;
mod utils;

use clap::CommandFactory;
use types::*;
#[allow(unused_imports)]
use handlers::{
    general::*, aliases::*, create_project::*,
};

pub fn cli_run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut path: String;
    if cfg!(windows) {
        path = format!("C:\\{}", match args.command {
            Some(Commands::General(ref cmd)) => cmd.path.clone(),
            _ => "" .to_owned()
        });
    } else {
        path = format!("{}", match args.command {
            Some(Commands::General(ref cmd)) => cmd.path.clone(),
            _ => "" .to_owned()
        });
    }

    match args.command {
        Some(Commands::General(cmd)) => {
            if cmd.current_user {
                path = format!("{}", change_dir_current::change_to_current());
            }
            if cmd.alias {
                path = see_alias::see_alias(cmd.path.clone()).expect("This alias does not exists");
            }
            if cmd.ls {
                ls::ls(path.clone());
            }
            if cmd.vsc {
                vsc::open_editor("vsc", path.clone());
            }
        },
        Some(Commands::Aliases(cmd)) => {
            aliases::aliases(cmd.path, cmd.alias)?;
        },
        Some(Commands::CreateProject(cmd)) => {
            create_project::create_project(
                cmd.lang,
                cmd.name,
                cmd.alias,
            )?;
        },
        None => { Cli::command().print_help().unwrap(); std::process::exit(1); }
    }

    Ok(())
}