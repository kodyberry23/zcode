# ZCode AI Agent Rules

## Project Overview

**Mission**: ZCode is a powerful terminal UI for integrating AI code assistants directly into your workflow. The application enables seamless AI-powered code generation, modification, and review with an interactive diff viewer, safe file operations, and Neovim/Vim integration.

**Tech Stack**: Rust, Ratatui (TUI framework), Crossterm (terminal manipulation), Tokio (async runtime)

**Architecture Style**: Message-driven (Elm architecture) with component-based UI system

## Core Principles

When working on this project, always:

1. **Read before writing** - Understand existing code patterns before making changes
2. **Follow Ratatui best practices** - Consult https://ratatui.rs/ for all UI work
3. **Use the theme system** - Never hardcode colors
4. **Test responsive layouts** - Verify at 60, 100, and 140 column widths
5. **Handle errors explicitly** - Use `Result<T>` and `?` operator
6. **Preserve terminal state** - Always restore raw mode and alternate screen

## Architecture Guidelines

### Message-Driven Design (Elm Architecture)

All state changes flow through the `Message` enum:

```
Event → Message → Update → Render
```

- State mutations ONLY happen in message handlers (`App::handle_message`)
- Components return `Option<Message>` to communicate
- Never mutate state directly outside message handlers
- Message enum is defined in `src/message.rs`

### Component System

All UI components implement the `Component` trait (`src/components/mod.rs`):

```rust
pub trait Component {
    fn init(&mut self, model: &mut AppModel) -> Option<Message>;
    fn update(&mut self, msg: &Message, model: &mut AppModel) -> Option<Message>;
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel);
    fn handle_key(&mut self, key: KeyEvent, model: &mut AppModel) -> Option<Message>;
    fn focused(&self) -> bool;
}
```

**Pattern**: Components delegate rendering to functions in `src/ui/` modules. Keep components focused on single responsibility.

### State Management

- `AppModel` (`src/model.rs`): Contains `AppState` + `Theme` + input modes
- `AppState` (`src/state.rs`): All application state
- `Mode` enum: High-level app states (ProviderSelect, PromptEntry, DiffReview, etc.)
- `InputMode` enum: Vim-style modal editing (Normal, Insert, Command, etc.)

## UI Development Standards

### Ratatui Best Practices (CRITICAL)

**ALWAYS consult Ratatui documentation first**: https://ratatui.rs/

#### Layout Constraints (https://ratatui.rs/concepts/layout/)

✅ **DO**:
- Use `Constraint::Min(n)` for flexible areas (preferred)
- Use `Constraint::Length(n)` only for truly fixed elements (status bars, single lines)
- Use nested layouts for complex UIs
- Add explicit spacers: `Constraint::Length(1)` between components

❌ **DON'T**:
- Never mix `Length` with `Percentage` in same layout
- Avoid complex single-layout constraints

#### Text Rendering (https://ratatui.rs/recipes/render/display-text/)

Use the hierarchy: **Span → Line → Text**

```rust
// Good - using Stylize trait
let line = Line::from(vec![
    "Hello ".into(),
    "World".yellow().bold(),
]);

// Good - alignment methods
paragraph.centered()
paragraph.left_aligned()
```

#### Block Widgets (https://ratatui.rs/recipes/widgets/block/)

```rust
// Standard pattern
Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)  // Use Rounded for consistency
    .border_style(theme.border_style)   // Always use theme
    .title(" Title ")
```

### Responsive Layout System

Three breakpoints defined in `src/ui/layout.rs`:

- **Compact** (< 80 cols): Vertical stack, single column
- **Normal** (80-120 cols): Optional sidebar, balanced split
- **Wide** (> 120 cols): Three-panel layout (Chat | Diff | Sidebar)

**Layout constraints**:
- Header: `Constraint::Length(3)` (fixed 3 lines, NOT 5!)
- Content: `Constraint::Min(8)` (flexible, minimum 8 lines)
- Input: `Constraint::Min(3)` + `Constraint::Max(6)` (multiline support)
- Status: `Constraint::Length(1)` (fixed single line)

