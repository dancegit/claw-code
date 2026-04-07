# Phase 1: Structural Cleanup — Plan

## Goal
Break the 9,820-line `main.rs` monolith into focused modules.

## Extraction Map

### 1.1 → args.rs (~800 lines)
Extract from main.rs lines 93-826:
- `CliAction` enum and all variants (line 177)
- `LocalHelpTopic` enum (line 254)
- `CliOutputFormat` enum + impl (line 261)
- `parse_args()` function (line 279)
- `parse_local_help_action()` (line 484)
- `is_help_flag()` (line 498)
- `parse_single_word_command_alias()` (line 502)
- `bare_slash_command_guidance()` (line 526)
- `join_optional_args()` (line 557)
- `parse_direct_slash_cli_action()` (line 563)
- All `format_unknown_*` / `suggest_*` / `ranked_suggestions` / `levenshtein_distance` helpers (lines 615-735)
- `resolve_model_alias()` (line 739)
- `normalize_allowed_tools()` (line 748)
- `current_tool_registry()` (line 755)
- `parse_permission_mode_arg()` and related (lines 772-826)
- `parse_system_prompt_args()` (line 826)
- `parse_resume_args()` (line 861)

### 1.2 → format.rs (~700 lines)
Extract format/display functions:
- `format_unknown_slash_command_message()` (line 2001)
- `format_model_report()` (line 2018)
- `format_model_switch_report()` (line 2031)
- `format_permissions_report()` (line 2040)
- `format_permissions_switch_report()` (line 2083)
- `format_cost_report()` (line 2094)
- `format_resume_report()` (line 2110)
- `render_resume_usage()` (line 2119)
- `format_compact_report()` (line 2128)
- `format_auto_compaction_notice()` (line 2146)
- `format_status_report()` (line 4450)
- `format_sandbox_report()` (line 4552)
- `format_multiline_detail()` (line 4595)
- `format_commit_preflight_report()` (line 4612)
- `format_commit_skipped_report()` (line 4626)
- `render_config_report()` (line 4700)
- `render_memory_report()` (line 4779)
- `render_version_report()` (line 5150)
- `render_export_text()` (line 5158)
- `render_diff_report()` / `render_diff_report_for()` (lines 4851-4876)
- `render_teleport_report()` (line 4891)
- `render_last_tool_debug_report()` (line 4941)
- `indent_block()` (line 4995)
- `format_bughunter_report()` (line 5017)
- `format_ultraplan_report()` (line 5027)
- `format_pr_report()` (line 5037)
- `format_issue_report()` (line 5048)

### 1.3 → session_mgr.rs (~650 lines)
Extract session management:
- `SessionHandle` struct (line 2676)
- `ManagedSessionSummary` struct (line 2682)
- `sessions_dir()` (line 4089)
- `create_managed_session_handle()` (line 4096)
- `resolve_session_reference()` (line 4104)
- `resolve_managed_session_path()` (line 4137)
- `is_managed_session_file()` (line 4148)
- `list_managed_sessions()` (line 4156)
- `latest_managed_session()` (line 4217)
- `format_missing_session_reference()` (line 4224)
- `format_no_managed_sessions()` (line 4230)
- `render_session_list()` (line 4236)
- `format_session_modified_age()` (line 4275)
- `write_session_clear_backup()` (line 4293)
- `session_clear_backup_path()` (line 4302)

### 1.4 → app.rs (LiveCli + support types, ~2500 lines)
Extract:
- `LiveCli` struct (line 2692)
- `RuntimePluginState` (line 2701)
- `RuntimeMcpState` + impl (lines 2708-3006)
- `BuiltRuntime` + impl (lines 2715-2793)
- `ToolSearchRequest`, `McpToolRequest`, `ListMcpResourcesRequest`, `ReadMcpResourceRequest` (lines 2794-2816)
- `build_runtime_mcp_state()` (line 3008)
- `mcp_runtime_tool_definition()` (line 3027)
- `mcp_wrapper_tool_definitions()` (line 3045)
- `permission_mode_for_mcp_tool()` (line 3095)
- `mcp_annotation_flag()` (line 3109)
- `HookAbortMonitor` (line 3117)
- `impl LiveCli` (lines 3172-4088) — all methods

### 1.5 — tui/ namespace (ALREADY EXISTS)

### 1.6 — Verification
- `cargo fmt`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

## Dependencies
All extractions modify main.rs so must be sequential. Order:
1. args.rs first (cleanest boundary, fewest dependencies)
2. session_mgr.rs (session types used by LiveCli)
3. format.rs (format functions used by LiveCli, status, etc.)
4. app.rs last (LiveCli depends on all above)

## Success Criteria
- [ ] main.rs reduced to ~200-300 lines (entrypoint dispatch only)
- [ ] 4 new module files created
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] All public API preserved (no behavior changes)
