use clap::Parser;

/**
 Parse CLI arguments for the olcs CLI.
*/
#[derive(Parser, Debug, Clone)]
#[command(name = "olcs")]
#[command(about = "Run common team commands easily")]
pub struct Cli {

    /** Override arguments (key=value) */
    #[arg(long = "arg")]
    pub args: Vec<String>,

    /** Show config and exit */
    #[arg(long)]
    pub show_active_params: bool,

    /** Clear stored data (last-used values) and exit */
    #[arg(long)]
    pub clear_stored: bool,

    /** Change active param group (params.yaml) */
    #[arg(short, long)]
    pub switch_param_group: bool,

    /** Run in interactive mode */
    #[arg(short, long)]
    pub interactive: bool,

    /** Run a specific command by name */
    pub command: Option<String>,

    /** Show all available commands from commands.yaml */
    #[arg(short, long)]
    pub list_cmds: bool,
}
