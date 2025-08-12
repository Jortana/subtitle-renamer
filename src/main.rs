mod cli;
mod config;
mod local;
mod matcher;
mod remote;

use cli::parse_args;
use local::rename_subtitles;

fn main() {
    let args = parse_args();
    let dir = &args.dir;
    let dry_run = args.dry_run;

    let (subtitles, videos) = local::scan_directory(dir);
    rename_subtitles(dir, subtitles, videos, dry_run);
}
