# Pi - AI Coding Assistant

English | [中文](README.md)

[![Rust](https://github.com/badlogic/pi-mono/actions/workflows/rust.yml/badge.svg)](https://github.com/badlogic/pi-mono/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A terminal AI coding assistant written in Rust, inspired by [pi-coding-agent](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent). Provides an interactive TUI interface with support for multiple LLM providers.

## Features

- **Multi-Provider Support**: OpenAI, Anthropic, Google, Moonshot, Ollama, Azure OpenAI, Mistral, Groq
- **Tool System**: Built-in file operation tools (read, write, edit, bash, grep, find, ls)
- **Session Management**: JSONL-based tree structure with branching support
- **Skill System**: Load custom skills to customize AI behavior
- **Interactive TUI**: Terminal user interface built with ratatui
- **Context Compaction**: Automatic summarization for long conversations
- **Extension System**: Extensible architecture for adding custom features

## Installation

### Build from Source

```bash
git clone https://github.com/yourusername/pi-rs.git
cd pi-rs
cargo build --release
```

### Binary Location

The compiled binary is located at `target/release/pi`

## Usage

### Quick Start

```bash
# Set API key (using Moonshot as example)
export MOONSHOT_API_KEY="your-api-key"

# Simple conversation
./target/release/pi --model moonshot-v1-8k "Hello, what can you do?"

# List available models
./target/release/pi --list-models
```

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
      --provider <NAME>       Provider name (openai, anthropic, moonshot, etc.)
      --model <MODEL>         Model name or pattern
      --thinking <LEVEL>       Thinking level (off, minimal, low, medium, high, xhigh)
      --api-key <KEY>         API key
      --list-models           List available models
      --tools <TOOLS>         Enable specific tools (comma-separated)
      --no-tools             Disable all built-in tools
  -e, --extension <PATH>   Load extension from path
      --skill <PATH>         Load skill from path
      --theme <PATH>         Load theme
  -p, --print               Print mode (non-interactive)
  -h, --help               Print help
  -V, --version            Print version
```

### Usage Examples

```bash
# Chat with Moonshot
./target/release/pi --model moonshot-v1-8k "List files in current directory"

# Use tools (bash, read, write, edit)
./target/release/pi --model moonshot-v1-8k "Read the Cargo.toml file"

# Use custom skills
./target/release/pi --model moonshot-v1-8k --skill /path/to/skill "trigger word"

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

## Skill System

Skills allow you to customize the AI's behavior for specific tasks.

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

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run -- --model moonshot-v1-8k "Hello"
```

### Code Quality

```bash
# Run clippy
cargo clippy

# Format code
cargo fmt
```

## Performance Metrics

- **Binary Size**: 6.8 MB
- **Runtime Memory**: ~9.2-9.7 MB

### Runtime Memory by Feature

| Feature | Memory Usage |
|---------|-------------|
| Simple chat | 9.2 MB |
| Tool call (bash) | 9.2 MB |
| Tool call (read) | 9.2 MB |
| Tool call (write) | 9.2 MB |
| Tool call (edit) | 9.2 MB |
| Tool call (grep) | 9.3 MB |
| Tool call (find) | 9.3 MB |
| Tool call (ls) | 9.3 MB |
| Skill system | 9.2 MB |

### Build & Test

- **Test Suite Memory**: ~63 MB
- **Test Directory Size**: 1.1 GB (debug build)
- **Test Coverage**: 107 unit tests, 30 test suites, all passing

### Functional Test Results

| Feature | Status | Notes |
|---------|--------|-------|
| Simple chat | ✅ Pass | Moonshot API responds normally |
| Tool call (bash) | ✅ Pass | Can execute ls and other commands |
| Tool call (read) | ✅ Pass | Can read file contents |
| Tool call (write) | ✅ Pass | Can create new files |
| Tool call (edit) | ✅ Pass | Can edit file contents |
| Tool call (grep) | ✅ Pass | Can search file contents |
| Tool call (find) | ✅ Pass | Can find files |
| Tool call (ls) | ✅ Pass | Can list directories |
| Skill system | ✅ Pass | Custom skills work correctly |
| Multi-turn chat | ✅ Pass | Supports context memory |

### Test Examples

```bash
# Simple chat
$ ./target/release/pi --model moonshot-v1-8k "Hello"
=== Response ===
Hello! How can I help you?

# Tool call (bash)
$ ./target/release/pi --model moonshot-v1-8k "Execute ls command using bash tool"
=== Response ===
After executing the `ls` command, the files and folders in the current directory are:
- Cargo.lock
- Cargo.toml
- src
- tests

# Tool call (read)
$ ./target/release/pi --model moonshot-v1-8k "Read the first 10 lines of Cargo.toml"
=== Response ===
The first 10 lines of Cargo.toml are:
[package]
name = "pi-rs"
version = "0.1.0"

# Skill system
$ ./target/release/pi --model moonshot-v1-8k --skill /path/to/skill "trigger"
=== Response ===
Skill is working!
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [pi-coding-agent](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent)
- Terminal UI built with [ratatui](https://github.com/ratatui-org/ratatui)
- CLI parsing uses [clap](https://github.com/clap-rs/clap)