**Centering**: Use `max_width_centered(area, 100)` instead of percentages for wide terminals.

### Theme System

All colors MUST come from `Theme` struct in `src/ui/colors.rs`:

```rust
// ✅ Good
.style(theme.normal_style)
.fg(theme.status_accepted.fg.unwrap_or(Color::Green))

// ❌ Bad - never hardcode colors
.style(Style::default().bg(Color::Rgb(15, 15, 15)))
```

Theme fields: `normal_style`, `border_style`, `status_accepted`, `error_style`, `added_style`, `removed_style`, etc.

### Editor Integration

Neovim/Vim integration pattern in `src/ui/editor.rs`:

1. **Suspend TUI**: `disable_raw_mode()` + `LeaveAlternateScreen`
2. **Launch editor**: `Command::new($EDITOR).arg(path).status()?`
3. **Resume TUI**: `enable_raw_mode()` + `EnterAlternateScreen` + `terminal.clear()`

Always check editor exists before suspending. Handle `OpenEditor` messages in app run loop (NOT in `handle_message`).

## Rust Code Standards

### Clean Code Principles

1. **No `unwrap()` in production code** - Use `?` operator or `unwrap_or_default()`
2. **Explicit error handling** - Return `Result<T>` from fallible operations
3. **No clippy warnings** - Run `cargo clippy -- -D warnings`
4. **Format before commit** - Run `cargo fmt --all`
5. **Document public APIs** - Use `///` doc comments

### Async/Await Patterns

- Use Tokio for async runtime: `tokio::spawn`, `tokio::task::JoinHandle`
- Use `async fn` for I/O-bound operations (providers, file ops)
- Keep event loop non-blocking - spawn tasks for long operations
- Use `tokio::sync::mpsc` for background task communication

### Error Handling

```rust
// Use anyhow for application errors
use anyhow::{Result, Context, bail};

fn load_config() -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .context("Failed to read config file")?;
    // ...
}

// Early return with error
if !path.exists() {
    bail!("File not found: {}", path.display());
}
```

Display errors in UI via `ErrorDisplay` struct in `src/state.rs`.

## File Organization

```
src/
├── main.rs              # Entry point, terminal setup
├── app.rs               # Main app loop, component orchestration
├── model.rs             # AppModel (state + theme + modes)
├── state.rs             # AppState (all application state)
├── message.rs           # Message enum (all state transitions)
├── events.rs            # Event system (keyboard, resize, tick)
├── executor.rs          # Async command execution
├── ui/                  # UI rendering (Ratatui)
│   ├── layout.rs        # Layout helpers (CRITICAL)
│   ├── editor.rs        # Neovim integration
│   ├── colors.rs        # Theme definitions
│   └── *.rs            # Rendering functions
├── components/          # UI components (trait-based)
├── input/              # Input handling (vim-style)
│   ├── keymap.rs       # Keybinding registry
│   └── parser.rs       # Multi-key sequence parser
├── providers/          # AI provider implementations
└── file_ops/           # Safe file operations
```

## Common Workflows

### Adding a New UI Component

1. Create struct in `src/components/your_component.rs`
2. Implement `Component` trait (init, update, view, handle_key, focused)
3. Create rendering function in `src/ui/your_component.rs`
4. Delegate from component's `view()` to rendering function
5. Add to `App` struct in `src/app.rs`
6. Include in view orchestration (`App::view()` or `App::render_main_layout()`)
7. **Use theme colors, never hardcode**

### Adding a New Message Type

1. Add variant to `Message` enum in `src/message.rs`
2. Handle in `App::handle_message()` in `src/app.rs`
3. Emit from components via `Option<Message>` return
4. Update keybindings in `src/input/keymap.rs` if needed
5. Document in README.md if user-facing

