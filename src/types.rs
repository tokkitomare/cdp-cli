pub use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(name="cdp", version)]
#[command(
    about="CDP - This CLI helps you quickly navigate to a directory and perform useful operations on it.",
    long_about="
CDP stands for CD Program, where 'cd' refers to the directory-changing command in Unix-like shells and PowerShell.

This CLI helps you quickly navigate to a directory and perform useful operations on it."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sets up all necessary environment for CDP to work properly (RECOMMENDED)
    Setup(SetupCommand),

    /// Perform basic file and folder operations on a given path
    #[command(alias="g")]
    General(GeneralCommand),

    /// Create and manage aliases for paths
    #[command(alias="alias")]
    Aliases(AliasesCommand),

    /// Create aliases for your commands
    #[command(alias="cmdal", alias="cmd-alias")]
    CommandAliases(CmdAliasesCommand),

    /// Create a project directory using a supported language
    #[command(alias="cp")]
    CreateProject(CPCommand),
}

// ------------------------------------------- Commands
#[derive(Args, Debug)]
pub struct SetupCommand {
    /// Enables verbose debug output for the setup command
    #[arg(long, short='v')]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct GeneralCommand {
    /// Path where CDP will perform the operation
    pub path: String,

    /// Lists the folders and files from the path
    #[arg(long)]
    pub ls: bool,

    /// Open the editor from the path
    #[arg(long, short='E')]
    pub editor: Option<Editors>,

    /// Open Visual Studio Code from the path
    #[arg(long, hide=true, conflicts_with="editor")]
    pub vsc: bool,

    #[cfg_attr(windows, doc = "Make the command start from 'USER' ($HOME)")]
    #[cfg_attr(unix, doc = "Make the command start from 'HOME' (~/)")]
    #[arg(short='C', long, conflicts_with="alias", hide = true)]
    pub current_user: bool,

    /// Enable the 'aliases mode' and let you type path aliases instead of whole paths (to create aliases: 'cdp aliases --help')
    #[arg(long, alias="al", hide = true)]
    pub alias: bool,
}

#[derive(Args, Debug)]
pub struct AliasesCommand {
    /// Path that you want to create an alias
    #[arg(value_name="PATH", required_unless_present_any=["list", "edit", "remove"])]
    pub path: Option<String>,
    /// Alias of the path.
    #[cfg_attr(windows, doc = "It will be stored on 'cdpaliases.txt' file (file path: '%USERPROFILE%\\.cdputils\\cdpaliases.txt')")]
    #[cfg_attr(unix, doc = "It will be stored on 'cdpaliases.txt' file (file path: '~/.cdputils/cdpaliases.txt')")]
    #[arg(value_name="ALIAS", required_unless_present_any=["list", "edit", "remove"])]
    pub alias: Option<String>,

    /// Lists all the aliases in
    #[cfg_attr(windows, doc = r"'%USERPROFILE%\.cdputils\cdpaliases.txt'")]
    #[cfg_attr(unix, doc = "'~/.cdputils/cdpaliases.txt'")]
    #[arg(long, short='l', alias="ls", conflicts_with="alias")]
    pub list: bool,

    /// Remove an alias from
    #[cfg_attr(windows, doc = r"'%USERPROFILE%\.cdputils\cdpaliases.txt'")]
    #[cfg_attr(unix, doc = "'~/.cdputils/cdpaliases.txt'")]
    #[arg(long, short='r', alias="rm", conflicts_with="edit")]
    pub remove: Option<String>,

    /// Edit an alias from
    #[cfg_attr(windows, doc = r"'%USERPROFILE%\.cdputils\cdpaliases.txt'")]
    #[cfg_attr(unix, doc = "'~/.cdputils/cdpaliases.txt'")]
    #[arg(long, short='e')]
    pub edit: Option<String>,
}

#[derive(Args, Debug)]
pub struct CmdAliasesCommand {
    /// Command that you want to create an alias.
    #[cfg_attr(windows, doc = "It will be stored on 'cdp_cmdaliases.txt' file (file path: '%USERPROFILE%\\.cdputils\\cdp_cmdaliases.txt')")]
    #[cfg_attr(unix, doc = "It will be stored on 'cdp_cmdaliases.txt' file (file path: '~/.cdputils/cdp_cmdaliases.txt')")]
    #[arg(value_name="COMMAND", required_unless_present_any=["list", "edit", "remove", "execute"])]
    pub cmd: Option<String>,
    /// Give a name for the command.
    #[cfg_attr(windows, doc = "It will be stored on 'cdp_cmdaliases.txt' file (file path: '%USERPROFILE%\\.cdputils\\cdp_cmdaliases.txt')")]
    #[cfg_attr(unix, doc = "It will be stored on 'cdp_cmdaliases.txt' file (file path: '~/.cdputils/cdp_cmdaliases.txt')")]
    #[arg(value_name="ALIAS", required_unless_present_any=["list", "edit", "remove", "execute"])]
    pub alias: Option<String>,

    /// The command that you want to execute (must be an alias) [WINDOWS ONLY]
    #[arg(long, short='E', alias="exe", required_unless_present_any=["list", "edit", "remove", "cmd", "alias"])]
    pub execute: Option<String>,

    /// Lists all the aliases in
    #[cfg_attr(windows, doc = r"'%USERPROFILE%\.cdputils\cdp_cmdaliases.txt'")]
    #[cfg_attr(unix, doc = "'~/.cdputils/cdp_cmdaliases.txt'")]
    #[arg(long, short='l', alias="ls", conflicts_with="cmd")]
    pub list: bool,

    /// Remove an alias from
    #[cfg_attr(windows, doc = r"'%USERPROFILE%\.cdputils\cdp_cmdaliases.txt'")]
    #[cfg_attr(unix, doc = "'~/.cdputils/cdp_cmdaliases.txt'")]
    #[arg(long, short='r', alias="rm", conflicts_with="edit")]
    pub remove: Option<String>,
    
    /// Edit an alias from
    #[cfg_attr(windows, doc = r"'%USERPROFILE%\.cdputils\cdp_cmdaliases.txt'")]
    #[cfg_attr(unix, doc = "'~/.cdputils/cdp_cmdaliases.txt'")]
    #[arg(long, short='e')]
    pub edit: Option<String>,
}

#[derive(Args, Debug)]
pub struct CPCommand {
    /// Project name. 
    #[cfg_attr(windows, doc = "Will be created in '%USERPROFILE%/.cdputils/projects'")]
    #[cfg_attr(unix, doc = "Will be created in '~/.cdputils/projects'")]
    /// by default. Use the '--path' flag to change it.
    pub name: String,
    /// Project lang
    pub lang: Langs,

    /// Create an alias for the path
    #[arg(long, alias="al")]
    pub alias: Option<String>,
    /// Create the project in a custom path (some paths require admin privileges
    #[cfg_attr(windows, doc = r"such as 'C:\Program Files')")]
    #[cfg_attr(unix, doc = "such as root directories (/bin, /usr, /dev, etc...))")]
    #[arg(long, short='p')]
    pub path: Option<String>,
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
    #[clap(alias="javascript", alias="nodejs")]
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