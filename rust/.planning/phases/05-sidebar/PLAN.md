# Phase 5: Sidebar & Task Panel

## Goal
Persistent sidebar showing tasks, files, and agent status.

## Tasks

### 5.1: tui/sidebar.rs — Sidebar container
- Create Sidebar struct implementing Component
- Manage internal tabs: Tasks, Files, Agents
- Track active tab

### 5.2: Task list widget (from todo tracking)
- Render TodoWrite items from runtime state
- Status icons: ⬚ pending, ◉ in_progress, ✓ completed
- Color coding per status

### 5.3: File change list widget
- Track files modified during session
- Show file path, change type (modified/created/deleted)
- Read from runtime's file operation history

### 5.4: Tab navigation between sections
- Tab key cycles: Tasks → Files → Agents
- Active tab highlighted
- Each tab shows its own content

### 5.5: Toggle sidebar visibility
- Ctrl+S or similar toggles sidebar
- Layout recalculates when sidebar hidden/shown
- Sidebar width configurable

### 5.6: Live updates as tools execute
- Sidebar updates as tools run
- New tasks appear as they're created
- File changes appear as files are modified

## Success Criteria
- Sidebar shows task list with status icons
- File change list updates in real-time
- Tab navigation works between sections
- Ctrl+S toggles sidebar visibility
