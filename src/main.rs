mod cli;
mod local;

use cli::parse_args;
use local::scan_directory;

fn main() {
    // 解析命令行参数
    let args = parse_args();

    // 获取扫描的目录路径
    let dir = &args.dir;
    let dry_run = args.dry_run;

    // 输出扫描目录
    println!("Scanning directory: {}", dir);

    // 扫描目录并获取字幕文件
    let subtitles = scan_directory(dir);

    if dry_run {
        // 如果是 dry-run 模式，显示重命名计划
        println!("Dry run enabled. The following subtitles will be renamed:");
        for subtitle in subtitles {
            println!("{}", subtitle.to_string_lossy());
        }
    } else {
        // 否则执行重命名
        println!("Renaming subtitles...");
        // 后续实现重命名功能
    }
}
