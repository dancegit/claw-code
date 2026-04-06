# Claw Code — Ratatui TUI Migration

## Vision

Transform the existing REPL-style claw CLI into a polished, modern ratatui-based TUI experience comparable to OpenCode and Claude Code CLI — with persistent sidebar, real-time task panels, collapsible message blocks, and split-pane layout — while preserving all existing functionality (streaming, tools, slash commands, sessions, permissions).

## Problem Statement

The current Rust CLI is functionally complete (streaming responses, tool execution, slash commands, session management) but presents a flat, linear REPL interface built on rustyline. It lacks:
- **Sidebar panel** for files/tasks/status visibility
- **Persistent task list** visible during interaction
- **Collapsible/expandable message blocks** for conversation flow
- **Status bar** with model, tokens, cost, session info
- **Multi-pane layout** for power-user orchestration

## Goals

1. **Full ratatui-based TUI** as the primary interface mode (opt-in via `--tui` flag initially)
2. **Split-pane layout**: sidebar (tasks/files/agents) | main chat area | input area
3. **Rich message rendering**: collapsible tool calls, syntax-highlighted code blocks, streaming markdown
4. **Live status bar**: model, token count, cost, session ID, git branch, elapsed time
5. **Task panel**: persistent todo list visible in sidebar with live progress
6. **Preserve existing REPL mode** as fallback (no `--tui` flag)

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│ Status Bar: model | tokens | cost | session | branch | elapsed  │
├────────────────┬─────────────────────────────────────────────────┤
│                │                                                  │
│   Sidebar      │   Chat View (scrollable)                        │
│   ─────────    │   ────────────────────────────                  │
│   Tasks        │   [User] message...                             │
│   Files        │   [Assistant] streaming response...             │
│   Agents       │   ┌─ bash ──────────────────┐                  │
│                │   │ $ cargo test             │ ✓               │
│                │   └──────────────────────────┘                  │
│                │                                                  │
│                │                                                  │
├────────────────┴─────────────────────────────────────────────────┤
│ Input Area: > type here... (Tab for completions, ? for help)    │
└──────────────────────────────────────────────────────────────────┘
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| TUI Framework | ratatui 0.30+ |
| Terminal Backend | crossterm 0.28 (already present) |
| Async Runtime | tokio (already present) |
| Markdown Rendering | pulldown-cmark (already present) |
| Syntax Highlighting | syntect (already present) |
| Text Input | ratatui-textarea |
| Event System | tokio::sync::mpsc channels |

## Module Structure (Target)

```
crates/rusty-claude-cli/src/
├── main.rs              # Entrypoint, arg dispatch (~200 lines)
├── args.rs              # CLI argument parsing
├── app.rs               # LiveCli struct, core REPL loop
├── format.rs            # Report formatting (status, cost, model, permissions, etc.)
├── session_mgr.rs       # Session CRUD: create, resume, list, switch, persist
├── init.rs              # Repo initialization (unchanged)
├── input.rs             # rustyline line editor (unchanged)
├── render.rs            # TerminalRenderer, Spinner, ColorTheme
└── tui/
    ├── mod.rs           # TUI module root, App struct, event loop
    ├── terminal.rs      # Terminal lifecycle (init, raw mode, restore)
    ├── events.rs        # Async event handler (tokio + crossterm)
    ├── layout.rs        # Layout definition (split-pane, constraints)
    ├── chat_view.rs     # Chat rendering (messages, streaming, collapsible)
    ├── sidebar.rs       # Sidebar panel (tasks, files, agents)
    ├── status_bar.rs    # Bottom status bar (model, tokens, cost)
    ├── input_area.rs    # Multi-line input widget
    ├── theme.rs         # Color theme definitions
    └── components.rs    # Shared component trait
```

## Success Criteria

- [ ] `cargo run --bin claw -- --tui` launches full-screen TUI
- [ ] Sidebar shows tasks, files, agents with tab navigation
- [ ] Chat view renders streaming markdown with collapsible tool calls
- [ ] Status bar shows model, tokens, cost, session, branch
- [ ] Input area supports multi-line, slash commands, tab completion
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] Terminal cleanly restores on exit/panic
