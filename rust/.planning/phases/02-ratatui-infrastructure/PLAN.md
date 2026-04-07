# Phase 2: Ratatui Infrastructure — Gap Fill

## Status: ~70% Complete

## What's Done
- ✅ ratatui 0.30 + ratatui-textarea 0.8 in Cargo.toml
- ✅ tui/terminal.rs — Terminal lifecycle (raw mode, alternate screen, restore on drop, panic hook)
- ✅ tui/events.rs — Async event handler (keyboard, mouse, resize, tick, custom via mpsc)
- ✅ tui/app.rs — TuiApp struct with render loop, key handling, placeholder UI
- ✅ tui/layout.rs — Split-pane layout (status bar, sidebar, chat, input)
- ✅ tui/components.rs — Component trait with default implementations
- ✅ tui/mod.rs — Module exports, TuiConfig, TuiError types

## Remaining Work

### 2.5: Add --tui flag to arg parser
- Add `--tui` boolean flag to CLI argument parsing
- Should be available as a top-level flag: `claw --tui` or `claw --tui prompt "hello"`
- When --tui is set, the REPL uses TUI mode instead of rustyline

### 2.6: Wire TUI mode in main entry point
- In the `run()` or `run_repl()` function, detect --tui flag
- If set: create TuiApp and call .run() instead of the rustyline REPL
- Ensure clean error handling if terminal doesn't support TUI

## Success Criteria
- `cargo run --bin claw -- --tui` shows full-screen alternate screen
- Ctrl+Q exits cleanly and restores terminal
- `cargo test --workspace` passes
- `cargo clippy --workspace --all-targets -- -D warnings` clean
