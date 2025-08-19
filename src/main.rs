use std::env;
use std::fs;
use std::path::Path;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut dir = ".".to_string();
    let mut dry_run = false;
    let mut show_help = false;

    // 解析命令行参数
    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => show_help = true,
            "-n" | "--dry-run" => dry_run = true,
            "-v" | "--version" => {
                println!("{} v{}", NAME, VERSION);
                process::exit(0);
            }
            arg if arg.starts_with('-') => {
                eprintln!("❌ 未知参数: {}", arg);
                print_usage();
                process::exit(1);
            }
            arg => dir = arg.to_string(),
        }
    }

    if show_help {
        print_usage();
        process::exit(0);
    }

    println!("扫描目录: {}", dir);

    match scan_and_rename(&dir, dry_run) {
        Ok(count) => {
            let mode = if dry_run { "模拟" } else { "执行" };
            println!("✅ {}重命名完成，共处理 {} 个文件", mode, count);
        }
        Err(e) => {
            eprintln!("❌ 错误: {}", e);
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("字幕文件重命名工具");
    println!();
    println!("用法:");
    println!("  {} [选项] [目录]", NAME);
    println!();
    println!("选项:");
    println!("  -h, --help     显示帮助信息");
    println!("  -v, --version  显示版本信息");
    println!("  -n, --dry-run  模拟模式，不实际重命名");
    println!();
    println!("参数:");
    println!("  目录            要扫描的目录 (默认: 当前目录)");
    println!();
    println!("示例:");
    println!("  {}                    # 重命名当前目录", NAME);
    println!("  {} /path/to/videos    # 重命名指定目录", NAME);
    println!("  {} -n                 # 模拟模式", NAME);
    println!("  {} /videos --dry-run  # 指定目录+模拟模式", NAME);
}

fn scan_and_rename(dir: &str, dry_run: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let mut videos = Vec::new();
    let mut subtitles = Vec::new();

    // 扫描文件
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();

            if is_video(&ext) {
                videos.push(path);
            } else if is_subtitle(&ext) {
                subtitles.push(path);
            }
        }
    }

    if videos.is_empty() || subtitles.is_empty() {
        return Ok(0);
    }

    // 排序并匹配
    videos.sort();
    subtitles.sort();

    let min_len = videos.len().min(subtitles.len());
    let mut count = 0;

    for i in 0..min_len {
        let video = &videos[i];
        let subtitle = &subtitles[i];

        // 生成新名字，保留或添加语言信息（不强行添加）
        let new_name = generate_new_name(video, subtitle);
        let new_path = subtitle.parent().unwrap().join(&new_name);

        if dry_run {
            println!(
                "模拟重命名: {} -> {}",
                subtitle.file_name().unwrap().to_string_lossy(),
                new_name
            );
        } else {
            fs::rename(subtitle, &new_path)?;
            println!(
                "重命名: {} -> {}",
                subtitle.file_name().unwrap().to_string_lossy(),
                new_name
            );
        }
        count += 1;
    }

    Ok(count)
}

fn generate_new_name(video: &Path, subtitle: &Path) -> String {
    let video_name = video.file_stem().unwrap().to_string_lossy();
    let subtitle_ext = subtitle.extension().unwrap().to_string_lossy();

    // 文件名已有语言信息则保留
    if let Some(language) = extract_language_from_filename(&subtitle.to_string_lossy()) {
        return format!("{}.{}.{}", video_name, language, subtitle_ext);
    }

    // 内容检测（仅在高置信度时返回语言）
    if let Some(detected_lang) = detect_subtitle_language(subtitle) {
        return format!("{}.{}.{}", video_name, detected_lang, subtitle_ext);
    }

    // 未检测到则不加语言后缀
    format!("{}.{}", video_name, subtitle_ext)
}

fn extract_language_from_filename(filename: &str) -> Option<String> {
    // 匹配常见的语言标识模式
    let language_patterns = [
        r"\.(zh|zh-cn|zh-hans|zh-hant|en|eng|ja|jp|ko|kr|fr|de|es|it|ru)\.", // 双点分隔
        r"\.(zh|zh-cn|zh-hans|zh-hant|en|eng|ja|jp|ko|kr|fr|de|es|it|ru)$",  // 结尾
        r"_(zh|zh-cn|zh-hans|zh-hant|en|eng|ja|jp|ko|kr|fr|de|es|it|ru)\.",  // 下划线分隔
    ];

    for pattern in &language_patterns {
        if let Ok(regex) = regex::Regex::new(pattern) {
            if let Some(captures) = regex.captures(filename) {
                if let Some(lang) = captures.get(1) {
                    return Some(lang.as_str().to_string());
                }
            }
        }
    }
    None
}

fn detect_subtitle_language(subtitle_path: &Path) -> Option<String> {
    // 尝试从文件内容检测语言
    if let Ok(content) = fs::read_to_string(subtitle_path) {
        return detect_language_from_content(&content);
    }
    None
}

fn detect_language_from_content(content: &str) -> Option<String> {
    // 过滤掉时间轴、纯数字行，降低噪音
    let filtered: String = content
        .lines()
        .filter(|line| {
            let l = line.trim();
            if l.is_empty() {
                return false;
            }
            if l.contains("-->") {
                return false;
            }
            // SRT 序号行
            if l.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
            true
        })
        .collect::<Vec<&str>>()
        .join("\n");

    if filtered.is_empty() {
        return None;
    }

    let mut chinese_chars: usize = 0;
    let mut latin_letters: usize = 0;
    let mut total_letters: usize = 0;

    for ch in filtered.chars() {
        if ch.is_ascii_alphabetic() {
            latin_letters += 1;
            total_letters += 1;
        } else if matches!(ch, '\u{4e00}'..='\u{9fff}' | '\u{3400}'..='\u{4dbf}' | '\u{20000}'..='\u{2a6df}')
        {
            chinese_chars += 1;
            total_letters += 1;
        }
    }

    if total_letters == 0 {
        return None;
    }

    let zh_ratio = chinese_chars as f64 / total_letters as f64;
    let en_ratio = latin_letters as f64 / total_letters as f64;

    // 阈值：中文 > 10% 视为中文；否则若英文 > 30% 视为英文；否则不确定
    if zh_ratio > 0.10 {
        Some("zh".to_string())
    } else if en_ratio > 0.30 {
        Some("en".to_string())
    } else {
        None
    }
}

fn is_video(ext: &str) -> bool {
    matches!(
        ext,
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "3gp" | "m4v" | "hevc"
    )
}

fn is_subtitle(ext: &str) -> bool {
    matches!(
        ext,
        "srt" | "ass" | "ssa" | "vtt" | "sub" | "idx" | "dfxp" | "ttml" | "smi" | "cpt" | "mks"
    )
}
