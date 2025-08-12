use clap::Parser;

#[derive(Parser)]
#[command(name = "subtitle_renamer")]
#[command(version = "1.0")]
#[command(about = "Renames subtitle files to match episode names")]
pub struct Cli {
    /// Directory to scan for subtitle files
    #[arg(short = 'd', long, value_name = "DIRECTORY", default_value = ".")]
    pub dir: String,

    /// Show the rename actions without actually renaming files
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
