# Birnio Roadmap

This roadmap is organized **backend-first**. The frontend (GTK/libadwaita) track
is currently unstaffed, so all UI work is consolidated into a single milestone
and deliberately deferred until backend contracts are stable and a frontend
owner is available. In the meantime, the CLI is promoted to the interim product
surface so the backend can be exercised and shipped end-to-end without the UI.

## Execution order

Backend phases run in this order. Milestone numbers are historical labels;
this document is the source of truth for sequencing.

| Phase | Milestone | Focus | Issues |
|-------|-----------|-------|--------|
| 1 | M0 - Workspace Foundation | Contributor docs, licensing, CI hygiene | [#6](https://github.com/birnio/birnio/issues/6), [#10](https://github.com/birnio/birnio/issues/10), [#15](https://github.com/birnio/birnio/issues/15), [#45](https://github.com/birnio/birnio/issues/45) |
| 2 | M1 - First Request Execution | Reliable HTTP engine (headers, bodies, timing, config) | [#2](https://github.com/birnio/birnio/issues/2), [#46](https://github.com/birnio/birnio/issues/46) |
| 3 | M2 - Collections and Persistence | HAR workspace contract, loader/writer, environments, variable interpolation, SQLite retirement | [#5](https://github.com/birnio/birnio/issues/5), [#18](https://github.com/birnio/birnio/issues/18), [#19](https://github.com/birnio/birnio/issues/19), [#20](https://github.com/birnio/birnio/issues/20), [#23](https://github.com/birnio/birnio/issues/23), [#41](https://github.com/birnio/birnio/issues/41), [#42](https://github.com/birnio/birnio/issues/42), [#43](https://github.com/birnio/birnio/issues/43) |
| 4 | M3 - Importers | Postman, Bruno, Insomnia imports into the HAR workspace | [#24](https://github.com/birnio/birnio/issues/24), [#25](https://github.com/birnio/birnio/issues/25), [#26](https://github.com/birnio/birnio/issues/26) |
| 5 | M5 - CLI Preview | `birnio-cli` as the interim product surface (validate, send, import) | [#7](https://github.com/birnio/birnio/issues/7), [#44](https://github.com/birnio/birnio/issues/44) |
| 6 | M6 - Secrets and Secure Storage | Secret storage design, secret references in core, security docs | [#31](https://github.com/birnio/birnio/issues/31), [#32](https://github.com/birnio/birnio/issues/32), [#34](https://github.com/birnio/birnio/issues/34) |
| — | M4 - Native GTK UX | **Frontend track — on hold** (see below) | [#1](https://github.com/birnio/birnio/issues/1), [#3](https://github.com/birnio/birnio/issues/3), [#4](https://github.com/birnio/birnio/issues/4), [#17](https://github.com/birnio/birnio/issues/17), [#21](https://github.com/birnio/birnio/issues/21), [#22](https://github.com/birnio/birnio/issues/22), [#27](https://github.com/birnio/birnio/issues/27), [#28](https://github.com/birnio/birnio/issues/28), [#29](https://github.com/birnio/birnio/issues/29), [#30](https://github.com/birnio/birnio/issues/30), [#33](https://github.com/birnio/birnio/issues/33) |

## Frontend track (M4) — on hold

All `area:ui-gtk` issues live in **M4 - Native GTK UX**, regardless of which
milestone they originally belonged to. They are not abandoned: the backend
phases intentionally keep clean crate boundaries (`birnio-core`, `birnio-http`,
`birnio-storage`, `birnio-import`) so the GTK app can be built on top later
without rework.

When a frontend owner joins, the suggested order within M4 is:

1. [#17](https://github.com/birnio/birnio/issues/17) — Relm4/libadwaita application architecture
2. [#1](https://github.com/birnio/birnio/issues/1) — Request editor state flow
3. [#3](https://github.com/birnio/birnio/issues/3) — Wire Send button to birnio-http
4. [#4](https://github.com/birnio/birnio/issues/4) — Render response status, headers, body and timing
5. [#21](https://github.com/birnio/birnio/issues/21) / [#22](https://github.com/birnio/birnio/issues/22) — Workspace open and save flows
6. [#27](https://github.com/birnio/birnio/issues/27) / [#28](https://github.com/birnio/birnio/issues/28) — Request and response tabs
7. [#29](https://github.com/birnio/birnio/issues/29) / [#30](https://github.com/birnio/birnio/issues/30) / [#33](https://github.com/birnio/birnio/issues/33) — Copy as curl, layout toggle, secret UX

By then the backend will already provide HTTP execution, HAR persistence,
environments with interpolation, importers, and a CLI proving the whole flow —
so the UI work becomes presentation, not plumbing.

## Notes on "First Request Execution" (M1)

M1 originally meant sending a request from the GTK UI. With the frontend on
hold, the first end-to-end request execution ships through the CLI instead
(M5), built on the same `birnio-core` + `birnio-http` engine hardened in M1.
