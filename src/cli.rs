use clap::Parser;

#[derive(Parser)]
#[command(name = "subtitle_renamer")]
#[command(version = "1.0")]
#[command(about = "Renames subtitle files to match episode names")]
pub struct Cli {
    // Directory to scan for subtitle files
    #[arg(short = 'd', long, value_name = "DIRECTORY", default_value = ".")]
    pub dir: String,

    // Show the rename actions without actually renaming files
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    // Connect via SSH
    #[arg(short = 's', long)]
    pub ssh: bool,

    // SSH host
    #[arg(long, default_value = "localhost")]
    pub ssh_host: String,

    // SSH port
    #[arg(long, default_value = "22")]
    pub ssh_port: u16,

    // SSH username
    #[arg(long)]
    pub ssh_user: Option<String>,

    // SSH password (not recommended, use key authentication)
    #[arg(long)]
    pub ssh_password: Option<String>,

    // SSH private key path
    #[arg(long)]
    pub ssh_key: Option<String>,

    // Remote directory for SSH mode
    #[arg(long)]
    pub remote_dir: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
