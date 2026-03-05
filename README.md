# Pi - AI 编程助手

[English](README_EN.md) | 中文

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

一款用 Rust 编写的终端 AI 编程助手，灵感来自 [pi-coding-agent](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent)。提供交互式 TUI 界面，支持多种 LLM 提供商。

## 功能特性

- **多提供商支持**：OpenAI、Anthropic、Google、Moonshot（月之暗面）、Ollama、Azure OpenAI、Mistral、Groq
- **工具系统**：内置文件操作工具（[read](#工具)、[write](#工具)、[edit](#工具)、[bash](#工具)、[grep](#工具)、[find](#工具)、[ls](#工具)、[epkg](#epkg-工具)）
- **会话管理**：基于 JSONL 的树形结构，支持分支
- **技能系统**：加载自定义技能以定制 AI 行为
- **交互式 TUI**：使用 [ratatui](https://github.com/ratatui-org/ratatui) 构建的终端用户界面
- **上下文压缩**：自动对长对话进行摘要
- **扩展系统**：可扩展架构，支持添加自定义功能

## 快速开始

```bash
# 克隆项目
git clone https://github.com/yourusername/pi-rs.git
cd pi-rs

# 构建
cargo build --release

# 设置 API 键（以 Moonshot 为例）
export MOONSHOT_API_KEY="your-api-key"

# 运行
./target/release/pi --model moonshot-v1-8k "你好，你会做什么？"
```

## 安装

### 从源码构建

```bash
git clone https://github.com/yourusername/pi-rs.git
cd pi-rs
cargo build --release
```

二进制文件位于 `target/release/pi`。

## 使用方法

### 命令行选项

```bash
pi [OPTIONS] [MESSAGE] [FILES]...

参数:
  MESSAGE      发送的初始消息
  FILES        要包含的文件（使用 @ 前缀）

选项:
  -c, --continue              继续最近的会话
  -r, --resume               恢复/选择会话
      --session <路径>        使用指定的会话文件
      --no-session           无会话（临时模式）
      --provider <名称>       提供商名称 (openai, anthropic, moonshot 等)
      --model <模型>         模型名称或模式
      --thinking <级别>       思考级别 (off, minimal, low, medium, high, xhigh)
      --api-key <密钥>       API 密钥
      --list-models           列出可用模型
      --tools <工具>         启用指定工具（逗号分隔）
      --no-tools             禁用所有内置工具
  -e, --extension <路径>   从路径加载扩展
      --skill <路径>         从路径加载技能
      --theme <路径>         加载主题
  -p, --print               打印模式（非交互式）
      --sandbox <路径>        启用沙箱模式（必需指定项目路径）
  -v                        沙箱额外挂载目录（需要 --sandbox）
  -E                        沙箱环境变量（需要 --sandbox）
      --sandbox-type <类型>  沙箱类型（默认：epkg）
      --no-sandbox           禁用沙箱（覆盖配置文件）
  -h, --help               打印帮助信息
  -V, --version            打印版本信息
```

### 使用示例

```bash
# 列出可用模型
./target/release/pi --list-models

# 使用 Moonshot 对话
./target/release/pi --model moonshot-v1-8k "列出当前目录的文件"

# 使用工具
./target/release/pi --model moonshot-v1-8k --tools bash,read "读取 Cargo.toml 文件"

# 继续之前的会话
./target/release/pi --continue
```

## 环境变量

| 变量 | 说明 |
|------|------|
| `OPENAI_API_KEY` | OpenAI API 密钥 |
| `ANTHROPIC_API_KEY` | Anthropic API 密钥 |
| `GOOGLE_API_KEY` | Google AI API 密钥 |
| `MOONSHOT_API_KEY` | Moonshot（月之暗面）API 密钥 |
| `OLLAMA_BASE_URL` | Ollama 基础 URL（默认：http://localhost:11434） |
| `AZURE_OPENAI_API_KEY` | Azure OpenAI API 密钥 |
| `AZURE_OPENAI_ENDPOINT` | Azure OpenAI 端点 |
| `MISTRAL_API_KEY` | Mistral API 密钥 |
| `GROQ_API_KEY` | Groq API 密钥 |

## 工具

| 工具 | 说明 |
|------|------|
| `read` | 从文件系统读取文件 |
| `write` | 向文件系统写入文件 |
| `edit` | 使用查找/替换编辑文件 |
| `bash` | 执行 shell 命令 |
| `grep` | 在文件中搜索模式 |
| `find` | 按名称查找文件 |
| `ls` | 列出目录内容 |
| `epkg` | 多源软件包管理器 |

### epkg 工具

集成 [epkg](https://atomgits.com/openeuler/epkg) 多源软件包管理器，支持从多个 Linux 发行版安装软件包（RPM、DEB、Alpine、Arch、Conda）。

```bash
# 使用 epkg 搜索包
./target/release/pi --tools epkg "搜索 vim 包"

# 使用 epkg 安装包
./target/release/pi --tools epkg "在 openeuler 环境安装 nginx"
```

支持的子命令：`install`, `remove`, `update`, `upgrade`, `search`, `info`, `list`, `env`, `run`, `history`, `restore`, `gc`, `repo`, `self`, `build`

## 技能系统

技能允许你为特定任务定制 AI 的行为。详见 [skills](docs/skills.md)。

### 创建技能

```
my-skill/
├── skill.json    # 技能清单
└── content.md   # 技能内容（系统提示词）
```

### skill.json 格式

```json
{
  "name": "my-skill",
  "version": "1.0.0",
  "description": "技能描述",
  "triggers": ["触发词1", "触发词2"],
  "variables": []
}
```

### content.md

包含系统提示词，当技能被触发时会预先添加到对话中。

## 工具

| 工具 | 说明 |
|------|------|
| `read` | 从文件系统读取文件 |
| `write` | 向文件系统写入文件 |
| `edit` | 使用查找/替换编辑文件 |
| `bash` | 执行 shell 命令 |
| `grep` | 在文件中搜索模式 |
| `find` | 按名称查找文件 |
| `ls` | 列出目录内容 |

### epkg 工具

集成 [epkg](https://atomgits.com/openeuler/epkg) 多源软件包管理器。

### 沙箱模式

支持在隔离的沙箱环境中运行，保护主机系统。

```bash
# 启用沙箱（必需指定项目路径）
pi-rs --sandbox /my/project

# 带额外挂载目录
pi-rs --sandbox /my/project -v /opt/epkg -v /data

# 带环境变量
pi-rs --sandbox /my/project -E CUSTOM_VAR=value

# 指定沙箱类型（默认：epkg）
pi-rs --sandbox /my/project --sandbox-type epkg

# 禁用沙箱（覆盖配置文件）
pi-rs --sandbox /my/project --no-sandbox
```

#### 配置文件

在项目目录创建 `.pi/sandbox.json`：

```json
{
  "enabled": true,
  "type": "epkg",
  "mounts": ["/opt/epkg", "/data"],
  "env": {
    "CUSTOM_VAR": "value"
  }
}
```

#### 环境变量

沙箱内自动继承以下环境变量：
- `MOONSHOT_API_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`
- `GOOGLE_API_KEY`, `OLLAMA_BASE_URL`, 等

## 项目结构

```
pi-rs/
├── src/
│   ├── main.rs           # CLI 入口点
│   ├── lib.rs            # 库导出
│   ├── core/             # 核心类型和工具
│   ├── session/          # 会话管理
│   ├── tools/            # 工具实现
│   ├── providers/        # LLM 提供商实现
│   ├── agent/            # 助手核心逻辑
│   ├── tui/              # 终端 UI
│   ├── skills/           # 技能系统
│   ├── sandbox/          # 沙箱模块
│   ├── prompts/          # 提示词模板
│   ├── compaction/       # 上下文压缩
│   └── extensions/       # 扩展系统
└── tests/                # 单元测试
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test skills
```

## 开发

```bash
# 调试构建
cargo build

# 发布构建
cargo build --release

# 运行 clippy
cargo clippy

# 格式化代码
cargo fmt
```

## 性能指标

- **二进制大小**：~8 MB
- **运行时内存**：约 9 MB

## 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- 灵感来自 [pi-coding-agent](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent)
- 终端 UI 使用 [ratatui](https://github.com/ratatui-org/ratatui) 构建
- CLI 参数解析使用 [clap](https://github.com/clap-rs/clap)
