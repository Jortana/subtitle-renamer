# 字幕重命名工具 (Subtitle Renamer)

一个用Rust编写的智能字幕文件重命名工具，支持本地和SSH远程操作模式。

## 功能特性

- 🎯 **智能匹配**: 自动匹配字幕文件和视频文件
- 🖥️ **本地模式**: 直接在本地机器上执行重命名操作
- 🔗 **SSH远程模式**: 通过SSH连接远程执行操作
- 💻 **交互式界面**: 友好的命令行交互界面
- 🔍 **模拟模式**: 预览重命名操作而不实际执行
- 🔐 **安全认证**: 支持密码和SSH密钥认证

## 安装

### 从源码编译

```bash
git clone <repository-url>
cd subtitle_renamer
cargo build --release
```

### 运行

```bash
# 将编译好的二进制文件复制到系统PATH
sudo cp target/release/subtitle_renamer /usr/local/bin/
```

## 使用方法

### 1. 本地模式 (默认)

```bash
# 扫描当前目录
subtitle_renamer

# 扫描指定目录
subtitle_renamer -d /path/to/videos

# 模拟模式 (预览操作)
subtitle_renamer -d /path/to/videos -n
```

### 2. SSH远程模式

#### 交互式模式
```bash
# 使用SSH密钥连接（推荐）
subtitle_renamer -s --ssh-user username --ssh-host server.com --ssh-key ~/.ssh/id_rsa

# 使用密码连接
subtitle_renamer -s --ssh-user username --ssh-host server.com --ssh-password your_password

# 指定SSH端口
subtitle_renamer -s --ssh-user username --ssh-host server.com --ssh-port 2222 --ssh-key ~/.ssh/id_rsa
```

#### 非交互式模式
```bash
# 直接重命名远程目录
subtitle_renamer -s --ssh-user username --ssh-host server.com --ssh-key ~/.ssh/id_rsa --remote-dir /path/to/videos

# 模拟重命名远程目录
subtitle_renamer -s --ssh-user username --ssh-host server.com --ssh-key ~/.ssh/id_rsa --remote-dir /path/to/videos -n
```

## 使用场景

### 场景1: 本地批量重命名
```bash
# 在视频目录中执行重命名
cd /path/to/videos
subtitle_renamer -n  # 先预览
subtitle_renamer     # 执行重命名
```

### 场景2: 远程服务器管理
```bash
# 连接到远程服务器并交互式操作
subtitle_renamer -s --ssh-user admin --ssh-host nas.local --ssh-key ~/.ssh/id_rsa

# 直接重命名远程目录
subtitle_renamer -s --ssh-user admin --ssh-host nas.local --ssh-key ~/.ssh/id_rsa --remote-dir /media/videos
```

### 场景3: 批量处理多个服务器
```bash
# 处理服务器A
subtitle_renamer -s --ssh-user user --ssh-host server-a.com --remote-dir /videos/season1

# 处理服务器B
subtitle_renamer -s --ssh-user user --ssh-host server-b.com --remote-dir /videos/season2
```

## 命令行参数

| 参数 | 短参数 | 描述 | 默认值 |
|------|--------|------|--------|
| `--dir` | `-d` | 要扫描的目录 | `.` |
| `--dry-run` | `-n` | 模拟模式，不实际重命名 | `false` |
| `--ssh` | `-s` | 启用SSH远程模式 | `false` |
| `--ssh-host` | | SSH服务器地址 | `localhost` |
| `--ssh-port` | | SSH端口 | `22` |
| `--ssh-user` | | SSH用户名 | 必需 |
| `--ssh-password` | | SSH密码 | 可选 |
| `--ssh-key` | | SSH私钥路径 | 可选 |
| `--remote-dir` | | 远程目录路径 | 可选 |

## SSH认证方式

### 1. SSH密钥认证（推荐）
```bash
# 生成SSH密钥对（如果还没有）
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"

# 将公钥复制到远程服务器
ssh-copy-id username@server.com

# 使用密钥连接
subtitle_renamer -s --ssh-user username --ssh-host server.com --ssh-key ~/.ssh/id_rsa
```

### 2. 密码认证
```bash
# 使用密码连接（不推荐，因为密码会出现在命令行历史中）
subtitle_renamer -s --ssh-user username --ssh-host server.com --ssh-password your_password
```

### 3. SSH Agent认证
```bash
# 启动SSH agent并添加密钥
eval $(ssh-agent -s)
ssh-add ~/.ssh/id_rsa

# 连接（会自动使用agent中的密钥）
subtitle_renamer -s --ssh-user username --ssh-host server.com
```

## 支持的文件格式

### 视频文件
- `.mp4`, `.avi`, `.mkv`, `.mov`, `.wmv`, `.flv`, `.webm`

### 字幕文件
- `.srt`, `.ass`, `.ssa`, `.sub`, `.vtt`

## 安全注意事项

1. **SSH密钥安全**: 确保SSH私钥文件权限正确（600）
2. **密码安全**: 避免在命令行中使用密码，优先使用SSH密钥
3. **网络安全**: 确保SSH连接使用加密传输
4. **文件权限**: 确保远程用户有足够的权限访问和修改目标目录

## 故障排除

### 常见问题

1. **SSH连接失败**
   ```bash
   # 测试SSH连接
   ssh username@server.com
   
   # 检查SSH密钥权限
   ls -la ~/.ssh/id_rsa
   chmod 600 ~/.ssh/id_rsa
   ```

2. **权限不足**
   ```bash
   # 检查远程目录权限
   ssh username@server.com "ls -la /path/to/videos"
   ```

3. **文件路径问题**
   ```bash
   # 确保使用绝对路径
   subtitle_renamer -s --ssh-user user --ssh-host server.com --remote-dir /absolute/path/to/videos
   ```

4. **SSH密钥问题**
   ```bash
   # 重新生成SSH密钥
   ssh-keygen -t rsa -b 4096
   
   # 重新复制公钥
   ssh-copy-id username@server.com
   ```

## 开发

### 构建开发版本
```bash
cargo build
```

### 运行测试
```bash
cargo test
```

### 代码格式化
```bash
cargo fmt
```

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！
