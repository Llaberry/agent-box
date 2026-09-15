# TODO

Every entry, one line each, sorted by id. The entry itself lives in the
`TODO/<category>.md` file its row links to, and it closes there with its own
acceptance command, actually run, with the output recorded.

⛔ **What to work on next is not here.** [`PROGRESS.md`](PROGRESS.md)'s
"Start here next session" is the work order and is the only file that carries
one. This file carries the list, the definitions, the counts, and the argument
behind the current ordering.

[`RULES.md`](RULES.md) is how this repository is worked on.
[`../docs/methodology/work-todo.md`](../docs/methodology/work-todo.md) is the
model. [`../docs/AGENTS.md`](../docs/AGENTS.md) routes sessions.

---

## Priority

- **P0** breaks correctness, loses data, or takes the process down.
- **P1** a documented capability does not work, or a flag does nothing.
- **P2** worth doing; nothing is wrong without it.
- **P3** worth recording so it is not rediscovered.

## Effort

S is under a day. M is a few days. L is a week. XL is longer, and ⚠ is almost
always two entries pretending to be one.

## Status

`open`, `partial`, `blocked`, `done`. ⛔ There is no `wontfix` and no
`deferred`: a blocked entry stays open with the blocker named and what would
unblock it.

---

## The ordering, and the argument behind it

The broker and the sandbox contract come first, because every other crate
builds on them and the security story is the reason this project exists.
Provider bindings come next, then sessions and chat, then the daemon and the
hardening passes. Docs and repo hygiene ride along from the start rather
than as a final sweep, so each entry keeps its own pages true.

Reference commits are pinned per entry at `docs/history/references/pins.md`.
Re-mine a reference when implementing against it: the pins name the commit
the entry was written against, and projects move.

---

## Counts

**26 items. 1 done, 0 partial, 0 blocked, 25 open.**

| priority | open | partial | blocked | done | total |
| --- | --- | --- | --- | --- | --- |
| P0 | 0 | 0 | 0 | 0 | 0 |
| P1 | 11 | 0 | 0 | 1 | 12 |
| P2 | 12 | 0 | 0 | 0 | 12 |
| P3 | 2 | 0 | 0 | 0 | 2 |
| **All** | 25 | 0 | 0 | 1 | 26 |

---

## Entries

| ID | Priority | Category | Status | Item |
| --- | --- | --- | --- | --- |
| [T-001](broker.md) | P1 | broker | open | CONNECT broker that gates hosts and injects provider credentials |
| [T-002](broker.md) | P1 | broker | open | Binding table with exact-match rules and fail-closed semantics |
| [T-003](sandbox.md) | P1 | sandbox | open | Sandbox backend trait plus capability probing |
| [T-004](sandbox.md) | P1 | sandbox | open | Bailey backend driver with policy generation |
| [T-005](sandbox.md) | P1 | sandbox | open | Podman backend driver with measured network flags |
| [T-006](sandbox.md) | P2 | sandbox | open | Windows/WSL2 backend report and refusal rules |
| [T-007](provider.md) | P1 | provider | open | Provider registry with provider colon model addressing |
| [T-008](provider.md) | P1 | provider | open | Streaming client with SSE parsing and usage accounting |
| [T-009](provider.md) | P2 | provider | open | models.dev catalog generation through an xtask |
| [T-010](provider.md) | P2 | provider | open | Delegate (cheap model) path with budget and deadline |
| [T-011](session.md) | P1 | session | open | Agent protocol: JSONL framing, commands, settlement |
| [T-012](session.md) | P1 | session | open | Session manager with registry, transcript journal, disk use |
| [T-013](chat.md) | P1 | chat | open | Chat seam plus Discord adapter with reconnect policy |
| [T-014](chat.md) | P2 | chat | open | Thread commands, diffs, and redaction before delivery |
| [T-015](daemon.md) | P1 | daemon | open | Daemon startup: config load, probe, report, refuse |
| [T-016](daemon.md) | P2 | daemon | open | Admission queue bounding concurrent provider turns |
| [T-017](daemon.md) | P2 | daemon | open | Web observer with loopback-only bind |
| [T-018](daemon.md) | P2 | daemon | open | GitHub integration behind brokered bindings |
| [T-019](meta.md) | P2 | meta | open | xtask with ascii, todo-count, and catalog guards |
| [T-020](meta.md) | P2 | meta | open | experiments directory with first containment probes |
| [T-021](meta.md) | P2 | meta | open | CHANGELOG with entries for the first shipped units |
| [T-022](meta.md) | P1 | meta | done | Reference sweeps filed under docs/history/references |
| [T-023](hardening.md) | P2 | hardening | open | Landlock ABI gating and fail-closed version checks |
| [T-024](hardening.md) | P2 | hardening | open | Exfiltration-through-authorized-channels mitigations |
| [T-025](hardening.md) | P3 | hardening | open | gVisor/Kata backend shape behind the same trait |
| [T-026](hardening.md) | P3 | hardening | open | Deep review pass over the skeleton and the plan |
