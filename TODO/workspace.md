# Workspace

The crates, the shared types, and the binary that wires them.

[`INDEX.md`](INDEX.md) is the list. [`RULES.md`](RULES.md) is how this
repository is worked on.

---

## T-001: Cargo workspace with a strictly downward crate graph

**Source:** `QaidVoid/kage` `docs/reference/architecture.md` (`6ad2708`), synthesised in [`../docs/history/references/orchestration/usable.md`](../docs/history/references/orchestration/usable.md).
**Category:** workspace
**Priority:** P1
**Effort:** S
**Status:** open

---

### Problem

There is no code. Every other entry needs somewhere to put it, and the shape
chosen now decides whether a later session can tell where something belongs
without asking.

### Premise

READ, not measured. `kage` states the rule at
`crates/../docs/reference/architecture.md`, under "crate graph": **"Layering is
strict: depend only downward. The `kage-cli` binary is the only crate that wires
the whole graph together."** Its own graph is eleven crates for an agent with a
terminal interface; this project needs fewer.

⚠ The claim that this shape works is inherited, not tested here. What is
checkable is that the rule is mechanical: a dependency pointing upward is a
failed check rather than a review comment.

### Approach

Create the workspace with these members and no others:

```
box-core                         leaf: ids, errors, cancel, shared wire types. No IO.
box-policy  box-broker  box-sandbox    depend on box-core
box-session                      depends on box-policy + box-sandbox
box-cli                          the binary, depends on everything it uses
xtask                            the gate, the probe, the miner. Not shipped.
```

⛔ **`box-plugin` is not created by this entry.** T-070 creates it, and only
once there is something to extend. A crate with no caller is machinery.

- `rust-toolchain.toml` pins the toolchain. Record the exact version in this
  entry's closing, because CI gains lints with every release and a local
  toolchain behind CI produces a green run that goes red on push
  ([`../docs/conventions/git.md`](../docs/conventions/git.md) section 6).
- Workspace lints: `unsafe_code = "forbid"` at the workspace level, overridden
  in exactly the crates that need it, each with a comment naming what it needs
  it for. ⚠ There may be none.
- `clippy::pedantic` on, with denials rather than warnings in CI.

⛔ **What it must not do:** create a crate for a second implementation that does
not exist, add an async runtime, or add any dependency beyond what a bare
workspace needs. [`../docs/conventions/code.md`](../docs/conventions/code.md)
is the rule, and `kage`'s architecture page carries a worked argument for
staying synchronous that this project should read before deciding otherwise.

### Decision

**Async or synchronous.** `kage` is synchronous and argues for it: the agent
loop is human-paced, providers expose blocking iterators, and adding a runtime
forces every layer to colour-async.

⭐ **Recommendation: the broker is async, everything else is synchronous.** The
broker serves many concurrent tunnels with long idle periods, which is the one
workload in this project that a thread per connection handles badly. The
alternative, threads throughout, loses on the broker and wins everywhere else;
the split costs one crate boundary that already exists.

⚠ **Not ruled.** The operator rules before T-020 starts, and T-020 cannot start
without it.

### Prove

```bash
cargo build --workspace --all-targets && cargo clippy --workspace --all-targets -- -D warnings
```

Passing: exit 0 on both, with every crate above present in
`cargo metadata --format-version 1` and no member outside the list.

### Closing

Not closed.

---

## T-002: Shared types in box-core, with no IO

**Source:** `QaidVoid/errand` `src/sandbox/backend.ts` (`58a178b`), read at [`../docs/history/references/errand/usable.md`](../docs/history/references/errand/usable.md).
**Category:** workspace
**Priority:** P1
**Effort:** S
**Status:** open
**Blocked by:** T-001

---

### Problem

Three crates need the same identifiers, path constants and error shapes. Without
one home for them, each grows its own and the two diverge in the way
[`../docs/conventions/forbidden-patterns.md`](../docs/conventions/forbidden-patterns.md)
names: a value in two places with no check that they agree.

### Premise

READ, not measured. errand keeps these in `src/sandbox/backend.ts`, at these
exact lines:

