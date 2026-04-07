# State — Claw TUI Migration

## Current Status
- **Branch**: feat/ratatui-tui
- **Phase**: Phase 2 (Ratatui Infrastructure) — IN PROGRESS
- **Blockers**: None

## What's Done
- TUI skeleton exists (~660 lines across 6 files in tui/)
- ratatui 0.30 + ratatui-textarea 0.8 + crossterm 0.28 in Cargo.toml
- tui/terminal.rs — Terminal lifecycle (raw mode, alternate screen, restore on drop, panic hook)
- tui/events.rs — Async event handler (keyboard, mouse, resize, tick, custom via mpsc)
- tui/app.rs — TuiApp with render loop, Ctrl+Q quit, placeholder UI
- tui/layout.rs — Split-pane layout (status bar, sidebar, chat, input)
- tui/components.rs — Component trait with default implementations
- tui/mod.rs — Module exports, TuiConfig, TuiError types

## Decisions
- **Phase 1 DEFERRED**: Structural cleanup (extract main.rs 9,820 lines) is too complex for automated extraction. Attempted twice — agents failed due to massive cross-reference web. main.rs extraction is a nice-to-have refactoring, NOT a blocker for TUI work. Revisit after TUI is functional.
- TUI skeleton will be extended, not rewritten
- Existing modules (init.rs, input.rs, render.rs) are NOT modified
- `pub(crate)` visibility needed on LiveCli for tui/ access

## Phase 2 Remaining Work
1. Add `--tui` flag to CliAction::Repl and parse_args()
2. Create `tui::run_tui()` entry point that wraps LiveCli
3. Make LiveCli + BuiltRuntime `pub(crate)` visible
4. Wire TuiApp to use LiveCli for conversation turns
5. Verify: `claw --tui` launches alternate screen, Ctrl+Q exits cleanly

## Phase 3+ Notes
- Librarian research completed: found ratatui chat patterns, pulldown-cmark+syntect integration, collapsible sections, ratatui-textarea examples
- Key reference repos: consult-llm-mcp (markdown rendering), rust-chat-server, awesome-ratatui (1608 stars)
- Phases 3-7 plan files written in .planning/phases/

## Session Log
- [session 1] Assessed codebase: main.rs = 9,820 lines, TUI skeleton exists, build clean
- [session 2] Attempted Phase 1 extraction twice — both failed (truncation, duplicate code not removed, agent timeout)
- [session 3] Deferred Phase 1, started Phase 2 (partial --tui flag work), librarian research for Phases 3-7
