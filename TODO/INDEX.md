# TODO

Every entry, one line each, sorted by id. The entry lives in the category file
its row links to, and it closes there with its own acceptance command, actually
run, with the output recorded.

⛔ **What to work on next is not here.** [`PROGRESS.md`](PROGRESS.md)'s "Start
here next session" is the work order and is the only place that carries one.
This file carries the list, the definitions, the counts, and the argument behind
the current ordering.

[`RULES.md`](RULES.md) is how this repository is worked on.
[`../docs/methodology/work-todo.md`](../docs/methodology/work-todo.md) is the
model. [`../docs/AGENTS.md`](../docs/AGENTS.md) routes sessions.

⭐ **The counts below are derived from the rows, never typed.**

```bash
cargo xtask record set T-NNN done
```

```bash
cargo xtask check record
```

⚠ **Neither command exists yet.** T-054 builds both, and until it does the
counts are maintained by hand and are the most likely thing in this file to be
wrong.

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

⭐ **Three entries come before everything, and none of them is the broker.**

**T-030, T-031 and T-046 are P0**, and what they share is that each is about
code running where it should not:

- **T-030**, the daemon running a tool against state the agent controls, which
  was measured to be host code execution as the daemon's user with no sandbox
  escape;
- **T-031**, a trusted event stream the session can write to, which made forged
  events reach a transcript as genuine agent output;
- **T-046**, the harness loading TypeScript extensions out of the repository it
  is working on, which runs with the harness's own permissions.

⛔ **All three are shape decisions**, so building anything else first means
rebuilding it. Two were measured against a running deployment and the third is
stated by the harness's own security page.

**Then the workspace, then the sandbox contract, then the broker.** T-001 to
T-003 exist so there is somewhere to put code. T-010 to T-013 are the
confinement, and T-013 is where the project's central claim lives: one route off
the host. T-020 to T-024 are the broker, and T-021 is the entry the project
exists for.

**The checks ride along rather than arriving at the end.** T-052 to T-054 are P1
because the rules in [`../docs/`](../docs/) are unenforced until they exist, and
a rule enforced by nobody becomes a preference. ⚠ The record check in particular
is holding the counts in this file, by hand, right now. T-056 is P1 for a
different reason: a session that has to set its own machine up by hand sets it
up differently each time.

**T-064 is P1 and it is the one that is easy to defer.** Every containment claim
this project makes today is somebody else's measurement on somebody else's
machine. Until the battery runs, [`../docs/limits.md`](../docs/limits.md) is a
page about other projects.

⭐ **The surfaces come after the boundary, and that is the whole argument for
this ordering.** T-090 rules that the target is full parity with the project
this replaces and then more, so the channel, the interface and the metrics are
all in scope. ⛔ **Every one of them is a way to reach a session**, and a session
that is not confined makes all of them worse. ⚠ **A parity target is a reason to
build more, never a reason to build it in a weaker order.**

**Within the surfaces, T-100 and T-102 are P1 and the rest are P2.** The channel
is the primary interface for this deployment, and work landing in a repository
is what the deployment is for. The web interface is genuinely secondary, and
⛔ **it is the one that adds a second trusted surface**, so it goes after the
broker rather than beside it.

**T-042 and T-043 are P1 because they are not optional here.** The deployment has
a small fixed number of provider slots and more people than slots. Without the
pool and the queue the answer to a busy provider is an error, a user retries,
and a burst becomes a rate-limit ban for everybody on a shared subscription.

**The open questions are P2, not P3**, because they are the honest part. T-027
and T-028 are unsolved at every reference read for this project, and recording
them as low priority would be recording them as unimportant. T-017 is P2 for the
same reason: it is a measurement that decides a real capability, and the
operator ruled that it is measured before it is decided.

⭐ **T-040 and T-090 are the two `done` entries, and neither shipped code.** Both
were rulings the operator made in the session that filed them, and both unblocked
several entries. ⚠ **An entry whose deliverable is a decision closes when the
decision is recorded**, which is why the counts show two done against no
implementation.

---

## Counts

**58 items. 56 open, 0 partial, 0 blocked, 2 done.**

| priority | open | partial | blocked | done | total |
| --- | --- | --- | --- | --- | --- |
| P0 | 3 | 0 | 0 | 0 | 3 |
| P1 | 27 | 0 | 0 | 2 | 29 |
| P2 | 22 | 0 | 0 | 0 | 22 |
| P3 | 4 | 0 | 0 | 0 | 4 |
| **All** | 56 | 0 | 0 | 2 | 58 |

---

## Entries

