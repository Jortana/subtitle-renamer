use std::ffi::OsString;
use std::fs;

pub fn scan_directory(dir: &str) -> Vec<OsString> {
    let mut subtitles = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "srt") {
                subtitles.push(path.file_name().unwrap().to_os_string());
            }
        }
    }
    subtitles
}
