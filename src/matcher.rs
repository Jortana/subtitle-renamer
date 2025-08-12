use std::path::{Path, PathBuf};

const VIDEO_EXTENSIONS: [&str; 10] = [
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "3gp", "m4v", "hevc",
];
const SUBTITLE_EXTENSIONS: [&str; 12] = [
    "srt", "ass", "ssa", "vtt", "sub", "idx", "dfxp", "ttml", "eia-608", "smi", "cpt", "mks",
];

// 判断文件是否为视频文件
pub fn is_video(file: &Path) -> bool {
    file.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext_lower = ext.to_ascii_lowercase();
            VIDEO_EXTENSIONS.contains(&ext_lower.as_str())
        })
        .unwrap_or(false)
}

// 判断文件是否为字幕文件
pub fn is_subtitle(file: &Path) -> bool {
    file.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext_lower = ext.to_ascii_lowercase();
            SUBTITLE_EXTENSIONS.contains(&ext_lower.as_str())
        })
        .unwrap_or(false)
}

// 按字母顺序排序文件列表
pub fn sort_files(files: &mut Vec<PathBuf>) {
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()))
}

// 匹配视频文件和字幕文件
pub fn match_files(videos: Vec<PathBuf>, subtitles: Vec<PathBuf>) -> Vec<(String, String)> {
    let mut matches = Vec::new();

    // 按文件名排序
    let mut sorted_videos = videos.clone();
    let mut sorted_subtitles = subtitles.clone();
    sort_files(&mut sorted_videos);
    sort_files(&mut sorted_subtitles);

    let min_len = sorted_videos.len().min(sorted_subtitles.len());

    for i in 0..min_len {
        let video = &sorted_videos[i];
        let subtitle = &sorted_subtitles[i];

        let subtitle_name = subtitle.file_name().unwrap().to_str().unwrap();
        let video_base_name = video.file_stem().unwrap().to_str().unwrap();
        // 获取字幕文件的扩展名
        let subtitle_extension = subtitle.extension().unwrap_or_default().to_str().unwrap();
        // 构造新的字幕文件名，保留原扩展名
        let new_subtitle_name = format!("{}.{}", video_base_name, subtitle_extension);

        matches.push((subtitle_name.to_string(), new_subtitle_name));
    }

    matches
}
