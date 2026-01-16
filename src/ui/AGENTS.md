# UI Module Agent Rules

## Critical Resources

**ALWAYS consult these first before any UI work**:
- 📚 [Ratatui Layout Guide](https://ratatui.rs/concepts/layout/) - Layout system documentation
- 📚 [Ratatui Recipes](https://ratatui.rs/recipes/) - Common UI patterns
- 📚 [Ratatui Website Source](https://github.com/ratatui/ratatui-website) - Reference implementations
- 📚 [Ratatui Examples](https://github.com/ratatui/ratatui/tree/main/ratatui/examples) - Official examples

## UI Module Purpose

This directory contains all Ratatui-based rendering logic. Each file is a pure rendering function that takes state and produces UI output. **No state mutations happen here.**

## Files in This Directory

- `layout.rs` - Layout helpers and responsive breakpoint system (CRITICAL)
- `editor.rs` - Neovim/Vim suspend/resume integration
- `colors.rs` - Theme definitions and color schemes
- `renderers.rs` - Legacy rendering functions
- `header.rs` - Top header bar rendering
- `status_bar.rs` - Bottom status bar rendering
- `session_turn.rs` - Chat message rendering
- `prompt_input.rs` - Prompt input rendering
- `overlay_diff.rs` - Diff overlay rendering
- `sidebar.rs` - Sidebar panel rendering
- `help.rs` - Help overlay rendering
- `logo.rs` - ASCII logo rendering
- `widgets/` - Custom Ratatui widgets

## Critical UI Rules

### Layout System (layout.rs)

**Read this file FIRST before any layout work**: `src/ui/layout.rs`

Three responsive breakpoints:
- **Compact** (< 80 cols): Single column, vertical stack
- **Normal** (80-120 cols): Optional sidebar, balanced split
- **Wide** (> 120 cols): Three-panel (Chat | Diff | Sidebar)

**Constraint Guidelines** (from [Ratatui Layout Docs](https://ratatui.rs/concepts/layout/)):

✅ **USE**:
```rust
// Flexible content area
Constraint::Min(8)           // Minimum 8 lines, grows as needed

// Fixed elements (status bars, single lines)
Constraint::Length(1)        // Exactly 1 line

// Capped flexible areas
Constraint::Min(3)           // At least 3 lines
Constraint::Max(6)           // But no more than 6 lines

// Explicit spacers
Constraint::Length(1)        // 1-line gap between components
```

❌ **NEVER USE**:
```rust
// Don't mix these in the same layout!
Constraint::Percentage(50)
Constraint::Length(10)

// Don't use percentages for fixed-width panels
Constraint::Percentage(20)   // Use Length instead for sidebars
```

**Standard Layout Pattern**:
```rust
let vertical = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),   // Header (fixed)
        Constraint::Min(8),      // Content (flexible)
        Constraint::Min(3),      // Input (flexible)
        Constraint::Max(6),      // Input cap
        Constraint::Length(1),   // Status (fixed)
    ])
    .split(area);
```

### Theme System (colors.rs)

**NEVER hardcode colors**. All colors come from `Theme` struct:

```rust
// ✅ Good - use theme
frame.render_widget(
    Paragraph::new(text).style(theme.normal_style),
    area
);

// ✅ Good - with theme color
.fg(theme.status_accepted.fg.unwrap_or(Color::Green))

// ❌ Bad - hardcoded color
.style(Style::default().bg(Color::Rgb(15, 15, 15)))
```

### Text Rendering

Follow the hierarchy from [Ratatui Text Guide](https://ratatui.rs/recipes/render/display-text/):

**Span → Line → Text**

```rust
// Use Stylize trait for cleaner code
let line = Line::from(vec![
    Span::raw("Normal "),
    "Yellow".yellow().bold(),  // Chainable styling
    " text".into(),
]);

// Alignment methods
paragraph.centered()
paragraph.left_aligned()
paragraph.right_aligned()
```

### Block Widgets

Standard pattern from [Ratatui Block Guide](https://ratatui.rs/recipes/widgets/block/):

```rust
Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)  // Use Rounded consistently
    .border_style(theme.border_style)  // Use theme
    .title(" Title ")                  // Space padding for title
```

### Editor Integration (editor.rs)

Pattern from [Ratatui Editor Guide](https://ratatui.rs/recipes/apps/spawn-vim/):

```rust
// 1. Suspend TUI
disable_raw_mode()?;
execute!(io::stdout(), LeaveAlternateScreen)?;

// 2. Launch editor
let status = Command::new($EDITOR).arg(path).status()?;

// 3. Resume TUI (ALWAYS, even on error)
enable_raw_mode()?;
execute!(io::stdout(), EnterAlternateScreen)?;
terminal.clear()?;
```

## Rendering Function Signature

All rendering functions follow this pattern:

```rust
pub fn render_component(
    frame: &mut Frame,
    area: Rect,
    state: &State,  // Or specific state slice
    theme: &Theme,
) {
    // Rendering logic only
    // NO state mutations
}
```

## Common Patterns

### Centering Content

```rust
// For dialogs/modals
use crate::ui::layout::centered_rect_percent;
let dialog = centered_rect_percent(area, 60, 40);

// For wide terminals with max-width
use crate::ui::layout::max_width_centered;
let input = max_width_centered(area, 100);
```

### Responsive Splits

```rust
// Check width and adjust layout
if area.width > 120 {
    // Wide: side-by-side
    let chunks = Layout::horizontal([
        Constraint::Min(40),
        Constraint::Length(25),
    ]).split(area);
} else {
    // Normal: stacked
    let chunks = Layout::vertical([
        Constraint::Min(10),
        Constraint::Length(10),
    ]).split(area);
}
```

### Nested Layouts

```rust
// Vertical split first
let vertical = Layout::vertical([
    Constraint::Length(3),
    Constraint::Min(0),
]).split(area);

// Then horizontal split within content
let horizontal = Layout::horizontal([
    Constraint::Min(40),
    Constraint::Length(25),
]).split(vertical[1]);
```

## Testing UI Changes

1. **Build and run**: `cargo build && cargo run`
2. **Test breakpoints**: Resize terminal to 60, 80, 100, 120, 160 columns
3. **Verify spacing**: Check that components have proper margins
4. **Check text**: Ensure no truncation or wrapping issues
5. **Confirm colors**: All colors come from theme

## Performance Considerations

- **Minimize allocations in render loop**: Use `&str` instead of `String` where possible
- **Cache static content**: Pre-build Lines/Spans that don't change
- **Avoid complex calculations**: Pre-compute in state, not during render
- **Profile render time**: Should be < 16ms per frame for 60fps

## UI Module Checklist

When modifying UI code:

- [ ] Consulted [Ratatui Layout Docs](https://ratatui.rs/concepts/layout/)
- [ ] Used correct constraint types (Min for flex, Length for fixed)
- [ ] No mixed Length/Percentage constraints in same layout
- [ ] All colors from theme, no hardcoded colors
- [ ] Tested at 60, 100, 140 column widths
- [ ] Used BorderType::Rounded for consistency
- [ ] Function signature follows pattern (frame, area, state, theme)
- [ ] No state mutations in rendering functions
- [ ] Added spacing between components
- [ ] Text uses Span → Line → Text hierarchy

## Examples from Ratatui

Reference these when building new UI:

- [Layout Examples](https://github.com/ratatui/ratatui/blob/main/ratatui/examples/layout.rs)
- [Text Rendering](https://github.com/ratatui/ratatui/blob/main/ratatui/examples/paragraph.rs)
- [Block Styling](https://github.com/ratatui/ratatui/blob/main/ratatui/examples/block.rs)
- [List Widget](https://github.com/ratatui/ratatui/blob/main/ratatui/examples/list.rs)

## Common Mistakes to Avoid

❌ Mutating state in render functions
❌ Using percentages for fixed-width sidebars
❌ Mixing Length and Percentage constraints
❌ Hardcoding colors instead of using theme
❌ Forgetting to restore terminal state in editor integration
❌ Not testing at different terminal widths
❌ Using complex calculations in hot render path
❌ Forgetting spacers between components

---

**When in doubt, check the [Ratatui documentation](https://ratatui.rs/) and [example code](https://github.com/ratatui/ratatui/tree/main/ratatui/examples) first!**
