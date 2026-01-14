# ZCode

[![CI](https://github.com/kodyberry23/zcode/actions/workflows/ci.yml/badge.svg)](https://github.com/kodyberry23/zcode/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE.md)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A powerful Zellij plugin for integrating AI code assistants directly into your terminal. ZCode enables seamless AI-powered code generation, modification, and review workflows within your Zellij workspace.

## Features

✨ **AI Integration**
- Support for multiple AI providers (Claude, Aider, and more)
- Extensible architecture for adding new providers
- Real-time streaming of AI responses
- Multi-turn conversation support

🔍 **Smart Diff Viewing**
- Interactive diff viewer with hunk-level navigation
- Syntax-aware line highlighting
- Scrollable diff display with pagination
- Hunk-by-hunk acceptance/rejection

✅ **Safe File Operations**
- Atomic file writes with automatic backups
- Transaction-like semantics across multiple files
- Automatic rollback on any operation failure
- Configurable backup retention

⚙️ **Highly Configurable**
- Custom keybindings for all modes
- User-selectable AI providers
- Display and color scheme options
- Context line customization

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Usage](#usage)
- [Architecture](#architecture)
- [Contributing](#contributing)
- [License](#license)

## Installation

### Prerequisites

- Rust 1.70 or later ([Install](https://rustup.rs/))
- Zellij 0.40.0 or later
- wasm32-wasip1 target: `rustup target add wasm32-wasip1`

### Build from Source

```bash
# Clone the repository
git clone https://github.com/kodyberry23/zcode
cd zcode

# Build the plugin
cargo build --release --target wasm32-wasip1

# The plugin will be at: target/wasm32-wasip1/release/zcode.wasm
```

### Add to Zellij

1. Copy the compiled plugin to your Zellij plugins directory:
   ```bash
   mkdir -p ~/.config/zellij/plugins
   cp target/wasm32-wasip1/release/zcode.wasm ~/.config/zellij/plugins/
   ```

2. Add to your `zellij` layout file or enable via configuration:
   ```yaml
   layout:
     panes:
       - plugin:
           location: "zcode"
   ```

## Quick Start

### Basic Workflow

1. **Open the Plugin**
   - Launch ZCode in your Zellij session
   - Select your preferred AI provider

2. **Enter Your Prompt**
   - Type your code modification request
   - Press `Enter` to submit

3. **Review Changes**
   - Navigate through diffs with `j`/`k` (next/prev hunk)
   - Accept hunks with `y`, reject with `n`
   - Accept all with `a`, reject all with `r`

4. **Apply Changes**
   - Press `Enter` to apply accepted changes
   - Backups are created automatically
   - Changes are applied atomically

## Configuration

Create `~/.config/zcode/config.toml` to customize behavior:

```toml
[general]
default_provider = "claude"        # Default AI provider
create_backups = true              # Auto-backup before changes
confirm_before_apply = true        # Require confirmation to apply
context_lines = 3                  # Lines of context in diffs

[display]
show_line_numbers = true           # Show line numbers in diffs
syntax_highlighting = true         # Highlight syntax
color_scheme = "dark"              # Color scheme

[keybindings]
# Override default keybindings
next_hunk = "j"
prev_hunk = "k"
accept_hunk = "y"
reject_hunk = "n"
quit = "q"
```

## Usage

### Keybindings

#### Diff Review Mode

| Key | Action |
|-----|--------|
| `j` | Next hunk |
| `k` | Previous hunk |
| `J` | Next file |
| `K` | Previous file |
| `g` | Go to beginning |
| `G` | Go to end |
| `y` | Accept current hunk |
| `n` | Reject current hunk |
| `a` | Accept all hunks |
| `r` | Reject all hunks |
| `Enter` | Apply changes |
| `q` / `Esc` | Quit |

#### Prompt Entry Mode

| Key | Action |
|-----|--------|
| `Ctrl+Enter` | Submit prompt |
| `Ctrl+c` | Cancel |
| `↑` / `↓` | History navigation |

#### Provider Selection Mode

| Key | Action |
|-----|--------|
| `j` / `k` | Select provider |
| `Enter` | Confirm selection |
| `q` | Cancel |

### Supported AI Providers

- **Claude** - Direct integration with Claude via Anthropic CLI
- **Aider** - Support for Aider's AI code assistant
- **Custom** - Extensible for other LLM tools

## Architecture

### Plugin Structure

```
zcode/
├── src/
│   ├── lib.rs                  # Plugin entry point
│   ├── state.rs                # Application state management
│   ├── config.rs               # Configuration loading
│   ├── diff.rs                 # Diff generation and extraction
│   ├── parsers.rs              # AI output parsing
│   ├── ui/                     # User interface
│   │   ├── mod.rs
│   │   ├── renderer.rs         # UI rendering trait
│   │   ├── components.rs       # UI components
│   │   ├── diff_view.rs        # Diff viewer
│   │   └── colors.rs           # Color schemes
│   ├── input/                  # Input handling
│   │   ├── mod.rs
│   │   ├── handler.rs          # Input handler trait
│   │   ├── keybindings.rs      # Keybinding management
│   │   └── modes/              # Mode-specific handlers
│   ├── file_ops/               # File operations
│   │   ├── mod.rs
│   │   ├── backup.rs           # Backup management
│   │   ├── apply.rs            # Apply changes
│   │   └── reconstruct.rs      # Hunk reconstruction
│   └── providers/              # AI provider implementations
│       ├── mod.rs
│       ├── claude.rs
│       └── aider.rs
├── Cargo.toml
├── README.md
├── LICENSE.md
└── CONTRIBUTING.md
```

### Key Components

- **State Management**: Centralized state machine for all modes
- **Diff Engine**: Unified diff generation using the `similar` crate
- **UI System**: ANSI escape codes with Zellij's native components
- **File Operations**: Atomic transactions with rollback support
- **Provider Abstraction**: Pluggable AI provider system

## Development

### Running Tests

```bash
# Run all tests
cargo test --lib

# Run specific test suite
cargo test --lib file_ops::

# Run with output
cargo test --lib -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy -- -D warnings

# Build WASM plugin
cargo build --release --target wasm32-wasip1
```

### Documentation

```bash
# Generate documentation
cargo doc --no-deps --open

# Build docs with warnings as errors
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Performance

- **Plugin Size**: ~1 MB (WASM release build)
- **Memory**: Minimal footprint (< 10 MB)
- **Response Time**: < 100ms for typical operations
- **Diff Generation**: Efficient diffing using patience algorithm

## Troubleshooting

### Plugin Not Loading
- Verify plugin is at `~/.config/zellij/plugins/zcode.wasm`
- Check Zellij version (requires 0.40.0+)
- Review Zellij logs for error messages

### AI Provider Not Found
- Ensure the AI tool CLI is installed and in PATH
- Check provider configuration in `config.toml`
- Verify provider binary permissions

### Backup Issues
- Backups are stored in `~/.cache/zcode/backups/`
- Ensure directory exists and is writable
- Check disk space availability

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:
- Reporting bugs
- Suggesting features
- Development setup
- Code quality standards
- Pull request process

## Roadmap

- [ ] Integration with Ollama for local LLM support
- [ ] Git integration for seamless diff management
- [ ] Language-specific parsers for better output extraction
- [ ] Plugin marketplace support
- [ ] Real-time collaboration features

## License

ZCode is licensed under the [MIT License](LICENSE.md).

## Acknowledgments

Inspired by tools like [Aider](https://aider.chat/), [GitHub Copilot](https://github.com/features/copilot), and the excellent [Zellij](https://zellij.dev/) plugin ecosystem.

Built with ❤️ for the Rust and AI development communities.

---

**Questions?** Open an [issue](https://github.com/kodyberry23/zcode/issues) or check existing discussions!
