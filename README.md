# Subtitle Renamer

一个字幕重命名工具，可以自动将字幕文件重命名为与视频文件匹配的名称。

## 功能特性

- 🎬 自动识别视频文件（支持 mp4、avi、mkv 等常见格式）
- 📝 自动识别字幕文件（支持 srt、vtt、ass 等常见格式）
- 🔄 智能匹配：按字母顺序排序后一一对应重命名
- 🧪 支持干运行模式，预览重命名操作而不实际执行
- 🚀 跨平台支持（Windows、macOS、Linux）
- 💻 命令行界面，简单易用

## 安装方法

### 从源码编译

1. 确保已安装 Rust（推荐使用 rustup）
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 克隆项目
```bash
git clone https://github.com/yourusername/subtitle_renamer.git
cd subtitle_renamer
```

3. 编译项目
```bash
cargo build --release
```

4. 安装到系统（可选）
```bash
cargo install --path .
```

### 从预编译版本下载

访问 [Releases](https://github.com/yourusername/subtitle_renamer/releases) 页面下载适合你系统的预编译版本。

## 使用方法

### 基本用法

```bash
# 在当前目录重命名字幕文件
subtitle_renamer

# 指定目录
subtitle_renamer -d /path/to/videos

# 使用长参数
subtitle_renamer --dir /path/to/videos
```

### 干运行模式

在正式重命名之前，建议先使用干运行模式预览操作：

```bash
# 预览重命名操作，不实际执行
subtitle_renamer -n

# 或使用长参数
subtitle_renamer --dry-run
```

### 参数说明

- `-d, --dir <DIRECTORY>`: 指定要扫描的目录（默认为当前目录）
- `-n, --dry-run`: 干运行模式，只显示重命名操作而不执行
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

## 使用示例

### 示例 1：基本重命名

假设你有以下文件：
```
Season1/
├── episode01.mp4
├── episode02.mp4
├── episode03.mp4
├── subtitle01.srt
├── subtitle02.srt
└── subtitle03.srt
```

运行命令：
```bash
subtitle_renamer -d Season1
```

结果：
```
Season1/
├── episode01.mp4
├── episode02.mp4
├── episode03.mp4
├── episode01.srt    # 重命名自 subtitle01.srt
├── episode02.srt    # 重命名自 subtitle02.srt
└── episode03.srt    # 重命名自 subtitle03.srt
```

### 示例 2：干运行预览

```bash
subtitle_renamer -d Season1 -n
```

输出：
```
Dry Run: Season1/subtitle01.srt -> Season1/episode01.srt
Dry Run: Season1/subtitle02.srt -> Season1/episode02.srt
Dry Run: Season1/subtitle03.srt -> Season1/episode03.srt
```

### 示例 3：处理不同字幕格式

支持多种字幕格式：
```
Season1/
├── episode01.mp4
├── episode02.mp4
├── subtitle01.srt
├── subtitle02.vtt
└── subtitle03.sub
```

重命名后：
```
Season1/
├── episode01.mp4
├── episode02.mp4
├── episode01.srt
├── episode02.vtt
└── episode03.sub
```

## 工作原理

1. **扫描目录**：扫描指定目录中的所有文件
2. **文件分类**：将文件分为视频文件和字幕文件两类
3. **排序匹配**：按文件名字母顺序排序，然后一一对应
4. **重命名**：将字幕文件重命名为对应的视频文件名（保留原扩展名）

## 注意事项

- 重命名操作基于文件名的字母顺序，确保视频和字幕文件按相同顺序排列
- 建议在重命名前备份重要文件
- 使用 `--dry-run` 参数预览操作结果
- 支持的字幕格式：srt、vtt、sub
- 支持的视频格式：mp4、avi、mkv

## 贡献

欢迎提交 Issue 和 Pull Request！
