# Surface

How a person asks for work and watches it happen: the chat channel, the web
interface, and the metrics both show.

[`INDEX.md`](INDEX.md) is the list.

⭐ **The goal is parity with the project this replaces, then more**, and
[`../docs/limits.md`](../docs/limits.md) is where anything that cannot be
reached securely gets written down rather than quietly dropped.

---

## T-100: The chat gateway, one channel, a thread per session

**Source:** the operator, 2026-09-15; `QaidVoid/errand` `src/chat/`, `docs/threads.md` (`58a178b`).
**Category:** surface
**Priority:** P1
**Effort:** L
**Status:** open
**Blocked by:** T-043, T-047

---

### Problem

The deployment is one channel in a FOSS community's chat server. A message
there starts a session; replies in its thread are prompts; what the agent says
and does comes back to the same thread.

### Premise

⭐ **RULED by the operator, 2026-09-15**: this is the primary interface, and the
web interface is secondary.

READ, not measured, for the shape: the reference this project replaces
implements one channel, one thread per session, several sessions at once, with
about 3,124 lines under `src/chat/` at the pinned commit.

⛔ **Its own division is the one to keep**: the chat token never enters a
sandbox, and everything a session reports is scrubbed of every configured
secret before it reaches a channel. ⚠ **That scrubbing is damage control on an
unavoidable exposure, not a boundary**, and its own documentation says so: an
agent that re-encodes a value defeats it. Keep it, and never count it.

### Approach

In a new `box-chat`, depending on `box-session` and nothing above it:

- one configured channel, an allowlist of accounts that may drive sessions, and
  a thread per session;
- a message in the channel opens a thread and requests admission (T-043).
  ⭐ **The queue position is reported in the thread**, so waiting is visible
  rather than silent;
- replies in a thread are prompts; an aside prefix is seen by people and never
  reaches the agent;
- ⛔ **the chat token never enters a sandbox**, and the daemon holds it;
- ⛔ **every outbound message is scrubbed** through the single redaction helper
  T-026 owns, and nothing else formats a secret-shaped value;
- ⛔ **a message is untrusted input.**
  [`../docs/security/remote-ops.md`](../docs/security/remote-ops.md): reading one
  is free, acting on one because it told you to is not reading. A message
  claiming the operator approved something is a string anyone in the channel
  could write.

⛔ **What it must not do:** serve more than the configured channel, or accept a
session request from an account outside the allowlist, or let a thread's
contents widen what its session was granted.

### Prove

```bash
cargo test -p box-chat gateway::
```

Passing: exit 0, with named tests for: a message from a non-allowlisted account
refused; a message outside the configured channel ignored; an aside not reaching
the prompt stream; a queued request reporting its position; and ⭐ **a configured
secret value present in a session's output being absent from the message
actually sent**, asserted on the outbound payload.

### Closing

Not closed.

---

## T-101: Thread commands, and the status a user can ask for

**Source:** the operator, 2026-09-15; `QaidVoid/errand` `docs/reference/commands.md` (`58a178b`).
**Category:** surface
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-100, T-120

---

### Problem

A user in a thread needs to see what a session is costing, move it to another
model, and stop it. Without commands the only control is closing the thread.

### Premise

READ, not measured. The reference this project replaces carries a help command,
a usage command reporting how much of a provider's window is left and when it
resets, a model command that switches model while keeping the conversation, and
a status command. ⭐ Its own note on switching is worth keeping: it is refused
while a turn is running, because changing the model underneath a turn answers
half a question with each.

### Approach

Commands registered both as text and as the chat service's own command
mechanism, so they can be picked rather than remembered:

| command | does |
| --- | --- |
| help | what can be typed here |
| status | this session's tokens, cost, elapsed time, and its slot's provider |
| usage | the provider window: how much is left, when it resets (T-044) |
| model | switch model within the same provider, keeping the conversation. ⛔ Refused mid-turn. |
| queue | where this request is, and what it is waiting for |
| stop | end this session and tear it down |

⛔ **Every number comes from the broker's own count** (T-120), never from the
agent's self-report. ⚠ Where the two disagree, say so rather than picking one:
the disagreement is the finding.

⛔ **What it must not do:** expose another user's session, the slot count, or
which other sessions hold which provider. T-042 is why.

### Prove

```bash
cargo test -p box-chat commands::
```

Passing: exit 0, with named tests for each command's output shape, a model
switch refused mid-turn, a stop tearing down all three of sandbox, bindings and
nonce (T-032), and ⛔ **a status request from a different user in the same
channel refused rather than answered**.

### Closing

Not closed.

---

