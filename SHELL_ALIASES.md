# Suggested Shell Aliases for ZCode Testing

Add these to your `~/.zshrc` for quick access:

```bash
# ZCode testing aliases
alias zcode-test='~/projects/dotfiles/scripts/test-zcode.sh test'
alias zcode-rebuild='~/projects/dotfiles/scripts/test-zcode.sh build-and-install'
alias zcode-full='~/projects/dotfiles/scripts/test-zcode.sh full'
alias zcode-info='~/projects/dotfiles/scripts/test-zcode.sh info'
alias zcode-clean='~/projects/dotfiles/scripts/test-zcode.sh cleanup'
alias zcode-logs='tail -f ~/.cache/zellij/zellij-log/*.log'

# Quick development cycle
alias zcode-dev='cd ~/zcode && cargo build --release --target wasm32-wasip1 && cp target/wasm32-wasip1/release/zcode.wasm ~/.config/zellij/plugins/'
```

## Usage Examples

After adding to your shell config and reloading (`source ~/.zshrc`):

```bash
# Quick test
zcode-test

# Full rebuild and test
zcode-full

# During development
cd ~/zcode
# ... make changes ...
zcode-rebuild
zcode-test

# Check status
zcode-info

# View logs while testing
zcode-logs

# Clean up when done
zcode-clean
```

## Integration with Your Workflow

Since you have a sessionizer script, you could also add:

```bash
# Add to ~/projects/dotfiles/scripts/sessionizer.sh
# Option to launch with ZCode layout
if [[ "$1" == "--zcode" ]]; then
    zellij --layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl
fi
```

Then use: `sessionizer.sh --zcode`
