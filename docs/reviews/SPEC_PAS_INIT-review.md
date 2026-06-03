---
spec: SPEC-005 — Automated Project Initialization and Declarative Quality Loops (`pas init`)
spec_path: /Users/scott/Downloads/SPEC_PAS_INIT.md
reviewer: claude-opus-4-7
date: 2026-05-29
verdict: needs_revision_before_implementation
---

# Review of SPEC-005 (`pas init` + `pas::quality`)

## Summary

The SPEC is directionally sound: detect toolchain → emit `pas.toml` → consume it in a `pas::quality` handler that retries on failure. The data model and the failure-feedback loop are well-motivated.

Before this is implementable, four P1 gaps need to be closed (most importantly the **missing-`pas.toml` behavior** Scott flagged), three integration assumptions need to be reconciled with the existing codebase, and the schema needs to grow a few fields it implicitly requires.

**P1 (blocks implementation):** 4
**P2 (should fix before merge):** 6
**P3 (nice-to-have):** 3

---

## P1 — Critical Gaps (block implementation)

### P1-1 — Missing `pas.toml` behavior is undefined (Scott's concern)

§4.2 step 1 says *"Reads and validates `pas.toml` from the current working directory."* It does not say what happens when the file is missing, malformed, or located in a parent directory (monorepo case). Three things can call into this code path with different reasonable behaviors:

| Caller | Reasonable behavior on missing `pas.toml` |
|---|---|
| `pas run <pipeline>` where pipeline contains a `pas::quality` node | **Hard error** at handler invocation — the node cannot meaningfully run |
| `pas run <pipeline>` where pipeline has no `pas::quality` node | **No warning** — the file is irrelevant |
| `pas validate <pipeline>` | **Warning** if the pipeline references `pas::quality` and no `pas.toml` exists at workdir |
| `pas init` re-run | **Prompt before overwrite** if file already exists |

**Recommended addition to SPEC §4.2:**

> **4.2.0 Manifest Resolution**
>
> The handler resolves `pas.toml` by walking from `--workdir` upward to the filesystem root (stopping at the first `.git` directory or `Cargo.toml`/`pyproject.toml`/`package.json` workspace root). The first `pas.toml` found wins.
>
> - **Not found:** the handler returns a `Fail` outcome with `system_guidance = "No pas.toml found at <searched paths>. Run 'pas init' to generate one."` The pipeline engine surfaces this as a structured error, not a panic.
> - **Parse error (malformed TOML):** the handler returns `Fail` with `last_error_log = "<toml parse diagnostic>"`. No fallback to defaults — silent fallbacks have hidden a category of bugs we've already fixed elsewhere.
> - **Schema validation error (missing required keys):** same as parse error.
>
> **Run-time warning:** When `pas run` loads a pipeline, it scans for `handler="pas::quality"` nodes. If any are present and no `pas.toml` can be resolved at the workdir, it emits a single warning at startup:
> `WARN: pipeline uses pas::quality but no pas.toml found at <workdir>. Run 'pas init' to generate one.`
> The pipeline still starts (so dry-runs and partial executions work), but the quality node will fail when reached.

This is the only place in the SPEC where the failure mode is ambiguous enough to drive divergent implementations. It needs to be pinned down.

---

### P1-2 — `cmd` execution from `pas.toml` has no trust model

§4.2 step 3 executes arbitrary `cmd` strings from `pas.toml` via `std::process::Command`. `pas.toml` is intended to be checked in. The implied flow is: `git clone <repo> && pas run <pipeline>` — at which point an attacker-controlled `pas.toml` can run arbitrary commands.

This is not theoretical for a CLI whose explicit purpose is to run AI-generated and template-generated code in untrusted contexts. The SPEC needs to specify one of:

- **Explicit trust prompt** on first `pas run` in a directory whose `pas.toml` hash is not in `~/.config/pas/trusted.json`. (Same pattern as VS Code Workspace Trust, direnv `allow`, mise `trust`.)
- **`--trust` flag** that must be passed for any run that resolves a previously-unseen `pas.toml`.
- **Allowlist of binaries** (`cargo`, `npm`, `npx`, `ruff`, `mypy`, `pytest`, `prettier`, `eslint`, `tsc`, `vitest`, `jest`) with non-allowlisted invocations requiring `--allow-arbitrary-cmd`.

I'd recommend the trust-prompt pattern: it matches developer muscle memory from `direnv`, and `pas init` can register trust automatically since the user just authored the file. The SPEC should also state that `cmd` is parsed as a single shell string (via `sh -c`) vs. argv (via `Command::new`) — currently ambiguous and the answer changes the attack surface (shell injection vs. exec-only).

