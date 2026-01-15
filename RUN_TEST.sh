#!/bin/bash

# Simple ZCode test launcher
# Run this from a REAL terminal (iTerm, Terminal.app, Ghostty)
# NOT from Cursor's integrated terminal

if [ -n "$ZELLIJ" ]; then
    echo "⚠️  Please run this from OUTSIDE Zellij"
    echo "Detach first with: Ctrl+q"
    exit 1
fi

LAYOUT="$HOME/projects/dotfiles/zellij/layouts/zcode-test.kdl"

echo "🚀 Launching ZCode test session..."
echo ""

# Use -n flag for new session with layout
exec zellij -n "$LAYOUT"
