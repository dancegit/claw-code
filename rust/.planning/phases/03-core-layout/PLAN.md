# Phase 3: Core Layout & Status Bar

## Goal
Implement the full split-pane layout with live status bar.

## Tasks

### 3.1: tui/layout.rs — Enhanced layout definitions
- Current layout is basic. Enhance with:
  - Proper border handling (borders take space)
  - Configurable sidebar width
  - Responsive breakpoints (hide sidebar below certain width)

### 3.2: tui/status_bar.rs — Live status bar widget
- Create dedicated status bar component implementing Component trait
- Display: model name, token count, cost, session ID, git branch, elapsed time
- Accept data updates from app state (model, tokens, etc.)
- Color-coded sections

### 3.3: tui/components.rs — Enhanced component trait
- The current Component trait is minimal. Consider:
  - `fn on_mount(&mut self)` / `fn on_unmount(&mut self)` lifecycle hooks
  - `fn as_any(&self) -> &dyn Any` for downcasting
  - Keep it simple — only add what's actually needed

### 3.4: Wire status bar into app render loop
- Replace placeholder draw_status_bar with real StatusBar component
- Pass app state to status bar for live updates

### 3.5: Responsive resize handling
- TuiApp should track current terminal size
- On resize event, recalculate layout
- Below 60 cols: auto-hide sidebar
- Below 40 cols: minimal layout

### 3.6: Live token counter updates
- Token counts come from AssistantEvent::Usage
- App state tracks cumulative input/output tokens
- Status bar reads from app state

## Success Criteria
- Status bar shows: model name, token count (0), cost ($0.00), session info
- Layout adapts to terminal resize
- Sidebar hides automatically on narrow terminals
