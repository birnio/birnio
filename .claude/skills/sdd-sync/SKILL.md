---
name: sdd-sync
description: >
  Generates or updates Birnio's Spec Driven Development artifacts in docs/
  (CONTRACTS, BEHAVIOR, C4_MODEL, SEQUENCE_DIAGRAM, DATA_SCHEMA, MADR, ROADMAP),
  derived from the code and the milestones. Use when starting a new milestone or
  when the code/contracts changed and the SDD went stale. Optional argument: the
  milestone to focus on (e.g. M1).
---

# Skill: sdd-sync

Syncs the SDD in `docs/` with the real state of the code and the milestones.

**Input:** `$ARGUMENTS` = optional milestone to focus on (e.g. `M1`). With no
argument, do a general pass.

## Steps
1. Spawn the **`sdd-docs`** subagent with the scope (milestone or general). It
   follows its 5 phases (discovery → domain → selection → generation →
   cross-validation).
2. If a milestone was passed, ask it to focus the artifacts on that scope (e.g.
   new Scenarios in `BEHAVIOR.feature`, new contracts in `CONTRACTS.md`, diagrams
   of the milestone's flow), while keeping `ROADMAP.md` globally up to date from
   GitHub (`gh api repos/birnio/birnio/milestones`).
3. On return, review the `docs/` diff and report to the user: what changed and the
   `TODO`s/inconsistencies surfaced by the cross-validation.
4. Do not commit automatically — let the user review (or ask for authorization).

## When to use in the general flow
- **Before** `pdca-milestone Mx` for a new milestone: run `sdd-sync Mx` to have the
  updated spec the implementers will follow.
- **After** contract/schema changes that escaped the inline update.
