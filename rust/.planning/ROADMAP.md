# Claw TUI — Roadmap

## Milestone: TUI v1 — Ratatui Migration

### Phase 1: Structural Cleanup
**Goal**: Break the 9,200-line `main.rs` monolith into focused modules.

| Task | Description | Effort | Depends on |
|------|-------------|--------|------------|
| 1.1 | Extract `LiveCli` struct + impl into `app.rs` | M | - |
| 1.2 | Extract format functions into `format.rs` | M | - |
| 1.3 | Extract session management into `session_mgr.rs` | S | - |
| 1.4 | Extract arg parsing into `args.rs` | S | - |
| 1.5 | Create `tui/mod.rs` module namespace | S | - |
| 1.6 | Verify all tests pass after extraction | S | 1.1-1.4 |

**QA**: `cargo test --workspace` passes, `cargo clippy` clean.

---

### Phase 2: Ratatui Infrastructure
**Goal**: Add ratatui dependency, terminal lifecycle, async event loop.

| Task | Description | Effort | Depends on |
|------|-------------|--------|------------|
| 2.1 | Add `ratatui` + `ratatui-textarea` to Cargo.toml | S | 1.5 |
| 2.2 | Create `tui/terminal.rs` — Terminal lifecycle | M | 2.1 |
| 2.3 | Create `tui/events.rs` — Async event loop | M | 2.1 |
| 2.4 | Create `tui/app.rs` — Main TUI app struct | M | 2.2, 2.3 |
| 2.5 | Add `--tui` flag to arg parser | S | 2.4 |
| 2.6 | Wire TUI mode in main.rs | S | 2.5 |
| 2.7 | Panic hook for graceful terminal restore | S | 2.2 |

**QA**: `cargo run --bin claw -- --tui` shows blank alternate screen, exits cleanly on 'q'.

---

### Phase 3: Core Layout & Status Bar
**Goal**: Implement the full split-pane layout with live status bar.

| Task | Description | Effort | Depends on |
|------|-------------|--------|------------|
| 3.1 | Create `tui/layout.rs` — Layout definitions | M | 2.4 |
| 3.2 | Create `tui/status_bar.rs` — Status bar widget | M | 3.1 |
| 3.3 | Create `tui/components.rs` — Component trait | S | 3.1 |
| 3.4 | Wire status bar into app render loop | S | 3.2 |
| 3.5 | Implement responsive resize | S | 3.1 |
| 3.6 | Live token counter updates | M | 3.2, Phase 4 |

**QA**: TUI shows proper split-pane layout with status bar, responsive to terminal resize.

---

### Phase 4: Chat View
**Goal**: Rich conversation rendering with streaming markdown and collapsible blocks.

| Task | Description | Effort | Depends on |
|------|-------------|--------|------------|
| 4.1 | Create `tui/chat_view.rs` — Chat view widget | L | 3.3 |
| 4.2 | Message rendering (user/assistant/system roles) | M | 4.1 |
| 4.3 | Streaming markdown rendering | L | 4.2 |
| 4.4 | Tool call block rendering (box-drawing) | M | 4.2 |
| 4.5 | Collapsible tool results | M | 4.4 |
| 4.6 | Syntax highlighting in code blocks | M | 4.3 |
| 4.7 | Auto-scroll with PgUp/PgDn | S | 4.1 |
| 4.8 | Thinking indicator | S | 4.2 |
| 4.9 | Wire AssistantEvent stream to chat view | M | 4.3 |

**QA**: Full conversation rendering with streaming, collapsible tool calls, scrolling.

---

### Phase 5: Sidebar & Task Panel
**Goal**: Persistent sidebar showing tasks, files, and agent status.

| Task | Description | Effort | Depends on |
|------|-------------|--------|------------|
| 5.1 | Create `tui/sidebar.rs` — Sidebar container | M | 3.3 |
| 5.2 | Task list widget (from todo tracking) | M | 5.1 |
| 5.3 | File change list widget | S | 5.1 |
| 5.4 | Tab navigation between sections | M | 5.2, 5.3 |
| 5.5 | Toggle sidebar visibility | S | 5.1 |
| 5.6 | Live updates as tools execute | M | 5.2 |

**QA**: Sidebar shows tasks with status, file changes, tab navigation, toggle visibility.

---

### Phase 6: Input & Commands
**Goal**: Full multi-line input with slash command support in TUI mode.

| Task | Description | Effort | Depends on |
|------|-------------|--------|------------|
| 6.1 | Create `tui/input_area.rs` — Multi-line input (textarea) | M | 3.3 |
| 6.2 | Slash command detection and tab completion | M | 6.1 |
| 6.3 | Input history navigation | S | 6.1 |
| 6.4 | Wire slash commands to existing handlers | M | 6.2 |
| 6.5 | Command output in chat view | M | 6.4 |
| 6.6 | Normal/Insert mode handling | M | 6.1 |
| 6.7 | Keybinding help overlay (?) | S | 6.6 |

**QA**: Full input experience matching REPL capabilities within TUI.

---

### Phase 7: Polish & Integration
**Goal**: Production-quality finishing touches.

| Task | Description | Effort | Depends on |
|------|-------------|--------|------------|
| 7.1 | Create `tui/theme.rs` — Color theme system | M | Phase 3 |
| 7.2 | Scrollbar widgets on chat and sidebar | S | Phase 4, 5 |
| 7.3 | Mouse support (click, scroll) | M | Phase 4, 5 |
| 7.4 | Terminal resize handling refinement | S | Phase 3 |
| 7.5 | Permission prompting in TUI mode | M | Phase 6 |
| 7.6 | Error display and recovery | S | Phase 4 |
| 7.7 | Session persistence integration | S | Phase 6 |
| 7.8 | Performance profiling | M | All |

**QA**: Polished, production-ready TUI with all features working end-to-end.

---

## Dependency Graph

```
Phase 1 (Structural Cleanup)
    ↓
Phase 2 (Ratatui Infrastructure)
    ↓
Phase 3 (Core Layout)
    ↓
    ├── Phase 4 (Chat View) ← parallel
    └── Phase 5 (Sidebar)  ← parallel
    ↓
Phase 6 (Input & Commands)
    ↓
Phase 7 (Polish)
```

Phases 4 and 5 can be parallelized after Phase 3 completes.
