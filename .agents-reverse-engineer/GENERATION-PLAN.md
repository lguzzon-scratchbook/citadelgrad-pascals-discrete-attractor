# Documentation Generation Plan

Generated: 2026-05-12
Project: /home/suser/DEVs/GITs/GITHUBs/citadelgrad-pascals-discrete-attractor

## Summary

- **Total Tasks**: 162
- **File Tasks**: 130
- **Directory Tasks**: 32
- **Traversal**: Post-order (children before parents)

---

## Phase 1: File Analysis (Post-Order Traversal)

### Depth 4: crates/attractor-pipeline/src/handlers/ (6 files)
- [x] `crates/attractor-pipeline/src/handlers/codergen_handler.rs`
- [x] `crates/attractor-pipeline/src/handlers/manager.rs`
- [x] `crates/attractor-pipeline/src/handlers/mod.rs`
- [x] `crates/attractor-pipeline/src/handlers/parallel.rs`
- [x] `crates/attractor-pipeline/src/handlers/tool_handler.rs`
- [x] `crates/attractor-pipeline/src/handlers/wait_human.rs`

### Depth 4: crates/attractor-cli/src/commands/ (10 files)
- [x] `crates/attractor-cli/src/commands/decompose.rs`
- [x] `crates/attractor-cli/src/commands/generate.rs`
- [x] `crates/attractor-cli/src/commands/generate.rs.bak`
- [x] `crates/attractor-cli/src/commands/info.rs`
- [x] `crates/attractor-cli/src/commands/launch.rs`
- [x] `crates/attractor-cli/src/commands/mod.rs`
- [x] `crates/attractor-cli/src/commands/plan.rs`
- [x] `crates/attractor-cli/src/commands/run.rs`
- [x] `crates/attractor-cli/src/commands/scaffold.rs`
- [x] `crates/attractor-cli/src/commands/validate.rs`

### Depth 4: crates/attractor-tools/src/builtin/ (7 files)
- [x] `crates/attractor-tools/src/builtin/edit_file.rs`
- [x] `crates/attractor-tools/src/builtin/glob.rs`
- [x] `crates/attractor-tools/src/builtin/grep.rs`
- [x] `crates/attractor-tools/src/builtin/mod.rs`
- [x] `crates/attractor-tools/src/builtin/read_file.rs`
- [x] `crates/attractor-tools/src/builtin/shell.rs`
- [x] `crates/attractor-tools/src/builtin/write_file.rs`

### Depth 4: crates/attractor-web/public/js/ (1 files)
- [x] `crates/attractor-web/public/js/xterm-setup.js`

### Depth 4: crates/attractor-web/src/components/ (10 files)
- [x] `crates/attractor-web/src/components/approval_bar.rs`
- [x] `crates/attractor-web/src/components/document_viewer.rs`
- [x] `crates/attractor-web/src/components/execution_node.rs`
- [x] `crates/attractor-web/src/components/execution_panel.rs`
- [x] `crates/attractor-web/src/components/folder_picker.rs`
- [x] `crates/attractor-web/src/components/layout.rs`
- [x] `crates/attractor-web/src/components/markdown_render.rs`
- [x] `crates/attractor-web/src/components/mod.rs`
- [x] `crates/attractor-web/src/components/project_sidebar.rs`
- [x] `crates/attractor-web/src/components/terminal.rs`

### Depth 4: crates/attractor-web/src/server/ (7 files)
- [x] `crates/attractor-web/src/server/db.rs`
- [x] `crates/attractor-web/src/server/documents.rs`
- [x] `crates/attractor-web/src/server/execute.rs`
- [x] `crates/attractor-web/src/server/mod.rs`
- [x] `crates/attractor-web/src/server/projects.rs`
- [x] `crates/attractor-web/src/server/stream.rs`
- [x] `crates/attractor-web/src/server/terminal.rs`

### Depth 3: crates/attractor-agent/src/ (6 files)
- [x] `crates/attractor-agent/src/fidelity.rs`
- [x] `crates/attractor-agent/src/lib.rs`
- [x] `crates/attractor-agent/src/loop_detection.rs`
- [x] `crates/attractor-agent/src/prompt_builder.rs`
- [x] `crates/attractor-agent/src/subagent.rs`
- [x] `crates/attractor-agent/src/test_utils.rs`