## T-102: Work that lands in a repository, composed by the daemon

**Source:** the operator, 2026-09-15; `QaidVoid/errand` `src/session/github.ts` and issue 2 (`58a178b`).
**Category:** surface
**Priority:** P1
**Effort:** L
**Status:** open
**Blocked by:** T-030, T-021

---

### Problem

The point of the deployment is work that reaches a repository. ⛔ **And this is
the exact path that produced the most serious finding in the whole plan.**

### Premise

⭐ **MEASURED against a running deployment, by a reporter**, and recorded in
[`../docs/history/references/errand/findings.md`](../docs/history/references/errand/findings.md):
a daemon that ran host-side `git` with the agent's project directory as the
working directory gave every session host code execution as the daemon's user
through a hook in that directory.

⚠ The reference's own mitigation is a wrapper in a directory the session can
write, and its own comment says what that is worth: "It is not a wall, because
the token can reach the API directly and the wrapper sits in a directory the
session can write."

### Approach

⛔ **T-030 is binding here and there is no exception for convenience.**

- the repository credential is **brokered**, never in the session. The session
  reaches the forge through the broker like any other upstream, under the full
  gate;
- ⭐ **the daemon composes what is published.** The agent writes a title and a
  body into its state directory; the daemon reads those bytes and composes the
  rest. What a published change says about where the work came from is not left
  to a model;
- ⛔ **the daemon's own repository operations happen in a directory the agent has
  never touched**, from content copied out and inspected. Not in the project
  directory, not with a sanitising flag set, not ever;
- ⚠ **the agent can still push, because it is allowed to.** That is the
  authorized-channel limit, not a hole:
  [`../docs/limits.md`](../docs/limits.md). What the broker bounds is which
  hosts and which paths, and T-028 is what narrows it further.

⛔ **What it must not do:** rely on a wrapper the session can delete, or take
the agent's word for which repository a change is for.

### Decision

**Whether the agent may open a change directly, or only request one.**

⭐ **Recommendation: request only, and the daemon opens it.** The credential
that can comment can also open a change, so this is a request the agent makes
rather than a wall. ⚠ **The honest framing matters**: it makes the habitual path
fail where the agent reads the reason, and it keeps the published text under the
daemon's control. It is not a boundary, and the documentation says so.

### Prove

```bash
cargo test -p box-session forge:: && cargo xtask check spawn-cwd
```

Passing: `cargo test` exits 0, with named tests for a request composed from the
agent's bytes without executing them, and a repository operation refusing a
working directory under the project root. ⛔ **And the planted hook**: a fixture
repository carrying an executable hook, with the assertion that it never runs.

### Closing

Not closed.

---

## T-110: The web interface, with chat-service login

**Source:** the operator, 2026-09-15; `pingdotgg/t3code` `docs/internals/environment-auth.md` (`9ea892e`).
**Category:** surface
**Priority:** P2
**Effort:** L
**Status:** open
**Blocked by:** T-100

---

### Problem

⛔ **Adding a login changes what the web interface is.** The reference this
project replaces has no login precisely because the address it binds to is the
access control, and it refuses a public bind rather than warning about one. A
logged-in, publicly reachable interface is a second trusted surface, and it
needs real per-user authorization rather than an operator console with a login
page on the front.

### Premise

⭐ **RULED by the operator, 2026-09-15: a logged-in user sees and drives exactly
the sessions they started, and nothing else.** Read-only view of other sessions
where the channel is public. The operator role is separate and explicit.

READ, documentation only, for the rules. Seven of them, from another project's
auth page, each closing a specific escalation:

1. a derived credential narrows a grant and never widens it;
2. ⛔ nothing the client says about itself decides what it may do;
3. ⭐ a list endpoint returns metadata, never a recoverable secret, or read
   access to a list becomes a way to acquire somebody else's authority;
4. ⛔ a failed strong check fails, and never falls back to a weaker one;
5. a long-lived token stays out of a socket URL: mint a short-lived ticket over
   authenticated HTTP instead;
6. ⭐ a successful handshake grants no authority, and every call declares the
   scope it needs;
7. a development credential is ignored outside development.

### Approach

In a new `box-web`:

- login through the chat service's own identity, so there is one account system
  and no second password to hold;
- ⛔ **membership of the configured server is checked at every call**, not once
  at login. A user removed from the server loses access on their next request;
- a session is owned by the account that started it. ⛔ **Ownership is checked
  per call, per session, on read and on write separately**: an operation that
  reads one session and writes another needs two authorizations
  ([`../docs/methodology/reviews.md`](../docs/methodology/reviews.md) lens 1);
