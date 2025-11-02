use crate::handlers::errors::CliErr;
use crate::utils::setup::*;
use crossterm::style::Stylize;

pub fn setup(verbose: bool) -> Result<(), CliErr> {
    if verbose {
        let home = if cfg!(windows) {
            "%USERPROFILE%/"
        } else {
            "~/"
        };
        create_cdputils::create_cdputils()?;
        println!("{}\t{} {}{}", "|---".blue(), "Created".green(), home.green(), ".cdputils".blue());
        
        create_cdpaliases::create_cdpaliases()?;
        println!("{}\t{} {}{}{}", "|---".blue(), "Created".green(), home.green(), ".cdputils/".green(), "cdpaliases.txt".blue());

        create_cdp_cmdaliases::create_cdp_cmdaliases()?;
        println!("{}\t{} {}{}{}", "|---".blue(), "Created".green(), home.green(), ".cdputils/".green(), "cdp_cmdaliases.txt".blue());
        
        create_projects::create_projects()?;
        println!("{}\t{} {}{}{}", "|---".blue(), "Created".green(), home.green(), ".cdputils/".green(), "projects".blue());
    } else {
        create_cdputils::create_cdputils()?;
        create_cdpaliases::create_cdpaliases()?;
        create_cdp_cmdaliases::create_cdp_cmdaliases()?;
        create_projects::create_projects()?;
    }

    println!("{}", "Setup done successfully!".green());
    Ok(())
}