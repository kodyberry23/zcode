# ZCode

[![CI](https://github.com/kodyberry23/zcode/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/kodyberry23/zcode/actions/workflows/ci.yml)
[![Release](https://github.com/kodyberry23/zcode/actions/workflows/release.yml/badge.svg)](https://github.com/kodyberry23/zcode/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE.md)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A powerful terminal UI for integrating AI code assistants directly into your workflow. ZCode enables seamless AI-powered code generation, modification, and review with an interactive diff viewer and safe file operations.

```
  _________ ___  ____  _____ 
 |__  / ___/ _ \|  _ \| ____|
   / / |  | | | | | | |  _|  
  / /| |__| |_| | |_| | |___ 
 /____\____\___/|____/|_____|
```

## Features

✨ **AI Integration**
- Support for multiple AI providers (Claude, Aider, GitHub Copilot, and more)
- Extensible architecture for adding custom providers
- Async command execution with real-time feedback
- Multi-turn conversation support

🎨 **Modern Terminal UI**
- Beautiful, responsive interface powered by Ratatui
- Follows Ratatui best practices for layout and rendering
- Clean, balanced layouts that adapt to terminal size (60-160+ columns)
- Improved spacing and visual hierarchy
- Helpful welcome screen with getting started guide
- Smooth animations and real-time feedback

🔍 **Interactive Diff Viewer**
- Hunk-level navigation and review
- Syntax-aware line highlighting
- Scrollable diff display with pagination
- Selective acceptance/rejection of changes
- Seamless Neovim/Vim integration for file editing

✅ **Safe File Operations**
- Atomic file writes with automatic backups
- Transaction-like semantics across multiple files
- Automatic rollback on any operation failure
- Configurable backup retention

🔧 **Editor Integration**
- Suspend TUI to launch Neovim/Vim for file editing
- Open files at specific line numbers
- Respects `$EDITOR` environment variable
- Terminal state preserved across editor sessions
- Changes automatically detected and reloaded

⚙️ **Highly Configurable**
- Custom keybindings for all modes
- User-selectable AI providers with persistence
- Display and color scheme options (dark/light themes)
- Context line customization
- XDG-compliant configuration

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
- A terminal emulator (any modern terminal)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/kodyberry23/zcode
cd zcode

# Build the application
cargo build --release

# The binary will be at: target/release/zcode
```

### Install to System

```bash
# Install to ~/.cargo/bin (make sure it's in your PATH)
cargo install --path .

# Or copy the binary to a location in your PATH
sudo cp target/release/zcode /usr/local/bin/
```

### Download Pre-built Binary

Download the latest pre-built binary from the [Releases page](https://github.com/kodyberry23/zcode/releases):

**macOS / Linux:**
```bash
# Download the latest release (replace v0.1.0 with the latest version tag)
curl -L -o zcode https://github.com/kodyberry23/zcode/releases/download/v0.1.0/zcode

# Make it executable
chmod +x zcode

# Move to a location in your PATH
sudo mv zcode /usr/local/bin/
```

**Note:** Check the [Releases page](https://github.com/kodyberry23/zcode/releases) for the latest version tag.

## Quick Start

### Run ZCode

```bash
# Run the application
zcode

# Or if built from source
./target/release/zcode
```

### Basic Workflow

1. **Select AI Provider**
   - Use `j`/`k` to navigate through available providers
   - Press `Enter` to select your preferred AI provider (e.g., Claude Code)
   - Your selection is saved for next time

2. **Enter Your Prompt**
   - Type your code modification request
   - Press `Enter` to submit your prompt
   - Watch the animated spinner while the AI processes

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
color_scheme = "dark"              # Color scheme (dark/light)

[keybindings]
# Override default keybindings
next_hunk = "j"
prev_hunk = "k"
accept_hunk = "y"
reject_hunk = "n"
quit = "q"

# Custom provider paths (optional)
# Useful if your AI tools are not in PATH or you want to use specific versions
[providers.claude]
enabled = true
# path = "/opt/homebrew/bin/claude"  # Optional: custom path

[providers.aider]
enabled = true
# path = "/usr/local/bin/aider"

[providers.copilot]
enabled = true
# path = "/usr/local/bin/gh"

# Add custom AI providers
[providers.my_custom_ai]
enabled = true
name = "My Custom AI"
path = "/path/to/my/ai/tool"
parser = "unified_diff"  # or "code_blocks", "json"
```

### Provider Detection

ZCode automatically detects installed AI providers by checking:

1. **PATH Detection**: Searches for executables in your system PATH
2. **Custom Paths**: Checks for custom paths defined in `config.toml`
3. **Cross-Platform**: Works on macOS, Linux, and Windows

If a provider is not detected automatically, you can specify its path in the config file.

## Usage

### Keybindings

#### Provider Selection Mode

| Key | Action |
|-----|--------|
| `j` / `k` | Navigate providers |
| `g` / `G` | Jump to first/last |
| `Enter` | Select provider |
| `q` | Quit |

#### Prompt Entry Mode

| Key | Action |
|-----|--------|
| Type | Enter prompt text |
| `Shift+Enter` | New line (multiline support) |
| `←` / `→` | Move cursor |
| `Home` / `End` | Jump to start/end |
| `Ctrl+U` | Clear line |
| `Enter` | Submit prompt |
| `Esc` | Back to provider selection |
| `Ctrl+C` | Quit |

#### Normal Mode

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll down/up |
| `h` / `l` | Scroll left/right |
| `g g` | Jump to top |
| `G` | Jump to bottom |
| `/` | Search |
| `:` | Command mode |
| `?` | Toggle help |
| `Ctrl+B` | Toggle sidebar |
| `q` | Quit |

#### Diff Review Mode

| Key | Action |
|-----|--------|
| `j` / `k` | Next/previous hunk |
| `J` / `K` | Next/previous file |
| `g` / `G` | Jump to beginning/end |
| `y` | Accept current hunk |
| `n` | Reject current hunk |
| `Y` | Accept all hunks |
| `N` | Reject all hunks |
| `Enter` | Apply changes |
| `q` / `Esc` | Quit |

#### Confirmation Mode

| Key | Action |
|-----|--------|
| `y` / `Enter` | Confirm |
| `n` / `Esc` | Cancel |

### Supported AI Providers

- **Claude** - Anthropic's Claude AI via official CLI
- **Aider** - AI pair programming tool
- **GitHub Copilot CLI** - GitHub's AI assistant
- **Kiro** - AWS's AI code assistant (formerly Amazon Q)
- **Custom** - Extensible for other LLM tools

### Editor Integration

ZCode seamlessly integrates with Neovim/Vim for editing files:

**How it works:**
1. The TUI suspends (disables raw mode, leaves alternate screen)
2. Your `$EDITOR` is launched (defaults to `nvim`)
3. Edit the file normally in your preferred editor
4. Upon exit, the TUI automatically resumes
5. File changes are detected and reloaded

**Environment Variables:**
- Set `$EDITOR` to your preferred editor (e.g., `export EDITOR=vim`)
- Falls back to `nvim` if `$EDITOR` is not set

**Features:**
- Open files at specific line numbers
- Terminal state fully preserved
- No corruption or artifacts
- Works with any terminal editor

**Future Enhancement:**
Keybindings will be added to open files directly from the diff view (e.g., press `e` on a hunk to edit that file at that location).

### Layout & Responsiveness

ZCode's UI adapts to your terminal size with three responsive breakpoints:

**Compact Mode** (< 80 columns):
- Vertical stack layout
- Single column view
- Ideal for split terminal windows

**Normal Mode** (80-120 columns):
- Optional sidebar (toggle with `Ctrl+B`)
- Balanced content/sidebar split (flexible:25)
- Comfortable for laptop screens

**Wide Mode** (> 120 columns):
- Three-panel layout: Chat | Diff | Sidebar
- Side-by-side chat and diff viewer
- Optimal for external monitors and ultra-wide displays

All layouts follow Ratatui best practices:
- Use `Constraint::Min` for flexible areas
- Avoid mixing fixed `Length` with `Percentage`
- Explicit spacers between components
- Max-width centering (100 cols) for input on wide terminals

## Architecture

### Application Structure

```
zcode/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main app struct with Ratatui integration
│   ├── state.rs             # Application state management
│   ├── model.rs             # Application model (state + config)
│   ├── config.rs            # Configuration loading
│   ├── executor.rs          # Async command execution
│   ├── events.rs            # Event system (keyboard, resize, etc)
│   ├── message.rs           # Message-driven architecture
│   ├── parsers.rs           # AI output parsing
│   ├── ui/                  # User interface
│   │   ├── mod.rs
│   │   ├── renderers.rs     # Ratatui-based rendering
│   │   ├── colors.rs        # Color schemes and themes
│   │   ├── layout.rs        # Responsive layout helpers
│   │   ├── editor.rs        # Neovim/Vim integration
│   │   ├── header.rs        # Top header bar
│   │   ├── status_bar.rs    # Bottom status bar
│   │   ├── session_turn.rs  # Chat message rendering
│   │   ├── prompt_input.rs  # Prompt input component
│   │   ├── overlay_diff.rs  # Diff overlay viewer
│   │   ├── sidebar.rs       # Sidebar panel
│   │   ├── help.rs          # Help overlay
│   │   ├── logo.rs          # ASCII logo
│   │   └── widgets/         # Custom Ratatui widgets
│   ├── components/          # UI component trait system
│   │   ├── mod.rs
│   │   ├── chat_panel.rs
│   │   ├── diff_view.rs
│   │   ├── provider_select.rs
│   │   └── ...
│   ├── input/               # Input handling
│   │   ├── mod.rs
│   │   ├── handler.rs       # Input handler trait
│   │   ├── keymap.rs        # Vim-style keybinding registry
│   │   ├── parser.rs        # Multi-key sequence parser
│   │   └── modes/           # Mode-specific handlers
│   ├── file_ops/            # File operations
│   │   ├── mod.rs
│   │   ├── backup.rs        # Backup management
│   │   ├── apply.rs         # Apply changes
│   │   └── reconstruct.rs   # Hunk reconstruction
│   ├── providers/           # AI provider implementations
│   │   ├── mod.rs
│   │   ├── claude.rs
│   │   ├── aider.rs
│   │   ├── copilot.rs
│   │   ├── amazon_q.rs
│   │   └── custom.rs
│   ├── neovim/              # Neovim RPC integration
│   │   ├── client.rs        # Neovim client
│   │   ├── extmarks.rs      # Extmark management
│   │   └── highlights.rs    # Highlight groups
│   └── session.rs           # Session management
├── Cargo.toml
├── README.md
├── LICENSE.md
└── CONTRIBUTING.md
```

### Key Components

- **Message-Driven Architecture**: Elm-style update pattern for predictable state management
- **Component System**: Reusable UI components with consistent trait interface
- **Responsive Layout**: Ratatui-based layouts that adapt to terminal size (3 breakpoints: Compact, Normal, Wide)
- **State Management**: Centralized state machine for all modes
- **Async Execution**: Non-blocking command execution with Tokio
- **Event System**: Unified event handling for keyboard, mouse, resize, and timer events
- **Diff Engine**: Unified diff generation using the `similar` crate
- **UI System**: Modern terminal UI following Ratatui best practices
- **Editor Integration**: Suspend/resume TUI pattern for seamless Neovim/Vim integration
- **File Operations**: Atomic transactions with rollback support
- **Provider Abstraction**: Pluggable AI provider system
- **Vim-style Keybindings**: Multi-key sequence support with modal editing

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

# Build release binary
cargo build --release
```

### Documentation

```bash
# Generate documentation
cargo doc --no-deps --open

# Build docs with warnings as errors
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Performance

- **Binary Size**: ~5 MB (release build, optimized)
- **Memory**: Minimal footprint (< 10 MB)
- **Response Time**: < 100ms for typical operations
- **Diff Generation**: Efficient diffing using patience algorithm
- **Async I/O**: Non-blocking command execution

## Troubleshooting

### Application Won't Start
- Verify you have Rust 1.70+ installed: `rustc --version`
- Try rebuilding: `cargo clean && cargo build --release`
- Check terminal compatibility (most modern terminals work)

### AI Provider Not Found
- Ensure the AI tool CLI is installed and in PATH
- Check provider configuration in `~/.config/zcode/config.toml`
- Verify provider binary permissions: `which claude` (or aider, etc.)
- Specify custom path in config if needed

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

### Recently Completed ✅
- [x] Responsive layout with Ratatui best practices (3 breakpoints)
- [x] Improved spacing and visual hierarchy
- [x] Neovim/Vim integration (suspend/resume TUI)
- [x] Welcome screen with getting started guide
- [x] Multiline prompt input support
- [x] Max-width centering for better wide terminal support
- [x] Message-driven architecture (Elm-style)
- [x] Component-based UI system

### In Progress 🚧
- [ ] Editor keybindings for opening files from diff view
- [ ] Syntax highlighting in diff viewer
- [ ] Auto-scroll in chat panel

### Planned 📋
- [ ] Real-time streaming of AI responses
- [ ] Better diff visualization with inline syntax highlighting
- [ ] Integration with Ollama for local LLM support
- [ ] Git integration for seamless diff management
- [ ] Command palette (fuzzy finder)
- [ ] Mouse support
- [ ] Plugin system for custom providers
- [ ] Theme customization and more color schemes
- [ ] Persistent session history

## Releases & Versioning

### Downloading Releases

Pre-built binaries are available on the [Releases page](https://github.com/kodyberry23/zcode/releases). Each release includes:

- Pre-compiled binary for your platform
- Installation instructions
- Change notes

### Creating a Release

To create a new release, follow these steps:

```bash
# 1. Update version in Cargo.toml
# 2. Commit the version bump
git add Cargo.toml
git commit -m "chore: bump version to X.Y.Z"

# 3. Create and push the version tag
git tag vX.Y.Z
git push origin main
git push origin vX.Y.Z
```

GitHub Actions will automatically:
1. Build the binary
2. Create a GitHub Release
3. Upload the binary as a downloadable asset

This uses a **tag-based release workflow** which:
- Gives explicit control over when releases happen
- Works for both direct pushes and PR merges
- Follows Rust ecosystem conventions
- Avoids accidental releases on every commit

## License

ZCode is licensed under the [MIT License](LICENSE.md).

## Acknowledgments

Inspired by tools like [Aider](https://aider.chat/), [GitHub Copilot](https://github.com/features/copilot), and the excellent [Ratatui](https://ratatui.rs/) TUI framework.

Built with ❤️ for the Rust and AI development communities.

---

**Questions?** Open an [issue](https://github.com/kodyberry23/zcode/issues) or check existing discussions!
