# Pi - AI Coding Assistant

English | [中文](README.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A terminal AI coding assistant written in Rust, inspired by [pi-coding-agent](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent). Provides an interactive TUI interface with support for multiple LLM providers.

## Features

- **Multi-Provider Support**: OpenAI, Anthropic, Google, Moonshot, Ollama, Azure OpenAI, Mistral, Groq
- **Tool System**: Built-in file operation tools ([read](#tools), [write](#tools), [edit](#tools), [bash](#tools), [grep](#tools), [find](#tools), [ls](#tools), [epkg](#epkg-tool))
- **Session Management**: JSONL-based tree structure with branching support
- **Skill System**: Load custom skills to customize AI behavior
- **Interactive TUI**: Terminal user interface built with [ratatui](https://github.com/ratatui-org/ratatui)
- **Context Compaction**: Automatic summarization for long conversations
- **Extension System**: Extensible architecture for adding custom features

## Quick Start

```bash
# Clone project
git clone https://github.com/yourusername/pi-rs.git
cd pi-rs

# Build
cargo build --release

# Set API key (using Moonshot as example)
export MOONSHOT_API_KEY="your-api-key"

# Run
./target/release/pi --model moonshot-v1-8k "Hello, what can you do?"
```

## Installation

### Build from Source

```bash
git clone https://github.com/yourusername/pi-rs.git
cd pi-rs
cargo build --release
```

Binary is located at `target/release/pi`.

## Usage

### Command Line Options

```bash
pi [OPTIONS] [MESSAGE] [FILES]...

Arguments:
  MESSAGE      Initial message to send
  FILES        Files to include (use @ prefix)

Options:
  -c, --continue              Continue the most recent session
  -r, --resume               Resume/select a session
      --session <PATH>        Use specified session file
      --no-session           No session (temporary mode)
      --provider <NAME>      Provider name (openai, anthropic, moonshot, etc.)
      --model <MODEL>        Model name or pattern
      --thinking <LEVEL>      Thinking level (off, minimal, low, medium, high, xhigh)
      --api-key <KEY>        API key
      --list-models           List available models
      --tools <TOOLS>         Enable specific tools (comma-separated)
      --no-tools             Disable all built-in tools
  -e, --extension <PATH>     Load extension from path
      --skill <PATH>         Load skill from path
      --theme <PATH>         Load theme
  -p, --print               Print mode (non-interactive)
      --sandbox <PATH>        Enable sandbox mode (project path required)
  -v                        Additional mount paths (requires --sandbox)
  -E                        Sandbox env vars (requires --sandbox)
      --sandbox-type <TYPE>  Sandbox type (default: epkg)
      --no-sandbox           Disable sandbox (override config file)
  -h, --help               Print help
  -V, --version            Print version
```

### Usage Examples

```bash
# List available models
./target/release/pi --list-models

# Chat with Moonshot
./target/release/pi --model moonshot-v1-8k "List files in current directory"

# Use tools
./target/release/pi --model moonshot-v1-8k --tools bash,read "Read the Cargo.toml file"

# Continue previous session
./target/release/pi --continue
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `OPENAI_API_KEY` | OpenAI API key |
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `GOOGLE_API_KEY` | Google AI API key |
| `MOONSHOT_API_KEY` | Moonshot API key |
| `OLLAMA_BASE_URL` | Ollama base URL (default: http://localhost:11434) |
| `AZURE_OPENAI_API_KEY` | Azure OpenAI API key |
| `AZURE_OPENAI_ENDPOINT` | Azure OpenAI endpoint |
| `MISTRAL_API_KEY` | Mistral API key |
| `GROQ_API_KEY` | Groq API key |

## Tools

| Tool | Description |
|------|-------------|
| `read` | Read files from the file system |
| `write` | Write files to the file system |
| `edit` | Edit files using find/replace |
| `bash` | Execute shell commands |
| `grep` | Search for patterns in files |
| `find` | Find files by name |
| `ls` | List directory contents |
| `epkg` | Multi-source package manager |

### epkg Tool

Integrates [epkg](https://atomgits.com/openeuler/epkg) - a multi-source package manager for Linux, supporting packages from multiple distributions (RPM, DEB, Alpine, Arch, Conda).

```bash
# Search packages with epkg
./target/release/pi --tools epkg "Search for vim package"

# Install packages with epkg
./target/release/pi --tools epkg "Install nginx in openeuler environment"
```

Supported subcommands: `install`, `remove`, `update`, `upgrade`, `search`, `info`, `list`, `env`, `run`, `history`, `restore`, `gc`, `repo`, `self`, `build`

## Skill System

Skills allow you to customize the AI's behavior for specific tasks. See [skills](docs/skills.md) for details.

### Creating a Skill

```
my-skill/
├── skill.json    # Skill manifest
└── content.md   # Skill content (system prompt)
```

### skill.json Format

```json
{
  "name": "my-skill",
  "version": "1.0.0",
  "description": "Skill description",
  "triggers": ["trigger1", "trigger2"],
  "variables": []
}
```

<<<<<<< HEAD
### content.md

Contains the system prompt that is prepended to the conversation when the skill is triggered.

## Tools

The following tools are provided by default:

| Tool | Description |
|------|-------------|
| `read` | Read files from the file system |
| `write` | Write files to the file system |
| `edit` | Edit files using find/replace |
| `bash` | Execute shell commands |
| `grep` | Search for patterns in files |
| `find` | Find files by name |
| `ls` | List directory contents |

### epkg Tool

Integrates [epkg](https://atomgits.com/openeuler/epkg) multi-source package manager.

### Sandbox Mode

Run in an isolated sandbox environment to protect the host system.

```bash
# Enable sandbox (project path required)
pi-rs --sandbox /my/project

# With additional mounts
pi-rs --sandbox /my/project -v /opt/epkg -v /data

# With environment variables
pi-rs --sandbox /my/project -E CUSTOM_VAR=value

# Specify sandbox type (default: epkg)
pi-rs --sandbox /my/project --sandbox-type epkg

# Disable sandbox (override config file)
pi-rs --sandbox /my/project --no-sandbox
```

#### Configuration File

Create `.pi/sandbox.json` in project directory:

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

#### Environment Variables

The following environment variables are automatically propagated into sandbox:
- `MOONSHOT_API_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`
- `GOOGLE_API_KEY`, `OLLAMA_BASE_URL`, etc.

## Project Structure

```
pi-rs/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library exports
│   ├── core/             # Core types and utilities
│   ├── session/          # Session management
│   ├── tools/            # Tool implementations
│   ├── providers/        # LLM provider implementations
│   ├── agent/            # Agent core logic
│   ├── tui/              # Terminal UI
│   ├── skills/           # Skill system
│   ├── prompts/          # Prompt templates
│   ├── compaction/       # Context compaction
│   └── extensions/       # Extension system
└── tests/                # Unit tests
```

## Testing

```bash
# Run all tests
cargo test

# Run specific tests
cargo test skills
```

## Development

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run clippy
cargo clippy

# Format code
cargo fmt
```

## Performance Metrics

- **Binary Size**: ~8 MB
- **Runtime Memory**: ~9 MB

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [pi-coding-agent](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent)
- Terminal UI built with [ratatui](https://github.com/ratatui-org/ratatui)
- CLI parsing uses [clap](https://github.com/clap-rs/clap)
