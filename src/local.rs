use crate::matcher::{is_subtitle, is_video, match_files};
use std::fs;
use std::path::{Path, PathBuf};

pub fn scan_directory(dir: &str) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut subtitles = Vec::new();
    let mut videos = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if is_subtitle(&path) {
                subtitles.push(path);
            } else if is_video(&path) {
                videos.push(path);
            }
        }
    }

    // 按字母顺序排序
    subtitles.sort();
    videos.sort();

    (subtitles, videos)
}

pub fn rename_subtitles(dir: &str, subtitles: Vec<PathBuf>, videos: Vec<PathBuf>, dry_run: bool) {
    let matches = match_files(videos, subtitles);

    for (subtitle_name, new_subtitle_name) in matches {
        let subtitle_path = Path::new(dir).join(&subtitle_name);
        let new_subtitle_path = Path::new(dir).join(&new_subtitle_name);

        if dry_run {
            println!(
                "Dry Run: {} -> {}",
                subtitle_path.display(),
                new_subtitle_path.display()
            );
        } else {
            println!(
                "Renaming: {} -> {}",
                subtitle_path.display(),
                new_subtitle_path.display()
            );
            fs::rename(subtitle_path, new_subtitle_path).unwrap();
        }
    }
}