### Depth 3: crates/attractor-cli/src/ (1 files)
- [x] `crates/attractor-cli/src/main.rs`

### Depth 3: crates/attractor-pipeline/src/ (14 files)
- [x] `crates/attractor-pipeline/src/checkpoint.rs`
- [x] `crates/attractor-pipeline/src/condition.rs`
- [x] `crates/attractor-pipeline/src/edge_selection.rs`
- [x] `crates/attractor-pipeline/src/engine.rs`
- [x] `crates/attractor-pipeline/src/events.rs`
- [x] `crates/attractor-pipeline/src/goal_gate.rs`
- [x] `crates/attractor-pipeline/src/graph.rs`
- [x] `crates/attractor-pipeline/src/handler.rs`
- [x] `crates/attractor-pipeline/src/interviewer.rs`
- [x] `crates/attractor-pipeline/src/lib.rs`
- [x] `crates/attractor-pipeline/src/retry.rs`
- [x] `crates/attractor-pipeline/src/stylesheet.rs`
- [x] `crates/attractor-pipeline/src/transforms.rs`
- [x] `crates/attractor-pipeline/src/validation.rs`

### Depth 3: crates/attractor-pipeline/tests/ (1 files)
- [x] `crates/attractor-pipeline/tests/integration.rs`

### Depth 3: crates/attractor-llm/src/ (7 files)
- [x] `crates/attractor-llm/src/anthropic.rs`
- [x] `crates/attractor-llm/src/client.rs`
- [x] `crates/attractor-llm/src/gemini.rs`
- [x] `crates/attractor-llm/src/lib.rs`
- [x] `crates/attractor-llm/src/openai.rs`
- [x] `crates/attractor-llm/src/provider.rs`
- [x] `crates/attractor-llm/src/types.rs`

### Depth 3: crates/attractor-dot/src/ (4 files)
- [x] `crates/attractor-dot/src/ast.rs`
- [x] `crates/attractor-dot/src/duration_serde.rs`
- [x] `crates/attractor-dot/src/lib.rs`
- [x] `crates/attractor-dot/src/parser.rs`

### Depth 3: crates/attractor-types/src/ (1 files)
- [x] `crates/attractor-types/src/lib.rs`

### Depth 3: crates/attractor-tools/src/ (6 files)
- [x] `crates/attractor-tools/src/environment.rs`
- [x] `crates/attractor-tools/src/lib.rs`
- [x] `crates/attractor-tools/src/local_env.rs`
- [x] `crates/attractor-tools/src/profiles.rs`
- [x] `crates/attractor-tools/src/tool.rs`
- [x] `crates/attractor-tools/src/truncation.rs`

### Depth 3: crates/attractor-web/src/ (3 files)
- [x] `crates/attractor-web/src/app.rs`
- [x] `crates/attractor-web/src/lib.rs`
- [x] `crates/attractor-web/src/main.rs`

### Depth 3: crates/attractor-web/style/ (1 files)
- [x] `crates/attractor-web/style/main.scss`

### Depth 2: crates/attractor-agent/ (1 files)
- [x] `crates/attractor-agent/Cargo.toml`

### Depth 2: crates/attractor-cli/ (1 files)
- [x] `crates/attractor-cli/Cargo.toml`

### Depth 2: crates/attractor-pipeline/ (1 files)
- [x] `crates/attractor-pipeline/Cargo.toml`

### Depth 2: crates/attractor-llm/ (1 files)
- [x] `crates/attractor-llm/Cargo.toml`

### Depth 2: crates/attractor-dot/ (1 files)
- [x] `crates/attractor-dot/Cargo.toml`

### Depth 2: crates/attractor-tools/ (1 files)
- [x] `crates/attractor-tools/Cargo.toml`

### Depth 2: crates/attractor-types/ (1 files)
- [x] `crates/attractor-types/Cargo.toml`

### Depth 2: crates/attractor-web/ (3 files)
- [x] `crates/attractor-web/Cargo.toml`
- [x] `crates/attractor-web/Leptos.toml`
- [x] `crates/attractor-web/package.json`

