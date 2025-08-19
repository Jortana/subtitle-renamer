# Subtitle Renamer

一个极简的字幕文件重命名工具，只做一件事：把字幕文件重命名为对应的视频文件名。

## 功能

- 扫描指定目录中的视频和字幕文件
- 按文件名顺序自动匹配
- 支持模拟模式（不实际重命名）
- 零依赖，纯Rust标准库
- 支持命令行参数，便于打包分发

## 安装

### 从源码编译
```bash
git clone <repository-url>
cd subtitle_renamer
cargo build --release
```

### 使用可执行文件
编译完成后，可执行文件位于 `target/release/subtitle_renamer`，可以复制到系统PATH中：

```bash
# macOS/Linux
sudo cp target/release/subtitle_renamer /usr/local/bin/

# 或者直接使用
./target/release/subtitle_renamer --help
```

## 用法

```bash
# 显示帮助信息
subtitle_renamer --help

# 显示版本信息
subtitle_renamer --version

# 重命名当前目录的字幕文件
subtitle_renamer

# 重命名指定目录的字幕文件
subtitle_renamer /path/to/videos

# 模拟模式（只显示会做什么，不实际执行）
subtitle_renamer --dry-run

# 模拟模式 + 指定目录
subtitle_renamer /path/to/videos --dry-run

# 使用短参数
subtitle_renamer -n /path/to/videos
```

## 命令行参数

| 参数 | 短参数 | 描述 | 默认值 |
|------|--------|------|--------|
| `--help` | `-h` | 显示帮助信息 | - |
| `--version` | `-v` | 显示版本信息 | - |
| `--dry-run` | `-n` | 模拟模式，不实际重命名 | `false` |
| 目录 | - | 要扫描的目录 | 当前目录 |

## 工作原理

1. 扫描目录中的视频文件（mp4, mkv, avi等）
2. 扫描目录中的字幕文件（srt, ass, vtt等）
3. 按文件名排序
4. 将字幕文件重命名为对应的视频文件名

## 支持的文件格式

**视频**: mp4, mkv, avi, mov, wmv, flv, webm, 3gp, m4v, hevc
**字幕**: srt, ass, ssa, vtt, sub, idx, dfxp, ttml, smi, cpt, mks

## 使用场景

### 场景1: 本地批量重命名
```bash
# 在视频目录中执行重命名
cd /path/to/videos
subtitle_renamer -n  # 先预览
subtitle_renamer     # 执行重命名
```

### 场景2: 跨平台使用
```bash
# 复制可执行文件到其他机器
scp subtitle_renamer user@server:/usr/local/bin/

# 在远程服务器上使用
ssh user@server "subtitle_renamer /media/videos"
```

### 场景3: 脚本集成
```bash
#!/bin/bash
# 批量处理多个目录
for dir in /videos/season1 /videos/season2 /videos/season3; do
    echo "处理目录: $dir"
    subtitle_renamer "$dir"
done
```

## 构建

```bash
# 开发版本
cargo build

# 发布版本（优化后的可执行文件）
cargo build --release

# 检查代码
cargo check

# 运行测试
cargo test
```

## 哲学

这个工具遵循Unix哲学：**做一件事，做好它**。

- 没有配置文件
- 没有SSH远程功能
- 没有进度条
- 没有复杂的错误处理
- 只有核心功能：重命名字幕文件
- 支持命令行参数，便于分发和使用

如果你需要更多功能，这个工具可能不适合你。如果你只需要重命名字幕文件，这就是你需要的。
