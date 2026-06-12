# CLAUDE.md

Onboarding context for Claude Code (and humans) working on **Birnio** — an
early-stage, local-first API client written in Rust with a native
GTK4/libadwaita desktop UI. The goal is a fast alternative to Bruno/Insomnia.

> If you only read one thing: the library crates (`core`, `http`, `storage`) are
> solid; the UI (`birnio-ui-gtk`) is a skeleton with **no wiring** to them yet.
> Most feature work is connecting the UI to the existing engine.

## Workspace map

```
crates/
  birnio-core      Domain types & shared data model. The stable center.
  birnio-http      HTTP execution via reqwest. Trait `HttpExecutor`.
  birnio-storage   SQLite persistence via sqlx. models + repositories pattern.
  birnio-import    Import adapters (Postman partial; Bruno/Insomnia stubbed).
  birnio-ui-gtk    GTK4/libadwaita/Relm4 desktop binary. Feature-based UI.
```

Dependency direction: `core` depends on nothing internal. `http`, `storage`,
`import` depend on `core`. `ui-gtk` depends on all four. **Keep business logic
in the library crates; `birnio-ui-gtk` must contain no business logic.**

### Crate responsibilities

- **birnio-core** — `Request`, `Response`, `Collection`/`RequestNode`,
  `Environment`, `Variable`, `Auth`, `HttpMethod`, `Body`, `Header`. All
  serde-serializable. Errors via `CoreError`/`CoreResult`. No HTTP/DB/UI deps.
- **birnio-http** — `trait HttpExecutor { fn execute(&self, &Request) -> impl
  Future<Output = HttpResult<Response>> + Send }` with `ReqwestExecutor`.
  Builds a `reqwest` request from a core `Request`, executes, parses back into a
  core `Response` (measuring `elapsed`). 60s timeout, 10 redirects, rustls-tls.
- **birnio-storage** — `connect`, `run_migrations`, `DbPool`. `models/` convert
  domain↔DB records (`from_domain`/`into_domain`); `repositories/` are async CRUD
  fns (`upsert`/`get`/`list`). Schema uses JSON columns (`nodes_json`,
  `request_json`, `variables_json`). Migration: `migrations/0001_init.sql`.
- **birnio-import** — `postman.rs` extracts only the collection name today;
  `bruno.rs`/`insomnia.rs` return `ImportError::UnsupportedFormat`.
- **birnio-ui-gtk** — feature-based UI under `src/features/<feature>/` with the
  TEA-style quartet `component.rs` / `message.rs` / `state.rs` / `mod.rs`.
  Features: `collections`, `environments`, `request_editor`, `response_viewer`,
  `settings`. Shell layout under `src/shell/`. **Currently static placeholders.**

## Current state (be honest about this)

| Crate | State |
|-------|-------|
| birnio-core | ~complete; domain model fully defined + serde roundtrip test |
| birnio-http | ~complete; execution works, tested against a local server |
| birnio-storage | ~complete; migrations + repositories with `:memory:` tests |
| birnio-import | ~20%; only Postman name extraction; Bruno/Insomnia stubbed |
| birnio-ui-gtk | ~10%; window + placeholder widgets, **no wiring** to engine |

## Conventions

- **Conventional Commits** (enforced socially, not by hook): `feat`, `fix`,
  `refactor`, `perf`, `test`, `docs`, `ci`, `chore`. Optional scope:
  `feat(request-editor): ...`. See `CONTRIBUTING.md`.
- **Errors**: `thiserror` in library crates (one `Error` enum + `Result` alias
  per crate, `#[error(transparent)]` for delegation); `anyhow` only in the binary.
- **Docs**: new public items in library crates get a `///` doc comment. Do not
  add comments that restate the code — only explain a non-obvious *why*.
- **UI architecture**: keep the `component/message/state/mod` quartet per feature.

## Quality gates (must pass before merge — CI enforces)

```sh
cargo fmt --all                                      # format
cargo clippy --workspace --all-targets -- -D warnings  # lint (warnings = errors)
cargo deny check                                     # licenses/advisories/bans
cargo nextest run --workspace                        # tests (cargo test also ok)
```

Coverage target is **60%** (codecov.yml). `assets/`, `**/migrations/`, and
`birnio-ui-gtk/src/shell/**` are excluded from coverage. Edition is **2024**.

## Developer commands (Makefile)

```sh
make help        # list commands
make fmt         # cargo fmt --all
make fmt-check   # cargo fmt --all --check
make check       # cargo check --workspace
make test        # cargo test --workspace
make test-core | test-http | test-storage | test-import   # per-crate
make check-ui    # cargo check -p birnio-ui-gtk
make run-ui      # cargo run -p birnio-ui-gtk
make tree        # dependency tree
```

> **Windows note:** building `birnio-ui-gtk` needs GTK4/libadwaita present
> (gvsbuild + `PKG_CONFIG_PATH`). The library crates build without GTK. When in
> doubt, scope checks to a single crate, e.g. `cargo check -p birnio-core`.

## How the AI harness works

This repo is set up for spec-driven, agent-assisted development:

- **`docs/`** holds the SDD (Spec Driven Development) artifacts — the source of
  truth. Read them before implementing, and keep them in sync afterward:
  - `docs/ROADMAP.md` — milestones M0–M6 (mirrors the GitHub milestones).
  - `docs/CONTRACTS.md` — internal crate/trait contracts.
  - `docs/C4_MODEL.md` — architecture (C4 + Mermaid).
  - `docs/SEQUENCE_DIAGRAM.md` — runtime interaction flows.
  - `docs/DATA_SCHEMA.md` — data model (ER + DDL).
  - `docs/BEHAVIOR.feature` — Gherkin behavior specs.
  - `docs/MADR.md` — architecture decision records.
- **`.claude/agents/`** — specialized subagents for parallel work and cross
  review: `sdd-docs`, `rust-implementer`, `rust-reviewer`, `ci-engineer`,
  `qa-tester`. Spawn them only as part of a cycle.
- **`.claude/skills/`** — user-invocable workflows (these replace slash
  commands): `pdca-milestone`, `issue-implement`, `sdd-sync`.

### PDCA loop (per milestone / issue)

1. **Plan** — read the milestone's GitHub issues (`gh`) + relevant SDD; break
   into tasks; mark which are independent (parallelizable).
2. **Do** — spawn `rust-implementer` / `ci-engineer` per independent issue, one
   git branch per issue.
3. **Check** — spawn `rust-reviewer` (cross review) and run fmt/clippy/nextest;
   confront the diff against the SDD.
4. **Act** — apply review fixes; open one PR per issue, linking the issue.

## Critical files to reference

- Domain model: `crates/birnio-core/src/{request,response,collection,environment,variable,auth}.rs`
- HTTP trait: `crates/birnio-http/src/executor.rs`
- Repositories: `crates/birnio-storage/src/repositories/*.rs`
- Schema: `crates/birnio-storage/migrations/0001_init.sql`
- Process/config: `CONTRIBUTING.md`, `Makefile`, `deny.toml`, `codecov.yml`
