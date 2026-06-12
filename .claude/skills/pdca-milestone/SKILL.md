---
name: pdca-milestone
description: >
  Orchestrates the PDCA cycle (Plan-Do-Check-Act) to close a Birnio milestone
  using specialized agents in parallel and cross-review. Use when the user asks to
  implement/close a milestone (e.g. "run PDCA on M0", "close milestone 1").
  Argument: the milestone identifier (e.g. M0).
---

# Skill: pdca-milestone

Closes a Birnio milestone with an orchestrated PDCA cycle. You (in the main chat)
are the orchestrator; the heavy lifting goes to subagents (`rust-implementer`,
`ci-engineer`, `qa-tester`) and the review to `rust-reviewer`.

**Input:** `$ARGUMENTS` = the milestone identifier (e.g. `M0`). If empty, ask
which milestone.

## Plan
1. Resolve the milestone on GitHub and list its open issues:
   `gh api repos/birnio/birnio/milestones` (find the internal number) and
   `gh issue list --milestone "<full title>" --state open`.
2. Read the relevant SDD artifacts in `docs/` (ROADMAP, CONTRACTS, DATA_SCHEMA,
   SEQUENCE_DIAGRAM, BEHAVIOR). For each issue, read `gh issue view <n>` and
   extract the acceptance criteria.
3. Build a task plan (use TaskCreate): one task per issue. Mark **dependencies**
   between issues (e.g. contract before implementation) and which are
   **independent** (parallelizable). Pick the agent by label: `area:ci` →
   `ci-engineer`; code → `rust-implementer`; coverage → `qa-tester`.
4. **Flag to the user** decisions that are theirs (e.g. on M0, the Homebrew tap
   token/PR in #10; the branch-protection change in #15) and confirm before
   proceeding on those.

## Do
For each independent issue, in parallel when possible:
1. Create a dedicated branch from `main` (1 branch/issue). Consider an isolated
   worktree (`isolation: "worktree"` on Agent) for issues running in parallel that
   touch nearby files.
2. Spawn the appropriate agent (`rust-implementer`/`ci-engineer`/`qa-tester`) with:
   the issue number, the branch, and pointers to the SDD. Dependent issues only
   start once their blockers finish.

## Check
1. For each delivered branch, spawn `rust-reviewer` (**cross**-review — never the
   same agent that implemented it) with the diff and the issue.
2. Run the gates: `cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo nextest run --workspace` (scope to crates if GTK is unavailable in the
   environment — record what was skipped).
3. Confront the diff with the SDD: a changed contract/schema/behavior requires the
   corresponding `docs/` artifact to be updated.

## Act
1. **blocker**/**should-fix** findings go back to the implementer (same branch)
   until the reviewer approves.
2. With the branch green and approved, **open 1 PR per issue** (`gh pr create`)
   linking the issue (`Closes #<n>`), filling the template
   (`.github/pull_request_template.md`). Opening a PR is an external action —
   confirm with the user before the first `gh pr create` if they have not
   authorized it in bulk.
3. Update the tasks (TaskUpdate) and report: PRs opened, what is left pending, and
   any maintainer decision still open.

## Rules
- Implementer and reviewer are **different** agents (true cross-review).
- 1 branch + 1 PR per issue; small, focused PRs.
- Commits and PRs only with the user's authorization (not auto-approved).
- Keep `docs/` in sync with what was implemented.
