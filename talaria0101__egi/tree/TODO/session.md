# Session

## T-011: Agent protocol: JSONL framing, commands, settlement

**Source:** errand `src/agent/protocol.ts` plus `framing.ts` plus `client.ts` (`634b7b9`).
**Category:** session
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

The daemon and the agent exchange commands and events over pipes. Without
a framed protocol with explicit settlement, a turn's end is ambiguous and
an automatic retry releases resources too early.

### Premise

READ, not measured. errand implements this protocol against `pi --mode
rpc`; egi keeps the wire shape and drives its own agent loop instead.

### Approach

Create `crates/egi-session` depending on `egi-core`:

- Port the framer from `src/agent/framing.ts` (read whole file):
  newline-delimited records, size cap with a typed error, no silent
  truncation.
- Port the protocol surface from `src/agent/protocol.ts` lines 10-208:
  `AgentRecord`, `AgentCommand`, `DialogRequest`/`DialogResponse` with
  `select`/`confirm`/`input`/`editor` methods, `isFireAndForget`,
  `messageText`, `messageRole`, `toolTarget`, thinking markers,
  `usageOf`. Keep the exact command set; read lines 54-66 for the
  command union.
- Port the client state machine from `src/agent/client.ts` lines 46-175:
  `AgentState` (`starting`, `ready`, `working`, `ended`), `AgentProcess`
  abstraction (stdout/stderr streams, write, exited), `AgentHandlers`
  for every outward event. Turn completion comes from settlement, not
  from process end: an end event may precede an automatic retry, so
  releasing an admission slot on it over-admits.
- Keep the stderr-kept cap (2000 chars) and the affirmative/negative
  dialog vocabularies at lines 107-175.

⛔ What it must not do: release admission on process end; swallow a
protocol violation and continue; block a turn on a dialog no thread can
serve (report `onUnsupportedDialog` instead).

### Prove

```bash
cargo test -p egi-session
```

Passing: framer tests (split records, oversize refused, no truncation);
state-machine tests (retry does not release, violation ends the session,
dialog vocabularies answered); fake-process client test, no sandbox.

### Closing

Not closed.

---

## T-012: Session manager with registry, transcript journal, disk use

**Source:** errand `src/session/manager.ts` plus `registry.ts` plus `record.ts` plus `transcript.ts` plus `disk.ts` plus `files.ts` (`634b7b9`).
**Category:** session
**Priority:** P1
**Effort:** L
**Status:** open

---

### Problem

Several sessions run at once across threads. The daemon must own their
lifetimes, journal every turn to disk, bound their disk use, and resume a
thread into the same conversation.

### Premise

READ, not measured. errand implements all of this; the entry ports it
crate by crate.

### Approach

Build in `crates/egi-session`:

- Port `SessionManager` from `src/session/manager.ts` lines 127-306:
  start outcomes (`StartOutcome`, lines 32-36), `ThreadFactory` and
  `ManagerOptions` (lines 37-126), one sandbox per session, thread to
  session binding. Read the whole file with `src/session/session.ts`
  (2147 lines, the session itself) before decomposing; that file splits
  into the session struct, the turn loop, and the tool dispatch, each its
  own module here.
- Port the registry from `src/session/registry.ts` (237 lines): live
  session tracking, orphan matching for teardown.
- Port the record layout from `src/session/record.ts` lines 20-47
  (`recordDir`, `transcriptPath`, `prepareRecordDir`) and the transcript
  from `src/session/transcript.ts` lines 18-26 plus the journal writer:
  append-only JSONL, every turn with its usage line, scrubbed of secrets
  before write.
- Port disk accounting from `src/session/disk.ts` (97 lines) and file
  placement from `src/session/files.ts` (178 lines): measure project
  plus state against the budget on a tick; passing it ends the session.
- Resume continues the conversation stored in the state directory
  (`--continue` shape in errand's `agentCommand`), not a new
  conversation sharing a directory.

⛔ What it must not do: keep cross-request state in module-level memory
(more than one instance exists); write an unscrubbed secret to the
transcript; resume into a fresh conversation.

### Prove

```bash
cargo test -p egi-session
```

Passing: manager tests (two sessions isolated, orphan reaped, resume
continues); journal tests (append, scrub, replay); disk tests (over
budget ends the session, tick paces itself).

### Closing

Not closed.