| ID | Priority | Category | Status | Item |
| --- | --- | --- | --- | --- |
| [T-001](workspace.md) | P1 | workspace | open | Cargo workspace with a strictly downward crate graph |
| [T-002](workspace.md) | P1 | workspace | open | Shared types in box-core, with no IO |
| [T-003](workspace.md) | P1 | workspace | open | The binary, its subcommands, and refusing an unknown configuration key |
| [T-010](sandbox.md) | P1 | sandbox | open | The backend trait and the capability report |
| [T-011](sandbox.md) | P1 | sandbox | open | A probe that attempts the real setup, and a gap that stops the daemon |
| [T-012](sandbox.md) | P1 | sandbox | open | Per-session policy generation |
| [T-013](sandbox.md) | P1 | sandbox | open | The bailey backend, locked to one broker |
| [T-014](sandbox.md) | P2 | sandbox | open | The podman backend on Linux |
| [T-015](sandbox.md) | P2 | sandbox | open | Podman in a machine, and the four Windows host checks |
| [T-016](sandbox.md) | P2 | sandbox | open | Interactive devices, so a terminal program works |
| [T-017](sandbox.md) | P2 | sandbox | open | Measure whether a session can run containers, then decide |
| [T-020](broker.md) | P1 | broker | open | The CONNECT proxy |
| [T-021](broker.md) | P1 | broker | open | The injection gate |
| [T-022](broker.md) | P1 | broker | open | Session scope resolution, which never retargets |
| [T-023](broker.md) | P1 | broker | open | The binding table, validated at admission and failing closed at use |
| [T-024](broker.md) | P1 | broker | open | The certificate authority, which authorizes before it signs |
| [T-025](broker.md) | P2 | broker | open | Substitutions, with per-surface encoding |
| [T-026](broker.md) | P1 | broker | open | The credential store, and failing closed two ways |
| [T-027](broker.md) | P2 | broker | open | Binding a destination address to an allowed name |
| [T-028](broker.md) | P2 | broker | open | Execution-scoped credentials |
| [T-029](broker.md) | P2 | broker | open | Detecting a session that is not using the broker |
| [T-030](session.md) | P0 | session | open | The daemon never runs a tool against agent-writable state |
| [T-031](session.md) | P0 | session | open | A trusted channel the session cannot forge into |
| [T-032](session.md) | P1 | session | open | Teardown, and orphan discovery at startup |
| [T-033](session.md) | P2 | session | open | Diagnostics good enough to work without kernel audit |
| [T-040](agent.md) | P1 | agent | done | Drive the harness, own the protocol boundary |
| [T-041](agent.md) | P2 | agent | open | A generated model catalogue |
| [T-042](provider.md) | P1 | provider | open | The provider slot pool |
| [T-043](provider.md) | P1 | provider | open | Admission, the queue, and telling a user where they are in it |
| [T-044](provider.md) | P2 | provider | open | Usage windows, and knowing when one resets |
| [T-045](provider.md) | P2 | provider | open | A vendor CLI as a provider, in its own sandbox |
| [T-046](provider.md) | P0 | provider | open | Pin the harness's project-trust setting |
| [T-047](provider.md) | P1 | provider | open | The harness adapter, and its framing |
| [T-050](tooling.md) | P1 | tooling | open | `cargo xtask doctor` |
| [T-051](tooling.md) | P1 | tooling | open | `cargo xtask gate` |
| [T-052](tooling.md) | P1 | tooling | open | The document and structure checks |
| [T-053](tooling.md) | P1 | tooling | open | The secret and attribution checks |
| [T-054](tooling.md) | P1 | tooling | open | `cargo xtask check record` |
| [T-055](tooling.md) | P2 | tooling | open | `cargo xtask mine` |
| [T-056](tooling.md) | P1 | tooling | open | One bootstrap command that prepares a session's environment |
| [T-060](docs.md) | P2 | docs | open | Bind addresses, and refusing a public bind |
| [T-061](docs.md) | P2 | docs | open | Write the limits page from measurement rather than from reading |
| [T-062](docs.md) | P3 | docs | open | Document the metadata-write limit where a user meets it |
| [T-063](docs.md) | P2 | docs | open | The Windows page |
| [T-064](docs.md) | P1 | docs | open | The containment battery |
| [T-070](plugins.md) | P3 | plugins | open | The capability model, and ruling on the extension language |
| [T-071](plugins.md) | P3 | plugins | open | Host-applied privileged requests, with a veto |
| [T-090](scope.md) | P1 | scope | done | Full parity with the project this replaces, then more |
| [T-091](scope.md) | P3 | scope | open | When a shared kernel stops being enough |
| [T-100](surface.md) | P1 | surface | open | The chat gateway, one channel, a thread per session |
| [T-101](surface.md) | P2 | surface | open | Thread commands, and the status a user can ask for |
| [T-102](surface.md) | P1 | surface | open | Work that lands in a repository, composed by the daemon |
| [T-110](surface.md) | P2 | surface | open | The web interface, with chat-service login |
| [T-111](surface.md) | P2 | surface | open | Watching a session, and starting one from the web |
| [T-112](surface.md) | P2 | surface | open | Metrics where a person looks |
| [T-120](metrics.md) | P1 | metrics | open | Usage accounting at the broker |
| [T-121](metrics.md) | P2 | metrics | open | Cost, derived from the catalogue and carrying its date |
| [T-122](metrics.md) | P2 | metrics | open | What a stuck session looks like |

---

## ⚠ What this index does not have

⛔ **No entry has been implemented, and no premise here has been measured by
this project.** Every "MEASURED" premise in an entry is somebody else's
measurement, on a machine this project has never seen, and each says so.

⭐ **T-064 is the entry that changes that**, and until it closes, every
containment claim in [`../docs/`](../docs/) is a reading.
