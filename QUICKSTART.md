# 快速开始指南

## 🚀 5分钟快速上手

### 1. 构建工具

```bash
# 克隆项目
git clone <repository-url>
cd subtitle_renamer

# 构建发布版本
cargo build --release

# 安装到系统（可选）
sudo cp target/release/subtitle_renamer /usr/local/bin/
```

### 2. 本地使用

```bash
# 扫描当前目录
subtitle_renamer

# 扫描指定目录
subtitle_renamer -d /path/to/videos

# 先预览再执行
subtitle_renamer -d /path/to/videos -n  # 预览
subtitle_renamer -d /path/to/videos     # 执行
```

### 3. SSH远程使用

#### 设置SSH密钥（首次使用）

```bash
# 生成SSH密钥
ssh-keygen -t rsa -b 4096

# 复制公钥到远程服务器
ssh-copy-id username@your-server.com

# 测试连接
ssh username@your-server.com
```

#### 使用SSH模式

```bash
# 交互式模式
subtitle_renamer -s --ssh-user username --ssh-host your-server.com

# 直接重命名远程目录
subtitle_renamer -s --ssh-user username --ssh-host your-server.com --remote-dir /path/to/videos

# 模拟重命名
subtitle_renamer -s --ssh-user username --ssh-host your-server.com --remote-dir /path/to/videos -n
```

## 📁 文件结构示例

### 重命名前
```
/videos/
├── episode01.mp4
├── episode02.mp4
├── episode03.mp4
├── subtitle01.srt
├── subtitle02.srt
└── subtitle03.srt
```

### 重命名后
```
/videos/
├── episode01.mp4
├── episode02.mp4
├── episode03.mp4
├── episode01.srt    # 重命名自 subtitle01.srt
├── episode02.srt    # 重命名自 subtitle02.srt
└── episode03.srt    # 重命名自 subtitle03.srt
```

## 🔧 常用命令

### 本地操作
```bash
# 基本用法
subtitle_renamer                    # 当前目录
subtitle_renamer -d /videos        # 指定目录
subtitle_renamer -d /videos -n     # 预览模式
```

### 远程操作
```bash
# SSH连接
subtitle_renamer -s --ssh-user admin --ssh-host nas.local

# 直接操作
subtitle_renamer -s --ssh-user admin --ssh-host nas.local --remote-dir /media/videos

# 预览远程操作
subtitle_renamer -s --ssh-user admin --ssh-host nas.local --remote-dir /media/videos -n
```

## ⚠️ 注意事项

1. **备份重要文件**: 重命名操作不可逆，建议先备份
2. **预览模式**: 使用 `-n` 参数预览操作结果
3. **文件匹配**: 按字母顺序一一对应，确保文件数量相同
4. **SSH安全**: 优先使用SSH密钥，避免密码认证

## 🆘 遇到问题？

### 常见问题

1. **SSH连接失败**
   ```bash
   # 测试SSH连接
   ssh username@server.com
   
   # 检查密钥权限
   chmod 600 ~/.ssh/id_rsa
   ```

2. **权限不足**
   ```bash
   # 检查目录权限
   ls -la /path/to/videos
   ```

3. **文件数量不匹配**
   ```bash
   # 检查文件数量
   ls *.mp4 | wc -l
   ls *.srt | wc -l
   ```

### 获取帮助

```bash
# 查看帮助
subtitle_renamer --help

# 查看版本
subtitle_renamer --version
```

## 🎯 下一步

- 阅读完整的 [README.md](README.md) 文档
- 查看 [examples/](examples/) 目录中的示例
- 了解支持的文件格式和高级功能 