### Depth 2: docs/examples/ (1 files)
- [x] `docs/examples/epic-runner.dot`

### Depth 2: docs/plans/ (2 files)
- [x] `docs/plans/2026-04-09-feat-reckoner-software-factory-plan.md`
- [x] `docs/plans/2026-04-14-fix-codebase-audit-outstanding-issues.md`

### Depth 1: .vscode/ (1 files)
- [x] `.vscode/settings.json`

### Depth 1: .tldr/ (2 files)
- [x] `.tldr/languages.json`
- [x] `.tldr/status`

### Depth 1: docs/ (9 files)
- [x] `docs/accept-execute-c4-dot.md`
- [x] `docs/accept-execute-c4.md`
- [x] `docs/accept-execute-flow.md`
- [x] `docs/cli-reference.md`
- [x] `docs/dot-dialect.md`
- [x] `docs/emergence-analysis.md`
- [x] `docs/guide.md`
- [x] `docs/task-verification.md`
- [x] `docs/web-interface-plan.md`

### Depth 1: pipelines/ (7 files)
- [x] `pipelines/attractor-e0n.dot`
- [x] `pipelines/attractor-kbu.dot`
- [x] `pipelines/attractor-qsb.dot`
- [x] `pipelines/attractor-s6a.dot`
- [x] `pipelines/build-beads-integration.dot`
- [x] `pipelines/phase-1-spec.dot`
- [x] `pipelines/phase-1-spec.dot.bak`

### Depth 1: templates/ (6 files)
- [x] `templates/beads.md`
- [x] `templates/epic-runner.dot`
- [x] `templates/pas.md`
- [x] `templates/plan-to-execute.dot`
- [x] `templates/prd-template.md`
- [x] `templates/spec-template.md`

### Depth 0: ./ (7 files)
- [x] `.envrc.example`
- [x] `.tldrignore`
- [x] `Cargo.toml`
- [x] `LICENSE-APACHE`
- [x] `LICENSE-MIT`
- [x] `README.md`
- [x] `install.sh`

---

## Phase 2: Directory AGENTS.md (Post-Order Traversal, 32 directories)

### Depth 4
- [x] `crates/attractor-pipeline/src/handlers/AGENTS.md`
- [x] `crates/attractor-cli/src/commands/AGENTS.md`
- [x] `crates/attractor-tools/src/builtin/AGENTS.md`
- [x] `crates/attractor-web/public/js/AGENTS.md`
- [x] `crates/attractor-web/src/components/AGENTS.md`
- [x] `crates/attractor-web/src/server/AGENTS.md`

### Depth 3
- [x] `crates/attractor-agent/src/AGENTS.md`
- [x] `crates/attractor-cli/src/AGENTS.md`
- [x] `crates/attractor-pipeline/src/AGENTS.md`
- [x] `crates/attractor-pipeline/tests/AGENTS.md`
- [x] `crates/attractor-llm/src/AGENTS.md`
- [x] `crates/attractor-dot/src/AGENTS.md`
- [x] `crates/attractor-types/src/AGENTS.md`
- [x] `crates/attractor-tools/src/AGENTS.md`
- [x] `crates/attractor-web/src/AGENTS.md`
- [x] `crates/attractor-web/style/AGENTS.md`

### Depth 2
- [x] `crates/attractor-agent/AGENTS.md`
- [x] `crates/attractor-cli/AGENTS.md`
- [x] `crates/attractor-pipeline/AGENTS.md`
- [x] `crates/attractor-llm/AGENTS.md`
- [x] `crates/attractor-dot/AGENTS.md`
- [x] `crates/attractor-tools/AGENTS.md`
- [x] `crates/attractor-types/AGENTS.md`
- [x] `crates/attractor-web/AGENTS.md`
- [x] `docs/examples/AGENTS.md`
- [x] `docs/plans/AGENTS.md`

### Depth 1
- [x] `.vscode/AGENTS.md`
- [x] `.tldr/AGENTS.md`
- [x] `docs/AGENTS.md`
- [x] `pipelines/AGENTS.md`
- [x] `templates/AGENTS.md`

### Depth 0
- [x] `./AGENTS.md` (root)
