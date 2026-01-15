# Quick Test: Provider Detection

This document verifies that the `which` crate integration works correctly.

## Test: Verify which crate can find Claude

Run this test to verify Claude is detected:

```bash
cargo test test_which_finds_claude -- --nocapture --ignored
```

## Manual Verification

You can also manually verify by checking if `which` finds Claude:

```bash
which claude
# Should output: /opt/homebrew/bin/claude (or similar)
```

## In Zellij

When you run the plugin in Zellij, it should now detect Claude Code automatically and show it in the provider selection menu.

The plugin uses the following detection strategy:
1. Check for custom path in `~/.config/zcode/config.toml`
2. Use `which` crate to search PATH
3. Mark provider as available if found
