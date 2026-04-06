# Claw TUI — Requirements

## v1 Requirements (This Milestone)

### R1: Structural Refactoring
- **R1.1**: Extract `LiveCli` struct + impl from main.rs into `app.rs`
- **R1.2**: Extract formatting functions into `format.rs` (status, cost, model reports)
- **R1.3**: Extract session management into `session_mgr.rs`
- **R1.4**: Extract arg parsing into `args.rs`
- **R1.5**: Create `tui/` module namespace
- **R1.6**: All existing tests pass after extraction

### R2: Ratatui Infrastructure
- **R2.1**: Add `ratatui` + `ratatui-textarea` to Cargo.toml
- **R2.2**: Terminal lifecycle management (raw mode, alternate screen, restore on exit/panic)
- **R2.3**: Async event loop with tokio channels (keyboard, mouse, resize, tick, render, custom)
- **R2.4**: `--tui` CLI flag to opt into TUI mode (REPL remains default)
- **R2.5**: Graceful terminal restore on panic (panic hook)

### R3: Core Layout
- **R3.1**: Vertical split: status bar (top) | body | input area (bottom)
- **R3.2**: Horizontal split within body: sidebar (left) | chat view (right)
- **R3.3**: Responsive layout that adapts to terminal resize
- **R3.4**: Live token counter in status bar (updated from AssistantEvent::Usage)
- **R3.5**: Model name, permission mode, session ID, git branch in status bar
- **R3.6**: Turn duration timer

### R4: Chat View
- **R4.1**: Scrollable conversation view with message history
- **R4.2**: User messages with distinct styling (role header)
- **R4.3**: Streaming markdown rendering (headings, bold, code, lists, tables)
- **R4.4**: Tool call blocks with box-drawing borders
- **R4.5**: Collapsible tool results (summary + expand for long outputs)
- **R4.6**: Syntax highlighting in code blocks (syntect)
- **R4.7**: Auto-scroll to bottom on new content
- **R4.8**: Thinking/reasoning indicator during processing

### R5: Sidebar Panel
- **R5.1**: Task/todo list with status indicators (pending/in_progress/completed)
- **R5.2**: File change list showing modified files in session
- **R5.3**: Tab navigation between sidebar sections
- **R5.4**: Toggle sidebar visibility with keybinding

### R6: Input & Commands
- **R6.1**: Multi-line text input (Shift+Enter for newline)
- **R6.2**: Slash command detection and tab completion
- **R6.3**: Input history navigation (up/down arrows)
- **R6.4**: All existing slash commands functional in TUI mode
- **R6.5**: Command output rendered in chat view
- **R6.6**: Normal/Insert mode with visual indicator
- **R6.7**: Keybinding help overlay (?)

### R7: Polish
- **R7.1**: Color theme system (dark default)
- **R7.2**: Scrollbar widgets on chat view and sidebar
- **R7.3**: Mouse support (click, scroll)
- **R7.4**: Terminal resize handling
- **R7.5**: Permission prompting in TUI mode
- **R7.6**: Error display and recovery
- **R7.7**: Session persistence integration

## v2 Requirements (Future)

### F1: Advanced Mouse Support
- Click to expand/collapse tool results
- Click sidebar items
- Text selection for copy

### F2: Color Themes
- Named themes (light, solarized, catppuccin)
- ANSI-256 / truecolor detection
- Configurable via config file

### F3: Advanced Navigation
- Search within conversation (/search)
- Interactive session picker with fuzzy filter
- Undo last file edit

### F4: Diff Visualization
- Colored unified diff rendering
- Side-by-side diff view
