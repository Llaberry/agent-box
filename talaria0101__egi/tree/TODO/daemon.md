# Daemon

## T-015: Daemon startup: config load, probe, report, refuse

**Source:** errand `src/daemon.ts` plus `src/config/` (`634b7b9`).
**Category:** daemon
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

The daemon holds the chat token and starts sandboxes. Starting with a
guarantee it cannot keep (a backend gap, a bad config) runs sessions
under a boundary that does not hold.

### Premise

READ, not measured. errand refuses to start rather than run degraded
unless the operator accepted it; egi keeps that posture.

### Approach

Create `crates/egi-daemon` depending on all library crates:

- Port the config surface from `src/config/schema.ts` lines 12-470:
  `SandboxBackend`, `NetworkMode`, `EgressMode`, `EgressConfig`,
  `ChatConfig`, `AgentConfig`, `SandboxConfig`, `Config`, `SECRET_PATHS`
  (line 411), `DEFAULTS` (lines 414-469). Keep `SECRET_PATHS` beside
  the schema: it names what redaction must cover.
- Port config discovery from `src/config/load.ts` lines 19-126:
  `CONFIG_VARIABLE`, search order, outright naming, JSONC support.
  Port validation from `src/config/validate.ts` lines 1-786 (read
  whole; it is long because every field is checked).
- Port the startup sequence from `src/daemon.ts` lines 35-202:
  `EnforcementGapError` (line 35), `createSandbox` (line 53),
  `inertSettings` (line 74), `renderStartupReport` (lines 88-130),
  `probeSandbox` (lines 131-147), the `Daemon` class from line 148.
  Probe before touching chat. Report gaps plainly. Refuse unless
  `requireFullEnforcement` is false AND the operator read the report.
- Write `config.example.json` with fake values for every key, mirroring
  errand's example, and generate the JSON schema from the Rust types so
  the two cannot drift.

⛔ What it must not do: start with a gap unreported; fall back to
unconfined; log a secret value at startup (names only).

### Prove

```bash
cargo test -p egi-daemon
```

Passing: config tests (example validates, bad field refused with a
readable message, secret paths covered); startup tests (gap refuses by
default, degraded mode starts only with the explicit flag, report names
every additive grant).

### Closing

Not closed.

---

## T-016: Admission queue bounding concurrent provider turns

**Source:** errand `src/admission/scheduler.ts` (306 lines).
**Category:** daemon
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

A burst of messages across sessions can flood the model provider and get
the account rate limited. Unbounded concurrency also multiplies spend
without bound.

### Premise

READ, not measured. errand's scheduler is small and well tested; the port
keeps its fairness shape.

### Approach

- Read `src/admission/scheduler.ts` end to end (306 lines with tests)
  before writing. Port the queue: bounded concurrent turns
  (`maxConcurrentTurns`), bounded live sessions (`maxLiveSessions`),
  FIFO with per-session fairness, idle timeout (`idleMs`) reaping.
- Settlement releases the slot (T-011), not process end. A retry holds
  its slot. Say so in the module docs; it is the invariant the next
  reader will question.
- Startup timeout (`startupMs`) kills a sandbox that never becomes
  ready, loudly.

⛔ What it must not do: admit past the cap during retries; starve a
session; reap a working session as idle.

### Prove

```bash
cargo test -p egi-daemon
```

Passing: scheduler tests (cap held under burst, fairness across
sessions, idle reap, startup timeout). Mirror errand's scheduler tests.

### Closing

Not closed.

---

## T-017: Web observer with loopback-only bind

**Source:** errand `src/web/server.ts`, `address.ts`, `view.ts`, `src/serve.ts`.
**Category:** daemon
**Priority:** P2
**Effort:** M
**Status:** open

---

### Problem

An operator wants to watch sessions as they happen and browse the
project. A web surface with a login is a second auth system; without one
the bind address is the access control.

### Premise

READ, not measured. errand's answer (loopback-only, public bind refused)
ports directly.

### Approach

- Port address validation from `src/web/address.ts` (read whole): a
  public bind is refused rather than warned about. Loopback only,
  unless `observer` mode explicitly widens read-only access.
- Port the server from `src/web/server.ts` (read whole) and views from
  `src/web/view.ts`: session stream reading, project browsing, new
  session start. Observer mode watches and reads but changes nothing.
- Port the memory store shape from `src/memory/store.ts` (269 lines,
  `MEMORY_FILENAME` at `src/serve.ts` line 40) for the system-prompt
  append: memory costs the agent no tool call and no round trip.

⛔ What it must not do: bind publicly; serve observer mode with a live
action reachable; show a read-only user a live button (the one-gated-door
class: enforce on every path, and the driven pass must try the button).

### Prove

```bash
cargo test -p egi-daemon
```

Passing: bind tests (public bind refused, loopback admitted); observer
tests (every mutating route refused in observer mode); driven pass notes
in the closing (which button was clicked, what happened).

### Closing

Not closed.

---

## T-018: GitHub integration behind brokered bindings

**Source:** errand `src/session/github.ts` (220 lines) plus `pr.ts` (456 lines) (`634b7b9`), agent-sandbox 1580 plus agent-vault 255 (sweep 2, sweep 4).
**Category:** daemon
**Priority:** P2
**Effort:** M
**Status:** open

---

### Problem

Working on somebody's repository means reading issues, checking builds,
and opening pull requests. Each needs the GitHub token, which the agent
must never hold.

### Premise

READ, not measured. errand hands the agent a GitHub token in the session
environment; egi must NOT repeat that. Every GitHub call crosses the
broker on a scoped binding instead.

### Approach

- Read `src/session/github.ts` and `src/session/pr.ts` end to end for
  the operation set (issue read, build check, PR open with generated
  diff), then re-derive each as a broker binding: exact host
  (`api.github.com`), exact paths per operation, short-lived token
  minted per session (GitHub App installation tokens, per-repo and
  per-permission scoped).
- Identity binding: the sandbox cannot forge which credential set it
  gets. Bind by session id at the broker, minted by the daemon, never
  by a value the session offers. Agent-sandbox issue 1580
  (execution-scoped authorization: credentials outliving their execution
  in reusable sandboxes) is the reason lifetimes are per-execution, not
  per-sandbox. Agent-vault issue 255 (GitHub App credential type for
  proxy-side installation-token injection) sets the token kind.
  Per-tenant scoping falls out: identical
  image, different bindings per session.
- Session lifecycle: mint at start, rotate on resume, revoke at end.
  A revoked session's nonce authenticates nothing.

⛔ What it must not do: place the token in the session environment (the
errand shape this rewrite exists to remove); scope a token wider than
the session's task; leave a token valid after the session ends.

### Prove

```bash
cargo test -p egi-daemon
```

Passing: binding tests (each operation matches exactly one binding,
wider paths refused); lifecycle tests (revoked nonce gets 401, resumed
session gets a fresh token); no-token-in-env test (session environment
fixture contains no GitHub credential).

### Closing

Not closed.