### Adding a New Keybinding

1. Add to `KeymapRegistry::default_vim()` in `src/input/keymap.rs`
2. Use multi-key sequences for vim-style: `&["g", "g"]` for gg
3. Bind to existing `Message` variant
4. Document in README.md keybindings section

### Creating a New Layout

1. Use `Layout::default()` with direction (Vertical/Horizontal)
2. Define constraints (prefer `Min` over `Percentage`)
3. Call `.split(area)` to get `Vec<Rect>`
4. Pass Rects to component render functions
5. **Test at all three breakpoints**: 60, 100, 140 columns

## Testing

- **Unit tests**: Same file, `#[cfg(test)] mod tests { ... }`
- **Integration tests**: `tests/` directory
- **Layout tests**: Ensure no overlaps, proper sizing at all breakpoints
- **Message flow tests**: Verify Event → Message → State change

## Critical Checklist

### Before Making Changes

- [ ] Read relevant existing code
- [ ] Understand message flow
- [ ] Check Ratatui docs for UI patterns
- [ ] Plan the changes

### While Implementing

- [ ] Follow existing patterns in codebase
- [ ] Use theme system for all colors
- [ ] Handle errors with `Result<T>` and `?`
- [ ] Keep async operations non-blocking
- [ ] Test at multiple terminal widths (60, 100, 140)

### Before Committing

- [ ] Run `cargo fmt --all`
- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo build` and verify compilation
- [ ] Test UI manually at different terminal sizes
- [ ] Update README.md if adding user-facing features
- [ ] Use conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`

## Critical Don'ts

❌ **DON'T** hardcode colors - use theme system
❌ **DON'T** mix `Constraint::Length` with `Constraint::Percentage`
❌ **DON'T** use `unwrap()` without error handling
❌ **DON'T** mutate state outside message handlers
❌ **DON'T** create files without reading existing code first
❌ **DON'T** use blocking I/O in event loop
❌ **DON'T** forget to restore terminal state (raw mode + alt screen)
❌ **DON'T** create documentation files unless explicitly requested

## Critical Do's

✅ **DO** read existing code before making changes
✅ **DO** follow Ratatui best practices religiously
✅ **DO** use `?` operator for error propagation
✅ **DO** test layout at multiple terminal widths
✅ **DO** use async/await for I/O operations
✅ **DO** run `cargo fmt` and `cargo clippy` before commit
✅ **DO** preserve terminal state in editor integration
✅ **DO** use descriptive variable names
✅ **DO** add comments for complex logic

## Key Documentation References

1. **Ratatui**: https://ratatui.rs/ (ALWAYS consult first for UI work)
   - Layout: https://ratatui.rs/concepts/layout/
   - Text: https://ratatui.rs/recipes/render/display-text/
   - Blocks: https://ratatui.rs/recipes/widgets/block/
   - Editor Integration: https://ratatui.rs/recipes/apps/spawn-vim/

2. **Crossterm**: https://docs.rs/crossterm/ (Terminal manipulation)

3. **Tokio**: https://tokio.rs/ (Async runtime)

4. **Internal Documentation**:
   - Architecture: `README.md` → Architecture section
   - Keybindings: `README.md` → Usage → Keybindings
   - Component System: `src/components/mod.rs`
   - Layout System: `src/ui/layout.rs`
   - Message Flow: `src/message.rs`

## Performance & Security

**Performance**:
- Minimize allocations in render loop
- Use `String::with_capacity()` for known sizes
- Cache computed layouts where possible
- Target 60fps (16ms frame budget)

**Security**:
- Validate all external input (AI provider output)
- Sanitize file paths before operations
- Never execute arbitrary commands from AI output
- Use atomic file operations with rollback
- Check command existence before spawning

---

**Remember**: This is a production-quality TUI application. Code quality, user experience, and terminal compatibility are paramount. When in doubt, consult Ratatui documentation and existing patterns in the codebase.