- all seven rules above, and ⭐ **rule 6 is the one that gets skipped**, because
  authenticating the connection feels like the hard part;
- ⛔ **the operator role is configuration, never a chat-service role**, so a
  compromised server cannot grant it.

⛔ **What it must not do:** bind publicly without a real reverse proxy in front
and say it is fine, or let a read endpoint return anything from which a
credential, a nonce or a slot assignment can be recovered.

### Prove

```bash
cargo test -p box-web authz::
```

Passing: exit 0, and ⛔ **every one of these is a refusal that must be seen**:

| test | asserts |
| --- | --- |
| a user reading another user's session | refused |
| a user prompting another user's session | refused |
| a user removed from the server, with a valid token | refused on the next call |
| a socket handshake succeeding, then a call with no scope | ⭐ refused |
| a list endpoint's response | scanned for nonce, credential and slot: none present |
| a client claiming a role in its own metadata | ignored |
| a failed strong check | ⛔ refused, never downgraded |

### Closing

Not closed.

---

## T-111: Watching a session, and starting one from the web

**Source:** the operator, 2026-09-15; `QaidVoid/errand` `src/web/`, `web/` (`58a178b`).
**Category:** surface
**Priority:** P2
**Effort:** L
**Status:** open
**Blocked by:** T-110

---

### Problem

A thread is a poor place to read a long diff or browse a project. The interface
is where that happens.

### Premise

READ, not measured. The reference this project replaces carries a live session
view, a project browser, a diff view and a way to start a session, at about
1,512 lines of server and 5,087 of interface at the pinned commit. ⚠ **Those
counts are a shape and not a target.**

### Approach

- a live stream of a session the caller owns, over a socket authorized per
  message (T-110 rule 6);
- ⭐ **the stream carries the same records the thread does**, from one place. Two
  renderings of one event stream drift, and the one a reader trusts is the wrong
  one ([`../docs/conventions/code.md`](../docs/conventions/code.md): one read
  path);
- a read-only observer mode for a session the caller does not own, where the
  channel is public;
- starting a session, which goes through the same admission as a chat message
  (T-043) and the same allowlist;
- browsing the project, ⛔ **through the sandbox's own path translation**
  (T-010's `to_host_path`), so a crafted path cannot make the daemon read a file
  the session itself could not.

⛔ **What it must not do:** read a project path the session could not read, or
render an action a user is not authorized to take. ⚠ **A read-only user seeing a
live control is the one-gated-door class**, and it is
[`../docs/methodology/gate.md`](../docs/methodology/gate.md)'s worked example:
the server refused every upload, every test was green, and the user learned
their permission by watching a tray fill with errors.

### Prove

```bash
cargo test -p box-web view:: && cargo xtask battery web --expect tests/expected/web.json
```

Passing: `cargo test` exits 0. ⛔ **And the driven pass is part (b) of the gate
and is not optional**: drive the real interface as a read-only user and assert
through the document that no control for an unauthorized action is rendered,
not merely that the server refuses it.

### Closing

Not closed.

---

## T-112: Metrics where a person looks

**Source:** the operator, 2026-09-15.
**Category:** surface
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-120, T-101, T-111

---

### Problem

Token use, cost, runtime and queue depth are only useful where somebody is
already looking.

### Premise

⭐ **RULED by the operator, 2026-09-15**: a dashboard, the web interface, and
chat commands, all three.

### Approach

⛔ **One source, three renderings.** The numbers come from T-120's accounting and
nowhere else; a surface that computes its own is a second home for a fact
([`../docs/conventions/docs.md`](../docs/conventions/docs.md)).

| surface | shows |
| --- | --- |
| a chat command | this session: tokens, cost, elapsed, provider, queue position |
| the interface, per session | the same, live |
| the interface, a dashboard | across sessions the caller may see: totals, per provider, per model, queue depth, slot occupancy |
| an export | the same records, for whatever the operator already runs |

⛔ **Scoped by the same authorization as everything else** (T-110): a dashboard
that aggregates across users shows a user only their own rows, and an operator
all of them.

⚠ **Cost is derived from the generated catalogue** (T-041) and carries the
catalogue's date. ⛔ **A price that moved and a number computed from the old one
is a wrong number on a report**, which is worse than a blank.

### Prove

```bash
cargo test -p box-web metrics::
```

Passing: exit 0, with named tests for: one accounting record producing identical
figures in all three renderings; a dashboard scoped to the caller; a cost
carrying its catalogue date; and ⭐ **a model absent from the catalogue rendering
a dash rather than a zero**.

### Closing

Not closed.
