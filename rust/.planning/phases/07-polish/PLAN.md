# Phase 7: Polish & Integration

## Goal
Production-quality finishing touches.

## Tasks

### 7.1: tui/theme.rs — Color theme system
- Define Theme struct with semantic colors
- Default dark theme
- Colors for: primary, secondary, success, error, warning, dim, accent
- Border styles: normal, focused, active

### 7.2: Scrollbar widgets on chat and sidebar
- Add scrollbar indicators on chat view and sidebar
- Use ratatui::widgets::Scrollbar
- Show current position and total content

### 7.3: Mouse support (click, scroll)
- Click on sidebar items to select
- Scroll in chat view and sidebar
- Click to expand/collapse tool results

### 7.4: Terminal resize handling refinement
- Smooth resize without flickering
- Preserve scroll position on resize
- Minimum terminal size warning

### 7.5: Permission prompting in TUI mode
- Render permission prompts in TUI (not stdout)
- Allow/deny via keypress (y/n)
- Show command details in prompt

### 7.6: Error display and recovery
- Error banner at top of chat view
- Auto-dismiss after timeout
- Error details expandable

### 7.7: Session persistence integration
- Save session on exit
- Resume sessions in TUI mode
- Session list in sidebar

### 7.8: Performance profiling
- Benchmark render loop for large conversations
- Lazy rendering for off-screen messages
- Optimize markdown parsing for streaming

## Success Criteria
- Theme system with dark default
- Scrollbars visible and functional
- Mouse click and scroll work
- Clean resize handling
- Permission prompts work in TUI mode
- Error display non-intrusive
- Sessions persist and resume
- Performance acceptable with 1000+ messages
