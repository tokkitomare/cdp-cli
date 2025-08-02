pub use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(name="cdp", version)]
#[command(
    about="CDP - This CLI helps you quickly navigate to a directory and perform useful operations on it.",
    long_about="
CDP stands for CD Program, where `cd` refers to the directory-changing command in Unix-like shells and PowerShell.

This CLI helps you quickly navigate to a directory and perform useful operations on it."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Commands to make simple operations on directories
    #[command(alias="g")]
    General(GeneralCommand),

    /// Create an alias for a path so you don't have to type it out every time
    #[command(alias="alias")]
    Aliases(AliasesCommand),

    /// Create a project directory with the supported langs.
    /// It doesn't yet work on Unix
    #[command(alias="cp")]
    CreateProject(CPCommand),
}

// ------------------------------------------- Commands
#[derive(Args, Debug)]
pub struct GeneralCommand {
    /// Path where CDP will perform the operation
    /// 
    /// If no flag specifying where the program must search, it will search from the current folder.
    pub path: String,

    /// Lists the folders and files from the path
    #[arg(long)]
    pub ls: bool,

    /// Open Visual Studio Code from the path
    #[arg(long, hide=true, conflicts_with="editor")]
    pub vsc: bool,

    /// Open the editor from the path
    #[arg(long, short='E')]
    pub editor: Option<Editors>,

    #[cfg_attr(windows, doc = "Make the command start from `USER` ($HOME)")]
    #[cfg_attr(unix, doc = "Make the command start from `HOME` (~/)")]
    #[arg(short='C', long, conflicts_with="alias", hide = true)]
    pub current_user: bool,

    /// Enable the `aliases mode` and let you type path aliases instead of whole paths (to create aliases: `cdp aliases --help`)
    #[arg(long, alias="al", hide = true)]
    pub alias: bool,
}

#[derive(Args, Debug)]
pub struct AliasesCommand {
    /// Path that you want to create an alias
    pub path: String,
    /// Alias of the path
    /// 
    #[cfg_attr(windows, doc = "It will be stored on `cdpaliases.txt` file (file path: `%USERPROFILE%\\.cdputils\\cdpaliases.txt`)")]
    #[cfg_attr(unix, doc = "It will be stored on `cdpaliases.txt` file (file path: `~/.cdputils/cdpaliases.txt`)")]
    pub alias: String,
}

#[derive(Args, Debug)]
pub struct CPCommand {
    /// Name of the project. 
    #[cfg_attr(windows, doc = "It will be in `%USERPROFILE%/.cdputils/projects`")]
    #[cfg_attr(unix, doc = "It will be in `~/.cdputils/projects`")]
    pub name: String,
    /// Project lang
    pub lang: Langs,

    /// Create an alias for the path
    #[arg(long, alias="al")]
    pub alias: Option<String>,
}


// ------------------------------------------- ValueEnums
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Editors {
    #[clap(alias="vscode")]
    Vsc,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Langs {
    #[clap(alias="rust")]
    Rs,
    #[clap(alias="javascript")]
    Js,
    #[clap(alias="typescript")]
    Ts,
    #[clap(alias="python")]
    Py,
    #[clap(alias="clang")]
    C,
    #[clap(alias="c++")]
    Cpp,
}