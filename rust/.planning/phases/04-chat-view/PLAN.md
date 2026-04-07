# Phase 4: Chat View

## Goal
Rich conversation rendering with streaming markdown and collapsible blocks.

## Tasks

### 4.1: tui/chat_view.rs — Chat view widget
- Create ChatView struct implementing Component
- Internal state: Vec<Message>, scroll position, auto-scroll flag
- Message types: User, Assistant, System, ToolCall, ToolResult

### 4.2: Message rendering (user/assistant/system roles)
- User messages: distinct color header (e.g., blue)
- Assistant messages: green/cyan header
- System messages: gray/dim
- Each message: role badge + content

### 4.3: Streaming markdown rendering
- Use pulldown-cmark (already in deps) to parse markdown
- Render: headings (bold, larger), bold, italic, code, lists, tables
- Streaming: append new content as it arrives

### 4.4: Tool call block rendering (box-drawing)
- Tool calls rendered in bordered boxes
- Show: tool name, arguments (truncated), status icon (⏳/✓/✗)
- Use ratatui Block widget with borders

### 4.5: Collapsible tool results
- Long tool results (>N lines) shown as collapsed summary
- Click or keypress to expand
- Track collapsed state per tool call

### 4.6: Syntax highlighting in code blocks
- Use syntect (already in deps) for syntax highlighting
- Map syntect themes to ratatui colors
- Detect language from ``` fences

### 4.7: Auto-scroll with PgUp/PgDn
- Auto-scroll to bottom on new content
- PgUp/PgDn for manual scrolling
- Disable auto-scroll when user scrolls up, re-enable on scroll to bottom

### 4.8: Thinking indicator
- Show spinner/animation when waiting for assistant response
- "Thinking..." text with animated dots

### 4.9: Wire AssistantEvent stream to chat view
- Connect runtime's AssistantEvent stream to ChatView
- Handle: TextDelta, ToolUse, ToolResult, Usage, Thinking events

## Success Criteria
- Chat view renders messages with role headers
- Streaming markdown works for headings, bold, code, lists
- Tool calls shown in bordered boxes
- Long tool results collapsible
- Code blocks syntax-highlighted
- PgUp/PgDn scrolling works
- Thinking indicator shows during processing
