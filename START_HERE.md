# 🚀 Start Testing ZCode - Quick Guide

## ✅ Setup Complete!

Your ZCode plugin is built and installed. Here's how to test it:

## 🎯 Two Ways to Test

### Method 1: Direct Command (Simplest)

From **outside** any Zellij session, run:

```bash
zellij --layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl --session zcode-test
```

Or use the helper script:

```bash
cd ~/zcode
./RUN_TEST.sh
```

### Method 2: From Your Current Terminal

If you're currently in a Zellij session:

1. **Detach** from Zellij: Press `Ctrl+q`
2. **Run the test**:
   ```bash
   ~/projects/dotfiles/scripts/test-zcode.sh test
   ```

Or just run:
```bash
zellij --layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl --session zcode-test
```

## 📋 What You'll See

When the session launches, you'll see:

```
┌────────────────────────────────────────────┐
│            Tab Bar                         │
├─────────────────────┬──────────────────────┤
│                     │                      │
│  Terminal (60%)     │  ZCode Plugin (40%)  │
│                     │                      │
│  Your shell here    │  Plugin UI here      │
│                     │                      │
└─────────────────────┴──────────────────────┘
```

## 🎮 Quick Test Steps

1. **Launch the session** (using one of the methods above)
2. **Look at the right pane** - ZCode should be running
3. **Test navigation**:
   - Use `Alt+h` to focus left pane (terminal)
   - Use `Alt+l` to focus right pane (ZCode)
4. **In ZCode pane**:
   - Should see provider selection or plugin UI
   - Use `j`/`k` to navigate
   - Press `Enter` to select

## 🔧 Development Cycle

After making changes to ZCode:

```bash
# 1. Rebuild and install (from any terminal)
~/projects/dotfiles/scripts/test-zcode.sh build-and-install

# 2. If test session is running, restart it:
#    - Press Alt+q to quit Zellij
#    - Or: zellij delete-session zcode-test

# 3. Launch again
zellij --layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl --session zcode-test
```

## ⚠️ Important Notes

1. **Must run from outside Zellij**: You can't launch a new Zellij session from within Zellij
2. **Detach vs Quit**:
   - `Ctrl+q` - Detach (session keeps running)
   - `Alt+q` - Quit (session ends)
3. **Your keybindings work**: All your normal Zellij keybindings work in the test session

## 🐛 Troubleshooting

**"Already in Zellij" error?**
- Press `Ctrl+q` to detach first
- Then run the test command

**Plugin not showing?**
- Check: `ls -lh ~/.config/zellij/plugins/zcode.wasm`
- Should be ~1MB
- Rebuild if needed: `~/projects/dotfiles/scripts/test-zcode.sh build-and-install`

**Session already exists?**
- Delete it: `zellij delete-session zcode-test`
- Or attach to it: `zellij attach zcode-test`

## 📚 More Documentation

- `TESTING_SUMMARY.md` - Complete overview
- `QUICK_TEST.md` - Quick reference
- `TESTING.md` - Comprehensive guide

## 🎉 Ready to Go!

Your plugin is built and ready. Just run:

```bash
zellij --layout ~/projects/dotfiles/zellij/layouts/zcode-test.kdl --session zcode-test
```

(Make sure you're not already in a Zellij session first!)
