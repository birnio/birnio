---
name: issue-implement
description: >
  Implements a single Birnio issue with the lean PDCA cycle (plan → implement →
  cross-review → PR). Use when the user wants to work a specific issue (e.g.
  "implement #45", "resolve issue 6"). Argument: the issue number.
---

# Skill: issue-implement

Single-issue version of the PDCA cycle. Good for targeted work outside a full
milestone sweep.

**Input:** `$ARGUMENTS` = the issue number (e.g. `45`). If empty, ask.

## Steps
1. **Plan.** `gh issue view <n>`; read the acceptance criteria and the relevant
   SDD artifacts in `docs/`. Identify the affected crates and the right agent by
   label (`area:ci` → `ci-engineer`; code → `rust-implementer`).
2. **Do.** Create a dedicated branch from `main`. Spawn the appropriate agent with
   the issue number, the branch, and pointers to the SDD.
3. **Check.** Spawn `rust-reviewer` (a different agent from the implementer) with
   the diff and the issue. Run the gates (`fmt --check`, `clippy -D warnings`,
   `nextest`; scope to crates if GTK is unavailable). Confront with the SDD.
4. **Act.** Apply the blocker/should-fix findings until approved. Open 1 PR
   (`gh pr create` with `Closes #<n>` and the template) — **confirm with the
   user** before `gh pr create`. Update `docs/` if any contract/behavior changed.

## Rules
- Implementer ≠ reviewer (cross-review).
- Commit/PR only with the user's authorization.
- Scope restricted to the issue; no scope creep.