---

### P1-3 — §5.1 telemetry JSON is malformed; the contract is not parseable

The example in §5.1 has a multi-line string literal inside a JSON value:

```json
"last_error_log": "error[E0308]: mismatched types
  --> src/main.rs:42:18
   |
42 |     let x: String = 100;
```

JSON strings cannot contain raw newlines — they must be `\n` escaped. As written, this is not valid JSON and any LLM downstream consumer will silently misparse it.

**Action:**
1. Re-render the example with `\n`-escaped newlines, or switch the spec example to a YAML/TOML representation if readability is the goal.
2. Add a sentence: *"`last_error_log` is JSON-string-encoded; newlines, tabs, and quotes are escaped per RFC 8259."*
3. Add a max-bytes guarantee at the contract level (currently `truncate_logs_after_bytes` lives under `[quality.telemetry]` config — restate it as a hard contract guarantee on the output schema, otherwise downstream consumers can't rely on it).

---

### P1-4 — Stage ordering is not specifiable in TOML

§3 declares `[quality.hooks]` with `format`, `lint`, `typecheck`, `test`. §4.2 step 2 says they execute in that order. But:

- TOML inline tables (and `[quality.hooks]` as written here) are unordered maps. `serde` will deserialize into a `HashMap`/`BTreeMap` and ordering becomes either lexical or insertion-order-dependent on the parser.
- The SPEC asserts a specific order (`format → lint → typecheck → test`) without making it user-configurable. A user who needs `typecheck` before `lint` (because their linter assumes types are valid) has no escape hatch.
- Hard-coding four well-known stage names also blocks anyone who wants a fifth stage (e.g., `audit` for `cargo audit`, `coverage`, `e2e`).

**Recommended schema change:**

```toml
[quality]
stages = ["format", "lint", "typecheck", "test"]   # required, ordered

[quality.hooks.format]
cmd = "cargo fmt --check"
allow_failure = false

[quality.hooks.lint]
cmd = "cargo clippy --all-targets -- -D warnings"
allow_failure = false
# ... etc
```

Move each hook to a sub-table so additional fields (timeout, env, cwd, retry) can be added without breaking the schema. The `[quality.stages]` array makes order explicit and extensible.

---

## P2 — Should Fix Before Implementation

### P2-1 — `serde_toml` is not the actual crate name

§3 says *"strongly-typed Rust parser (`serde_toml`)"*. The crate that exists is [`toml`](https://crates.io/crates/toml) (which has a `serde` feature). `serde_toml` doesn't exist on crates.io. Replace the reference with `toml` (currently `0.8.x`).

### P2-2 — "Two-phase" header contradicts three phases; non-interactive mode missing

§2 opens with *"two-phase initialization process"* then lists Phase 1, 2, **and 3**. Phase 3 is *"Interactive TUI Confirmation"*. That contradicts the SPEC's own earlier line: *"a user **or autonomous workspace builder** executes `pas init`"*.

Two fixes:
1. Rename the section to "three-phase" or fold the TUI under Phase 2 as an optional confirmation step.
2. Specify a `--yes` / `--non-interactive` flag for autonomous mode that skips Phase 3 and writes the manifest directly. Also specify behavior when an existing `pas.toml` is present in non-interactive mode (default: refuse to overwrite without `--force`).

Specify the TUI library too — `ratatui`, `dialoguer`, or a custom impl? This affects dependency footprint significantly.

### P2-3 — `allow_failure = true` semantics undefined

§3 shows `allow_failure = true` only in the TypeScript template's `format` stage. §4.2 step 4 only describes the `allow_failure = false` branch. Add a sentence:

> When `allow_failure = true`, a non-zero exit code emits a `WARN`-level log entry but the engine proceeds to the next stage. The node final status is `pass` if all `allow_failure = false` stages succeed.

### P2-4 — Polyglot detection contradicts single `primary_language`

§2 Phase 2 explicitly handles *"a polyglot monorepo or a Python backend with a TypeScript frontend"*. §3 then offers only `primary_language = "rust" | "python" | "typescript" | "custom"`. A polyglot repo has no single "primary" language.

Either:
- Document `primary_language = "custom"` as the polyglot escape hatch (and show a polyglot example with per-stage `cwd`).
- Switch to `[[toolchain]]` (array-of-tables) so a repo can declare multiple toolchains with separate hook tables per language.

### P2-5 — Loop control: "same error footprint" and iteration binding are ambiguous

§5.2 has two ambiguities:
1. *"the same error footprint"* — what is a footprint? A hash of `last_error_log`? Of the diff between iterations? Of stage name + exit code? Without a definition, two implementations will diverge.
2. *"loop counter bound to the specific target node ID"* — is the target the `pas::quality` node or the upstream `codergen` node that's looping back? §5.2 step 1 implies the latter ("loops back to `pas::quality`"), but then `max_fix_iterations = 3` reads as "max 3 quality runs" not "max 3 codergen runs".

Define:
- **Footprint:** `(failed_stage, sha256(last_error_log)[..16])` is a reasonable default. Documents what changes invalidate the counter.
- **Binding:** The counter is keyed on the `pas::quality` node ID and resets when a different upstream node sends control flow into it. (Or document the alternative.)

Also: `backoff_factor = 1.5` is defined but never referenced in prose — what does it back off? If it's sleep-between-retries, say so (`sleep = base * factor^iteration`).

### P2-6 — No spec for handler registration or async/threading model

The SPEC introduces `handler="pas::quality"` but doesn't describe how it plugs into the existing `crates/attractor-pipeline/src/handlers/manager.rs`. Existing handlers (`codergen`, `wait_human`, `parallel`) use simple lowercase names without `::`. Either:
- Document that `pas::*` is a reserved namespace for built-in handlers and update the dispatch logic accordingly.
- Use a lowercase name (`pas_quality` or just `quality`) that fits the existing convention.

Separately: §4.2 says *"triggers the internal quality runner thread"* but the engine is `tokio` async (`#[tokio::main]` in `crates/attractor-cli/src/main.rs`). Spawn a tokio task, not an OS thread. `std::process::Command` should be `tokio::process::Command` to avoid blocking the runtime during long test runs.

---

## P3 — Nice-to-Have

### P3-1 — Unused fields in the example

`[toolchain] min_version` and `package_manager` appear in the example but aren't referenced anywhere in §4 or §5. Either explain how they're used (e.g., guard the run with a `rustc --version` check, gate `cmd` template variables on package manager) or drop them.

### P3-2 — Missing per-stage `env`, `cwd`, and `timeout`

The hook table is `{cmd, allow_failure}` only. Realistic projects need:
- `cwd` — monorepo subprojects (`apps/web`, `crates/foo`)
- `env` — `CI=1`, `RUST_BACKTRACE=1`, `PYTHONDONTWRITEBYTECODE=1`
- `timeout_secs` — long test suites should be bounded; the existing pipeline runner already has timeout tiers (see `docs/cli-reference.md` "Timeout tiers" table).

Add these to the schema now — adding them later is a breaking schema change.

### P3-3 — Phase 2 LLM enrichment trigger and payload need narrowing

§2 Phase 2 fires *"if multiple toolchains are detected … or if configurations are missing"*. "Configurations are missing" is vague — missing from what? The payload — *"top-level source files, project structure, and file extensions"* — risks leaking secret-looking filenames (`.env.production.local`, `secrets.yaml`). Recommend:
- Specify exact detection trigger conditions as a decision table.
- Document a filename allowlist for the LLM payload (exclude `.env*`, `*.key`, `*.pem`, `secrets/*`, etc.).
- Make Phase 2 opt-out via `pas init --no-agent` for users who can't send paths to a third-party LLM.

---

## Cross-Cutting Notes

- **Checkpoint interaction:** The engine already has `crates/attractor-pipeline/src/checkpoint.rs`. If a pipeline fails at `pas::quality` and the user re-runs (resume from checkpoint), does the loop counter reset to 0? The SPEC should answer this — silent counter reset would let a runaway loop hide behind a checkpoint resume.
- **Naming:** Crate names use `attractor-*`, binary is `pas`, SPEC uses `pas` and `pas::quality`. Consistent for the user-facing surface, but if `pas::quality` lives in a new crate, decide whether it's `attractor-quality` (matches the workspace pattern) or `pas-quality` (matches the SPEC narrative). Not blocking — just call it out before naming.
- **Existing CLI surface:** `pas init` is not in the current `Commands` enum (`crates/attractor-cli/src/main.rs`). New subcommand, no conflicts. Add to `docs/cli-reference.md` in the same PR.

---

## Recommended Next Steps

1. **Revise SPEC §4.2** to specify missing/malformed/parent-dir resolution (P1-1).
2. **Add a §6 (Trust Model)** to address `cmd` execution risk (P1-2).
3. **Re-render §5.1 example** as valid JSON or YAML; document the contract (P1-3).
4. **Restructure `[quality.hooks]`** with explicit `stages` array (P1-4) — this is the most consequential schema change, easier to do now than after v1.
5. **Reconcile the existing handler convention** before naming `pas::quality` (P2-6).
6. After revisions, this is ready to decompose into Linear tasks.
