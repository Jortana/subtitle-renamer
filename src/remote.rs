use ssh2::Session;
use std::io::{self, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

pub struct SshClient {
    session: Session,
    host: String,
    username: String,
}

impl SshClient {
    pub fn new(
        host: &str,
        username: &str,
        password: Option<&str>,
        key_path: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tcp = TcpStream::connect(host)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        if let Some(key_path) = key_path {
            session.userauth_pubkey_file(username, None, Path::new(key_path), None)?;
        } else if let Some(password) = password {
            session.userauth_password(username, password)?;
        } else {
            session.userauth_agent(username)?;
        }

        if !session.authenticated() {
            return Err("SSH认证失败".into());
        }

        Ok(SshClient {
            session,
            host: host.to_string(),
            username: username.to_string(),
        })
    }

    fn is_video_extension(ext: &str) -> bool {
        matches!(ext, "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm")
    }

    fn is_subtitle_extension(ext: &str) -> bool {
        matches!(ext, "srt" | "ass" | "ssa" | "sub" | "vtt")
    }

    pub fn scan_directory(
        &self,
        directory: &str,
    ) -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
        let sftp = self.session.sftp()?;
        let dir_path = Path::new(directory);

        // 读取远程目录内容
        let entries = sftp.readdir(dir_path)?;

        let mut videos: Vec<PathBuf> = Vec::new();
        let mut subtitles: Vec<PathBuf> = Vec::new();

        for (path, _stat) in entries {
            // 只处理文件（readdir可能返回目录与文件的混合，简单用扩展名来判断类型）
            if let Some(ext) = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
            {
                if Self::is_video_extension(&ext) {
                    videos.push(path);
                } else if Self::is_subtitle_extension(&ext) {
                    subtitles.push(path);
                }
            }
        }

        // 排序，保证匹配的一致性
        videos.sort();
        subtitles.sort();

        Ok((
            subtitles
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            videos
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
        ))
    }

    pub fn rename_subtitles(
        &self,
        directory: &str,
        dry_run: bool,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let (subtitles, videos) = self.scan_directory(directory)?;

        if subtitles.len() != videos.len() {
            return Err(format!(
                "字幕文件数量 ({}) 与视频文件数量 ({}) 不匹配",
                subtitles.len(),
                videos.len()
            )
            .into());
        }

        let sftp = self.session.sftp()?;
        let mut operations = Vec::new();

        for (subtitle, video) in subtitles.iter().zip(videos.iter()) {
            let video_stem = Path::new(video)
                .file_stem()
                .ok_or("无法获取视频文件名")?
                .to_string_lossy();
            let subtitle_ext = Path::new(subtitle)
                .extension()
                .ok_or("无法获取字幕扩展名")?
                .to_string_lossy();

            let new_name = format!("{}.{}", video_stem, subtitle_ext);

            let old_path = Path::new(subtitle).to_path_buf();
            let new_path = old_path
                .parent()
                .unwrap_or(Path::new(directory))
                .join(new_name);

            operations.push((
                old_path.to_string_lossy().to_string(),
                new_path.to_string_lossy().to_string(),
            ));

            if !dry_run {
                sftp.rename(&old_path, &new_path, None)?;
            }
        }

        Ok(operations)
    }

    pub fn interactive_mode(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 SSH远程字幕重命名客户端");
        println!("连接到服务器: {}@{}", self.username, self.host);

        loop {
            println!("\n请选择操作:");
            println!("1. 扫描目录");
            println!("2. 重命名字幕文件");
            println!("3. 模拟重命名");
            println!("4. 退出");
            print!("请输入选项 (1-4): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();

            match choice {
                "1" => {
                    print!("请输入远程目录路径: ");
                    io::stdout().flush()?;
                    let mut dir_input = String::new();
                    io::stdin().read_line(&mut dir_input)?;
                    let directory = dir_input.trim();

                    match self.scan_directory(directory) {
                        Ok((subtitles, videos)) => {
                            println!("✅ 扫描完成");
                            println!("📁 字幕文件 ({}):", subtitles.len());
                            for subtitle in &subtitles {
                                println!("   {}", subtitle);
                            }
                            println!("🎬 视频文件 ({}):", videos.len());
                            for video in &videos {
                                println!("   {}", video);
                            }
                        }
                        Err(e) => println!("❌ 扫描失败: {}", e),
                    }
                }
                "2" => {
                    print!("请输入远程目录路径: ");
                    io::stdout().flush()?;
                    let mut dir_input = String::new();
                    io::stdin().read_line(&mut dir_input)?;
                    let directory = dir_input.trim();

                    match self.rename_subtitles(directory, false) {
                        Ok(operations) => {
                            println!("✅ 重命名完成");
                            for (from, to) in &operations {
                                println!("   {} -> {}", from, to);
                            }
                        }
                        Err(e) => println!("❌ 重命名失败: {}", e),
                    }
                }
                "3" => {
                    print!("请输入远程目录路径: ");
                    io::stdout().flush()?;
                    let mut dir_input = String::new();
                    io::stdin().read_line(&mut dir_input)?;
                    let directory = dir_input.trim();

                    match self.rename_subtitles(directory, true) {
                        Ok(operations) => {
                            println!("🔍 模拟重命名结果:");
                            for (from, to) in &operations {
                                println!("   {} -> {}", from, to);
                            }
                        }
                        Err(e) => println!("❌ 模拟失败: {}", e),
                    }
                }
                "4" => {
                    println!("👋 再见!");
                    break;
                }
                _ => println!("❌ 无效选项，请重新选择"),
            }
        }

        Ok(())
    }
}
