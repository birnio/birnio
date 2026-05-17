# Contributing to Birnio

Thanks for your interest in contributing. This guide covers everything you need to go from zero to a merged PR.

## Table of contents

- [Project structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Building](#building)
- [Running](#running)
- [Testing](#testing)
- [Code style](#code-style)
- [Commit conventions](#commit-conventions)
- [Submitting a pull request](#submitting-a-pull-request)

---

## Project structure

```
birnio/
├── crates/
│   ├── birnio-core        # Domain types and business logic
│   ├── birnio-http        # HTTP engine (request execution)
│   ├── birnio-import      # Collection importers (Bruno, Insomnia, …)
│   ├── birnio-storage     # Persistence layer and SQL migrations
│   └── birnio-ui-gtk      # GTK4 / Relm4 / libadwaita frontend (binary)
├── deny.toml              # cargo-deny policy (licences, advisories, bans)
└── codecov.yml            # Coverage configuration
```

Keep platform-agnostic logic in the library crates and GTK-specific code in `birnio-ui-gtk`. The UI crate should contain no business logic.

---

## Prerequisites

### Rust

Install via [rustup](https://rustup.rs) — the `rust-toolchain.toml` file (if present) pins the exact version automatically.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### GTK4 and libadwaita system libraries

These are native libraries that must be installed before `cargo build` will succeed.

**Ubuntu / Debian**

```sh
sudo apt-get install libgtk-4-dev libadwaita-1-dev pkg-config
```

**Fedora**

```sh
sudo dnf install gtk4-devel libadwaita-devel
```

**Arch Linux**

```sh
sudo pacman -S gtk4 libadwaita pkgconf
```

**macOS** (requires [Homebrew](https://brew.sh))

```sh
brew install gtk4 libadwaita pkg-config
```

**Windows**

GTK4 on Windows requires building the libraries from source via [gvsbuild](https://github.com/wingtk/gvsbuild):

```powershell
pip install gvsbuild
gvsbuild build gtk4 libadwaita
```

Then set the following environment variables (adjust `C:\gtk-build` if you used a different prefix):

```powershell
$env:PKG_CONFIG_PATH = "C:\gtk-build\gtk\x64\release\lib\pkgconfig"
$env:PATH = "C:\gtk-build\gtk\x64\release\bin;" + $env:PATH
```

Also install `pkgconfiglite` so Cargo can call `pkg-config`:

```powershell
choco install pkgconfiglite
```

### Optional: cargo-nextest

The CI test runner. Faster output and better failure summaries than `cargo test`.

```sh
cargo install cargo-nextest --locked
```

---

## Building

```sh
cargo build --workspace            # debug build
cargo build --workspace --release  # release build
```

The compiled binary is at `target/debug/birnio` (or `target/release/birnio` on Linux/macOS, `.exe` on Windows).

---

## Running

```sh
cargo run -p birnio-ui-gtk
```

To see debug-level tracing output:

```sh
RUST_LOG=debug cargo run -p birnio-ui-gtk
```

---

## Testing

```sh
# Run the full test suite (recommended)
cargo nextest run --workspace

# Or with the standard runner
cargo test --workspace
```

Tests that require a display server are expected to be gated behind a cfg flag or marked `#[ignore]` so they don't break headless CI.

---

## Code style

All code must pass the following checks before a PR is merged (CI enforces this):

```sh
# Format
cargo fmt --all

# Lint — warnings are errors
cargo clippy --workspace --all-targets -- -D warnings

# Dependency policy
cargo deny check
```

A few guidelines that the tools don't catch:

- Prefer `thiserror` for library error types; `anyhow` is fine in the binary crate.
- Do not add comments that restate what the code does. Add one only when the *why* is non-obvious.
- New public items in library crates should have a doc comment (`///`).

---

## Commit conventions

This project follows [Conventional Commits](https://www.conventionalcommits.org/). Every commit message must start with a type:

| Type | When to use |
|------|-------------|
| `feat` | New user-facing feature |
| `fix` | Bug fix |
| `refactor` | Code change with no behaviour change |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `docs` | Documentation only |
| `ci` | CI / workflow changes |
| `chore` | Dependency bumps, tooling, misc |

Examples:

```
feat(request-editor): add keyboard shortcut for sending request
fix(storage): prevent panic on malformed collection JSON
chore: bump relm4 to 0.10.1
```

Scope is optional but encouraged for changes that clearly belong to one crate or feature area.

---

## Submitting a pull request

1. **Fork** the repository and create a branch from `main`.
2. **Make your changes.** Small, focused PRs are easier to review and merge faster.
3. **Ensure all checks pass** locally (fmt, clippy, nextest, deny).
4. **Push** your branch and open a PR against `main`.
5. Fill in the PR template — the description and checklist help reviewers understand your intent quickly.
6. A maintainer will review your PR. CodeRabbit will also post automated feedback; address or resolve each comment before requesting a human re-review.

If you are unsure whether a change is in scope or how to approach it, open an issue first and discuss it before writing code.
