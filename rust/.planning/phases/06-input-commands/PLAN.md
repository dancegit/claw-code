# Phase 6: Input & Commands

## Goal
Full multi-line input with slash command support in TUI mode.

## Tasks

### 6.1: tui/input_area.rs — Multi-line input (textarea)
- Use ratatui-textarea for multi-line input
- Shift+Enter for newline, Enter to submit
- Line wrapping for long lines
- Placeholder text when empty

### 6.2: Slash command detection and tab completion
- Detect `/` at start of input as slash command
- Tab completion for command names
- Show completion popup

### 6.3: Input history navigation
- Up/Down arrows navigate input history
- History stored per session

### 6.4: Wire slash commands to existing handlers
- Route slash commands to the same handlers used by REPL mode
- Results rendered in chat view

### 6.5: Command output in chat view
- Slash command output appears as system messages in chat view
- Support text and JSON output formats

### 6.6: Normal/Insert mode handling
- Default: Insert mode (input captures keys)
- Esc: Normal mode (keys navigate/chat)
- Visual mode indicator in status bar

### 6.7: Keybinding help overlay (?)
- Press `?` in Normal mode to show keybinding help
- Modal overlay with all keybindings
- Press any key to dismiss

## Success Criteria
- Multi-line input works with Shift+Enter for newline
- Slash commands auto-complete on Tab
- Up/Down navigates input history
- All REPL slash commands work in TUI mode
- Normal/Insert modes with visual indicator
