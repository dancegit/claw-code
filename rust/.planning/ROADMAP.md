# Claw TUI — Roadmap

## Milestone: TUI v1 — Ratatui Migration

### Phase 1: Structural Cleanup ~~DEFERRED~~
**Goal**: Break the 9,200-line `main.rs` monolith into focused modules.

> **STATUS: DEFERRED** — Attempted automated extraction twice; failed due to massive cross-reference web in the 9,820-line file. Not a blocker for TUI work. Revisit after TUI is functional.

| Task | Description | Effort | Status |
|------|-------------|--------|--------|
| 1.1 | Extract `LiveCli` struct + impl into `app.rs` | M | deferred |
| 1.2 | Extract format functions into `format.rs` | M | deferred |
| 1.3 | Extract session management into `session_mgr.rs` | S | deferred |
| 1.4 | Extract arg parsing into `args.rs` | S | deferred |
| 1.5 | Create `tui/` module namespace | S | ✅ done |
| 1.6 | Verify all tests pass after extraction | S | deferred |

---

### Phase 2: Ratatui Infrastructure
**Goal**: Add ratatui dependency, terminal lifecycle, async event loop, `--tui` flag.

| Task | Description | Effort | Status |
|------|-------------|--------|--------|
| 2.1 | Add `ratatui` + `ratatui-textarea` to Cargo.toml | S | ✅ done |
| 2.2 | Create `tui/terminal.rs` — Terminal lifecycle | M | ✅ done |
| 2.3 | Create `tui/events.rs` — Async event loop | M | ✅ done |
| 2.4 | Create `tui/app.rs` — Main TUI app struct | M | ✅ done (skeleton) |
| 2.5 | Add `--tui` flag to arg parser | S | ⬜ next |
| 2.6 | Wire TUI mode in main.rs + `pub(crate)` LiveCli | S | ⬜ next |
| 2.7 | Panic hook for graceful terminal restore | S | ✅ done |

**QA**: `cargo run --bin claw -- --tui` shows alternate screen with layout, exits cleanly on Ctrl+Q.

---

### Phase 3: Core Layout & Status Bar
**Goal**: Implement the full split-pane layout with live status bar.

| Task | Description | Effort | Status |
|------|-------------|--------|--------|
| 3.1 | Create `tui/layout.rs` — Layout definitions | M | ✅ done (basic) |
| 3.2 | Create `tui/status_bar.rs` — Status bar widget | M | ⬜ |
| 3.3 | Create `tui/components.rs` — Component trait | S | ✅ done |
| 3.4 | Wire status bar into app render loop | S | ⬜ |
| 3.5 | Implement responsive resize | S | ⬜ |
| 3.6 | Live token counter updates | M | ⬜ |

**QA**: TUI shows proper split-pane layout with status bar, responsive to terminal resize.

---

### Phase 4: Chat View
**Goal**: Rich conversation rendering with streaming markdown and collapsible blocks.

| Task | Description | Effort | Status |
|------|-------------|--------|--------|
| 4.1 | Create `tui/chat_view.rs` — Chat view widget | L | ⬜ |
| 4.2 | Message rendering (user/assistant/system roles) | M | ⬜ |
| 4.3 | Streaming markdown rendering (pulldown-cmark) | L | ⬜ |
| 4.4 | Tool call block rendering (box-drawing) | M | ⬜ |
| 4.5 | Collapsible tool results | M | ⬜ |
| 4.6 | Syntax highlighting in code blocks (syntect) | M | ⬜ |
| 4.7 | Auto-scroll with PgUp/PgDn | S | ⬜ |
| 4.8 | Thinking indicator | S | ⬜ |
| 4.9 | Wire AssistantEvent stream to chat view | M | ⬜ |

**QA**: Full conversation rendering with streaming, collapsible tool calls, scrolling.

---

### Phase 5: Sidebar & Task Panel
**Goal**: Persistent sidebar showing tasks, files, and agent status.

| Task | Description | Effort | Status |
|------|-------------|--------|--------|
| 5.1 | Create `tui/sidebar.rs` — Sidebar container | M | ⬜ |
| 5.2 | Task list widget (from todo tracking) | M | ⬜ |
| 5.3 | File change list widget | S | ⬜ |
| 5.4 | Tab navigation between sections | M | ⬜ |
| 5.5 | Toggle sidebar visibility | S | ⬜ |
| 5.6 | Live updates as tools execute | M | ⬜ |

**QA**: Sidebar shows tasks with status, file changes, tab navigation, toggle visibility.

---

### Phase 6: Input & Commands
**Goal**: Full multi-line input with slash command support in TUI mode.

| Task | Description | Effort | Status |
|------|-------------|--------|--------|
| 6.1 | Create `tui/input_area.rs` — Multi-line input (textarea) | M | ⬜ |
| 6.2 | Slash command detection and tab completion | M | ⬜ |
| 6.3 | Input history navigation | S | ⬜ |
| 6.4 | Wire slash commands to existing handlers | M | ⬜ |
| 6.5 | Command output in chat view | M | ⬜ |
| 6.6 | Normal/Insert mode handling | M | ⬜ |
| 6.7 | Keybinding help overlay (?) | S | ⬜ |

**QA**: Full input experience matching REPL capabilities within TUI.

---

### Phase 7: Polish & Integration
**Goal**: Production-quality finishing touches.

| Task | Description | Effort | Status |
|------|-------------|--------|--------|
| 7.1 | Create `tui/theme.rs` — Color theme system | M | ⬜ |
| 7.2 | Scrollbar widgets on chat and sidebar | S | ⬜ |
| 7.3 | Mouse support (click, scroll) | M | ⬜ |
| 7.4 | Terminal resize handling refinement | S | ⬜ |
| 7.5 | Permission prompting in TUI mode | M | ⬜ |
| 7.6 | Error display and recovery | S | ⬜ |
| 7.7 | Session persistence integration | S | ⬜ |
| 7.8 | Performance profiling | M | ⬜ |

**QA**: Polished, production-ready TUI with all features working end-to-end.

---

## Dependency Graph

```
Phase 1 (Structural Cleanup) ~~DEFERRED~~
    ↓
Phase 2 (Ratatui Infrastructure) ← CURRENT
    ↓
Phase 3 (Core Layout & Status Bar)
    ↓
    ├── Phase 4 (Chat View) ← parallel
    └── Phase 5 (Sidebar)  ← parallel
        ↓
Phase 6 (Input & Commands)
    ↓
Phase 7 (Polish)
```

Phases 4 and 5 can be parallelized after Phase 3 completes.

## Research Notes
- Key reference: `consult-llm-mcp` repo — pulldown-cmark + syntect + ratatui markdown rendering
- `ratatui-textarea` (0.8) — multi-line input widget
- `awesome-ratatui` (1608 stars) — curated list of ratatui resources