| what | lines |
| --- | --- |
| `WORKSPACE_PATH`, `STATE_PATH`, `AGENT_HOME`, `AGENT_BIN`, `AGENT_SESSIONS` | 14-38 |
| the discovery label, the session label, the name prefix | 40-47 |
| `sandboxName` | 156-158 |

The doc comment at lines 14-25 carries the reason the paths are constants and
not host paths, and it is the reason to copy: **"The agent is never shown a host
path. A path carries the operator's name and the shape of their machine, which
is not access the sandbox can take back once the agent has read it, and it would
then appear in anything the agent writes or says."**

### Approach

In `box-core`:

- `SessionId`, a newtype over a string, constructed only from a cryptographic
  random source. ⛔ Never a timestamp, never a counter
  ([`../docs/conventions/code.md`](../docs/conventions/code.md)).
- The path constants above, as `&'static str`, with errand's comment ported as
  the reason.
- `BackendName`, an enum, not a string.
- A cancellation flag.
- The error types, typed and converted at the boundary. ⛔ Never a bare string
  error.

⛔ **No IO in this crate**, and the check in T-052 asserts it: no `std::fs`, no
`std::net`, no `std::process`.

⛔ **No secret type lives here.** The credential store's types belong to
`box-broker`, so that a crate that cannot hold a secret structurally cannot.

### Prove

```bash
cargo test -p box-core && cargo xtask check layering
```

Passing: exit 0 on both. `check layering` reports `box-core` importing neither
`std::fs`, `std::net` nor `std::process`, and reports no workspace dependency
pointing upward.

⚠ `cargo xtask check layering` does not exist until T-052. Until it does, this
entry closes with the test alone and **stays partial**, not done.

### Closing

Not closed.

---

## T-003: The binary, its subcommands, and refusing an unknown configuration key

**Source:** [`../docs/architecture.md`](../docs/architecture.md), "Configuration"; `QaidVoid/errand` `src/config/validate.ts` (`58a178b`).
**Category:** workspace
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-001, T-002

---

### Problem

An operator needs one command to run the daemon, one to inspect the host, and
one to see what a session was given. Without a configuration loader that refuses
what it does not understand, a misspelled key reads as a setting in force.

### Premise

READ, not measured. errand rejects unknown keys per section:
`src/config/validate.ts:60-90` builds the known-key sets and
`rejectUnknown(...)` is called per section, for example at `:637` for the egress
block. It refuses rather than warning.

⚠ Its configuration is JSON with a published schema
(`config.schema.json`, 429 lines). This project uses TOML; the schema question
is a decision below.

### Approach

`box-cli`, with these subcommands and no others until something asks:

| command | does |
| --- | --- |
| `agent-box run` | the daemon, until told to stop |
| `agent-box doctor` | the same probe the daemon runs at startup, printed and exited |
| `agent-box sessions` | list, inspect and remove what past sessions left |

Configuration loading:

- search order per [`../docs/architecture.md`](../docs/architecture.md),
  "Configuration", with `AGENT_BOX_CONFIG` naming one outright;
- ⛔ **an unknown key is an error naming the key and the section**, never a
  warning;
- ⛔ **a value of the wrong type is an error at load**, never at use. The
  exhibit is CubeSandbox 1458, where a policy the admin API accepted with 200
  produced an uncaught error on every subsequent request and the operator saw
  failures with no indication that what they installed was malformed;
- the loaded configuration is printed by `doctor` with every secret-shaped field
  replaced by its name. ⛔ **Never a prefix of a value**
  ([`../docs/security/secrets.md`](../docs/security/secrets.md)).

### Decision

**Ship a published schema, or not.**

⭐ **Recommendation: yes, generated from the types by `xtask`, and checked.** It
gives an editor completion and it is a second reader of the same shape. The cost
is one generator and one check that the committed file matches the types; the
alternative is a schema that drifts, which is worse than none.

### Prove

```bash
cargo test -p box-cli config::
```

Passing: exit 0, with named tests covering a misspelled key at each section
depth (refused, naming the key), a wrong-typed value at load (refused), the
documented search order on both platforms, and `doctor` output containing the
name of a secret-shaped field and no part of its value.

### Closing

Not closed.
