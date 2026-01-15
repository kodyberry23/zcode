# ZCode Testing Guide

This guide explains how to test the ZCode Zellij plugin with your current Zellij configuration.

## Quick Start

### Option 1: Using the Test Script (Recommended)

A convenient test script has been created at `~/projects/dotfiles/scripts/test-zcode.sh`:

```bash
# Full workflow: rebuild, install, and launch
~/projects/dotfiles/scripts/test-zcode.sh full

# Or step by step:
~/projects/dotfiles/scripts/test-zcode.sh rebuild           # Build the plugin
~/projects/dotfiles/scripts/test-zcode.sh install          # Install to Zellij
~/projects/dotfiles/scripts/test-zcode.sh test             # Launch test session

# Other useful commands:
~/projects/dotfiles/scripts/test-zcode.sh info             # Show plugin info
~/projects/dotfiles/scripts/test-zcode.sh cleanup          # Clean up test sessions
```

### Option 2: Manual Testing

```bash
# 1. Build the plugin
cd ~/zcode
cargo build --release --target wasm32-wasip1

# 2. Install to Zellij
cp target/wasm32-wasip1/release/zcode.wasm ~/.config/zellij/plugins/

# 3. Launch with test layout
zellij --layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl --session zcode-test
```

## Test Layout

A custom layout has been created at `~/projects/dotfiles/zellij/layouts/zcode-test.kdl`:

- **Left pane (60%)**: Terminal for running commands
- **Right pane (40%)**: ZCode plugin interface

This layout allows you to:
- Work in the terminal on the left
- See the ZCode plugin UI on the right
- Test the plugin without it taking over your entire screen

## Configuration

The test configuration is located at `~/.config/zcode/config.toml` with sensible defaults:

- **Provider**: Claude (default)
- **Backups**: Enabled
- **Confirmation**: Required before applying changes
- **Context lines**: 3 lines around diffs

## Testing Workflow

### 1. Initial Setup
```bash
# Ensure you have the wasm32-wasip1 target
rustup target add wasm32-wasip1

# Build and install
~/projects/dotfiles/scripts/test-zcode.sh build-and-install
```

### 2. Launch Test Session
```bash
# Start a dedicated test session
~/projects/dotfiles/scripts/test-zcode.sh test
```

### 3. Test Basic Functionality

Once in the Zellij session with ZCode loaded:

#### a. Provider Selection
- The plugin should start in provider selection mode
- Use `j`/`k` to navigate providers
- Press `Enter` to select

#### b. Prompt Entry
- Type a code modification prompt
- Press `Ctrl+Enter` to submit (or the configured key)
- Watch for AI response streaming

#### c. Diff Review
- Navigate hunks with `j` (next) and `k` (previous)
- Accept hunks with `y`, reject with `n`
- Use `a` to accept all, `r` to reject all
- Press `Enter` to apply accepted changes

#### d. File Operations
- Check that backups are created in `~/.cache/zcode/backups/`
- Verify atomic file writes work correctly
- Test rollback on errors

### 4. Test with Your Zellij Config

Your current Zellij config has these relevant features:

#### Autolock Plugin
- The autolock plugin watches for `nvim|vim` and `fzf|zoxide|atuin`
- This should work alongside ZCode without conflicts
- Test switching between locked and normal modes

#### Keybindings Compatibility
Your config uses:
- `Alt+h/j/k/l` for pane navigation
- `Alt+t` for new tab
- `Ctrl+p` for pane mode
- `Ctrl+g` for locked mode

**Potential conflicts**: None! ZCode uses its own keybindings within the plugin pane.

#### Navigation Testing
1. Open ZCode in a pane
2. Use `Alt+h/l` to navigate between terminal and ZCode panes
3. Verify autolock doesn't interfere with ZCode's input handling

## Development Iteration

### Quick Rebuild Cycle
```bash
# Make changes to ZCode source
cd ~/zcode

# Rebuild and reload
~/projects/dotfiles/scripts/test-zcode.sh build-and-install

# Kill and restart the test session
zellij delete-session zcode-test
~/projects/dotfiles/scripts/test-zcode.sh test
```

### Debugging

#### Check Plugin Logs
```bash
# Zellij logs are typically at:
tail -f ~/.cache/zellij/zellij-log/*.log
```

#### Verify Plugin Loading
```bash
# Check plugin exists and size
ls -lh ~/.config/zellij/plugins/zcode.wasm

# Should be around 1MB for release build
```

#### Test Build Issues
```bash
# Clean build
cd ~/zcode
cargo clean
cargo build --release --target wasm32-wasip1

# Run tests
cargo test --lib

# Check for warnings
cargo clippy -- -D warnings
```

## Integration with Your Workflow

### Using with Nvim
Your nvim config includes zellij-nav.nvim for seamless navigation. To use ZCode with nvim:

1. Open nvim in one pane
2. Open ZCode in another pane
3. Use `Alt+h/l` to navigate between them
4. ZCode can modify files that nvim has open (nvim will detect changes)

### Using with Your Session Manager
Your sessionizer script (`~/projects/dotfiles/scripts/sessionizer.sh`) can be extended:

```bash
# Add to sessionizer.sh to launch with ZCode
zellij --layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl
```

## Common Issues

### Plugin Not Loading
- **Check**: Plugin file exists at `~/.config/zellij/plugins/zcode.wasm`
- **Check**: Zellij version is 0.40.0+ (`zellij --version`)
- **Fix**: Rebuild with `~/projects/dotfiles/scripts/test-zcode.sh rebuild`

### AI Provider Not Found
- **Check**: Claude CLI or Aider is installed and in PATH
- **Check**: Provider configuration in `~/.config/zcode/config.toml`
- **Fix**: Install the required AI tool or change provider

### Keybinding Conflicts
- **Check**: Your Zellij config for conflicting bindings
- **Fix**: Modify keybindings in `~/.config/zcode/config.toml`

### Backup Directory Issues
- **Check**: `~/.cache/zcode/backups/` exists and is writable
- **Fix**: `mkdir -p ~/.cache/zcode/backups`

## Performance Testing

### Build Size
```bash
# Check WASM size (should be ~1MB)
du -h ~/zcode/target/wasm32-wasip1/release/zcode.wasm
```

### Memory Usage
```bash
# Monitor Zellij memory while using ZCode
ps aux | grep zellij
```

### Response Time
- Diff generation should be < 100ms for typical files
- UI updates should feel instant
- Large files (>1000 lines) may take longer

## Advanced Testing

### Test Multiple Files
1. Create a test project with multiple files
2. Ask ZCode to modify multiple files
3. Verify all changes are shown in diff view
4. Test accepting some hunks and rejecting others

### Test Error Handling
1. Try to modify a read-only file
2. Verify proper error messages
3. Check that rollback works correctly

### Test Long Prompts
1. Enter a very long prompt
2. Verify scrolling works in prompt entry mode
3. Test history navigation

## CI/CD Integration

The ZCode project has GitHub Actions CI. To test locally:

```bash
cd ~/zcode

# Run the same checks as CI
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test --lib
cargo build --release --target wasm32-wasip1
```

## Next Steps

After testing, consider:
1. Adding ZCode to your default Zellij layout
2. Creating custom keybindings that match your workflow
3. Setting up your preferred AI provider
4. Contributing improvements back to the project

## Resources

- [Zellij Documentation](https://zellij.dev/documentation/)
- [Zellij Plugin API](https://zellij.dev/documentation/plugin-api)
- [ZCode Repository](https://github.com/kodyberry23/zcode)
- Your Zellij Config: `~/projects/dotfiles/zellij/config.kdl`
