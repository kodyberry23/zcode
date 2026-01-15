# ZCode Quick Test Reference

## 🚀 Quick Start (30 seconds)

### Step 1: Build & Install (from any terminal)
```bash
~/projects/dotfiles/scripts/test-zcode.sh build-and-install
```

### Step 2: Launch Test Session

**From a real terminal (iTerm, Terminal.app, Ghostty, etc.) - NOT from Cursor:**

```bash
zellij attach -c zcode-test options --default-layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl
```

Or shorter:
```bash
zellij -n ~/projects/dotfiles/zellij/layouts/zcode-test.kdl
```

> ⚠️ **Note**: Zellij needs a real terminal. Run the launch command from iTerm/Terminal.app/Ghostty, not from Cursor's integrated terminal.

## 📋 Test Checklist

### Basic Functionality
- [ ] Plugin loads without errors
- [ ] Provider selection screen appears
- [ ] Can select a provider (j/k + Enter)
- [ ] Prompt entry mode works
- [ ] Can type and submit prompts
- [ ] Diff view displays correctly
- [ ] Can navigate hunks (j/k)
- [ ] Can accept/reject hunks (y/n)
- [ ] Changes apply correctly

### Integration with Your Zellij Config
- [ ] Autolock plugin doesn't conflict
- [ ] Alt+h/l navigation works between panes
- [ ] Can use ZCode while nvim is in another pane
- [ ] Keybindings don't conflict
- [ ] Theme looks good with your tokyo-night-storm theme

### File Operations
- [ ] Backups created in ~/.cache/zcode/backups/
- [ ] Files modified atomically
- [ ] Rollback works on errors

## 🎯 Quick Commands

```bash
# Rebuild after code changes
~/projects/dotfiles/scripts/test-zcode.sh build-and-install

# Check plugin info
~/projects/dotfiles/scripts/test-zcode.sh info

# Clean up test sessions
~/projects/dotfiles/scripts/test-zcode.sh cleanup

# View logs
tail -f ~/.cache/zellij/zellij-log/*.log
```

## 🔑 Key Bindings (Inside ZCode Plugin)

### Diff Review Mode
| Key | Action |
|-----|--------|
| `j` | Next hunk |
| `k` | Previous hunk |
| `y` | Accept hunk |
| `n` | Reject hunk |
| `a` | Accept all |
| `r` | Reject all |
| `Enter` | Apply changes |
| `q` | Quit |

### Your Zellij Keybindings (Outside Plugin)
| Key | Action |
|-----|--------|
| `Alt+h/l` | Navigate panes |
| `Alt+j/k` | Navigate panes |
| `Alt+t` | New tab |
| `Alt+x` | Close pane |
| `Ctrl+p` | Pane mode |

## 🐛 Quick Troubleshooting

**Plugin not loading?**
```bash
ls -lh ~/.config/zellij/plugins/zcode.wasm
# Should show ~1MB file
```

**Need to rebuild?**
```bash
cd ~/zcode && cargo build --release --target wasm32-wasip1
```

**Zellij version check:**
```bash
zellij --version  # Should be 0.40.0+
```

## 📁 Important Paths

- Plugin: `~/.config/zellij/plugins/zcode.wasm`
- Config: `~/.config/zcode/config.toml`
- Layout: `~/projects/dotfiles/zellij/layouts/zcode-test.kdl`
- Backups: `~/.cache/zcode/backups/`
- Logs: `~/.cache/zellij/zellij-log/*.log`

## 🎨 Layout Structure

```
┌─────────────────────────────────────────────┐
│              Tab Bar                        │
├──────────────────────┬──────────────────────┤
│                      │                      │
│   Terminal (60%)     │   ZCode (40%)        │
│                      │                      │
│   Your commands      │   Plugin UI          │
│   here               │   here               │
│                      │                      │
├──────────────────────┴──────────────────────┤
│              Status Bar                     │
└─────────────────────────────────────────────┘
```

## 💡 Pro Tips

1. **Quick iteration**: Keep the test script handy for rapid rebuild cycles
2. **Separate session**: Use `zcode-test` session name to avoid interfering with your main work
3. **Pane navigation**: Use Alt+h/l to quickly switch between terminal and ZCode
4. **Logs**: Keep a terminal with `tail -f` on Zellij logs for debugging
5. **Backups**: Check ~/.cache/zcode/backups/ to verify backup functionality

## 🔄 Development Workflow

```bash
# 1. Make changes to ZCode source
vim ~/zcode/src/...

# 2. Rebuild and install
~/projects/dotfiles/scripts/test-zcode.sh build-and-install

# 3. Restart test session
zellij delete-session zcode-test
~/projects/dotfiles/scripts/test-zcode.sh test

# 4. Test changes
# ... use the plugin ...

# 5. Repeat
```

## ✅ Success Indicators

You'll know it's working when:
- ✅ Plugin pane shows ZCode UI (not an error)
- ✅ You can select a provider
- ✅ You can enter prompts
- ✅ Diffs display with syntax highlighting
- ✅ File changes apply successfully
- ✅ Backups are created

## 🎓 Next Steps

Once basic testing works:
1. Test with real code files
2. Try different AI providers
3. Test multi-file changes
4. Customize keybindings
5. Add to your default layout
