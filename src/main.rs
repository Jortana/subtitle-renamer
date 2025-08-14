mod cli;
mod config;
mod local;
mod matcher;
mod remote;

use cli::parse_args;
use local::rename_subtitles;

fn main() {
    let args = parse_args();

    if args.ssh {
        // SSH远程模式
        run_ssh_mode(args).unwrap_or_else(|e| {
            eprintln!("❌ SSH模式错误: {}", e);
            std::process::exit(1);
        });
    } else {
        // 本地模式
        let dir = &args.dir;
        let dry_run = args.dry_run;

        let (subtitles, videos) = local::scan_directory(dir);
        rename_subtitles(dir, subtitles, videos, dry_run);
    }
}

fn run_ssh_mode(args: cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
    let username = args
        .ssh_user
        .ok_or("SSH用户名是必需的，请使用 --ssh-user 参数")?;
    let host = format!("{}:{}", args.ssh_host, args.ssh_port);

    println!("🔗 正在连接到SSH服务器: {}@{}", username, host);

    // 创建SSH客户端
    let client = remote::SshClient::new(
        &host,
        &username,
        args.ssh_password.as_deref(),
        args.ssh_key.as_deref(),
    )?;

    println!("✅ SSH连接成功!");

    if let Some(remote_dir) = args.remote_dir {
        // 非交互模式，直接执行操作
        if args.dry_run {
            println!("🔍 模拟重命名远程目录: {}", remote_dir);
            match client.rename_subtitles(&remote_dir, true) {
                Ok(operations) => {
                    println!("模拟重命名结果:");
                    for (from, to) in &operations {
                        println!("   {} -> {}", from, to);
                    }
                }
                Err(e) => {
                    eprintln!("❌ 模拟失败: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            println!("🔄 重命名远程目录: {}", remote_dir);
            match client.rename_subtitles(&remote_dir, false) {
                Ok(operations) => {
                    println!("✅ 重命名完成:");
                    for (from, to) in &operations {
                        println!("   {} -> {}", from, to);
                    }
                }
                Err(e) => {
                    eprintln!("❌ 重命名失败: {}", e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        // 交互模式
        client.interactive_mode()?;
    }

    Ok(())
}
