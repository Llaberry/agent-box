# Metrics

Counting what a session used, where the agent cannot lie about it.

[`INDEX.md`](INDEX.md) is the list. [`surface.md`](surface.md) is where the
numbers are shown; this is where they come from.

---

## T-120: Usage accounting at the broker

**Source:** the operator, 2026-09-15; `badlogic/pi-mono` `packages/coding-agent/docs/usage.md` (`f9bcd35`).
**Category:** metrics
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-021

---

### Problem

Tokens, cost and runtime decide who waits, what a subscription is spent on, and
whether a session is stuck. The obvious place to count them is the agent, and
that is the wrong place.

### Premise

⭐ **The harness already counts its own**, and its usage page says the totals
"include assistant responses, usage reported by tools, and summary generation".
So a number exists without this entry.

⛔ **An agent reporting its own usage is the agent reporting on itself.** The
thing being metered is untrusted, is running model-chosen code, and has an
incentive nobody designed. ⭐ **The broker sees every request and every response
body, terminates the TLS, and cannot be lied to by the session.**

⚠ READ, not measured, and two things are not established: whether every provider
reports usage in a shape the broker can parse, and how far the two counts
diverge in practice. ⛔ **The second is this entry's most interesting output.**

### Approach

In `box-broker`, on the same path that already reads responses for T-044:

- per request: the session, the slot, the provider, the model, the request and
  response token counts, cache figures where reported, wall time, and the
  outcome;
- per session: the sum, plus elapsed time from launch to teardown;
- ⛔ **parse from the response body, per provider, against a committed fixture.**
  A parser with no sample is a parser nobody can check when a provider changes
  shape;
- ⛔ **a field the provider did not report is a dash, never a zero.** Zero is a
  measurement and a dash is an absence, and a report that confuses them invites
  a comparison that means nothing;
- ⭐ **record the agent's own self-reported total beside the broker's**, and
  ⛔ **never reconcile them silently.** Where they disagree, both are stored and
  the disagreement is surfaced. It is either a parser defect, a provider shape
  this project does not know, or something worth looking at;
- ⛔ **a streaming response is counted when it completes**, and a response that
  never completes is recorded as incomplete rather than dropped.

⛔ **What it must not do:** log a request or response body. ⚠ **A prompt is a
user's content and a response can carry anything the model was given.** Count,
do not keep.

### Decision

**Whether request and response bodies are retained at all.**

⭐ **Recommendation: no, and counts only.** The deployment is a public community
channel, so a body store is other people's content, at rest, in a process that
also holds every credential. The counts answer every question the operator asked
for. ⚠ **The cost is that a parser defect cannot be diagnosed after the fact**,
and the mitigation is the committed fixtures rather than a body store.

### Prove

```bash
cargo test -p box-broker accounting::
```

Passing: exit 0, with named tests for: each provider's shape against its
committed fixture; an unreported field rendering a dash; a streaming response
counted once, on completion; an incomplete response recorded as incomplete;
⭐ **a divergence between the two sources surfaced rather than reconciled**; and
⛔ a fuzz or property test asserting no accounting record can carry a fragment of
a request or response body.

### Closing

Not closed.

---

## T-121: Cost, derived from the catalogue and carrying its date

**Source:** the operator, 2026-09-15; `QaidVoid/kage` `crates/kage-provider/src/catalog/mod.rs` (`6ad2708`).
**Category:** metrics
**Priority:** P2
**Effort:** S
**Status:** open
**Blocked by:** T-041, T-120

---

### Problem

A token count is only a cost once multiplied by a price, and a price is
somebody else's number that moves.

### Premise

READ, not measured. The catalogue shape this project generates carries
per-million-token pricing as an **optional** field, "when the catalog reports
it" (`catalog/mod.rs:35-52`). ⭐ **Optional is the correct shape and it is the
part to preserve**: a model with no published price has no cost, and a zero
would be a lie.

### Approach

- cost is computed from T-041's generated catalogue, ⛔ **never from a table
  written by hand**;
- ⛔ **every cost figure carries the catalogue's generation date.** A price that
  moved and a figure computed from the old one is a wrong number on a report;
- ⛔ **a model with no published price produces a dash**, and the surfaces render
  a dash. Not zero, not "unknown cost, showing 0";
- a subscription slot has no per-token price at all. ⭐ **Say so** rather than
  computing a notional one: what a subscription session costs is a share of a
  fixed fee, and inventing a per-token figure for it is fabricating a number.

### Prove

```bash
cargo test -p box-broker cost::
```

Passing: exit 0, with named tests for: a priced model producing a figure with
the catalogue date attached; an unpriced model producing a dash; a subscription
slot producing no per-token cost; and ⛔ **no code path producing a numeric zero
for an absent price**, asserted by type rather than by value where possible.

### Closing

Not closed.

---

## T-122: What a stuck session looks like

**Source:** the operator, 2026-09-15; `herdrdev/herdr` `README.md` (`052779c`).
**Category:** metrics
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-120

---

### Problem

In a shared channel the expensive failure is not a crash. It is a session
holding a slot while doing nothing, because nobody can tell it from one that is
working.

### Premise

READ, documentation only. A terminal runtime for agents names this as the
problem it solves: "never hunt for the stuck one, every pane is marked working,
blocked, or idle. when an agent stops and needs an answer, herdr says so."

⭐ **That is the right vocabulary and this project can produce it from what the
broker already sees**, without owning any terminals.
[`../docs/history/references/harnesses/findings.md`](../docs/history/references/harnesses/findings.md)
is why that project is not adopted.

⚠ **What the three states mean here is not established.** Deciding it is this
entry's first job, and getting it wrong in the direction of "idle" reaps
sessions that were thinking.

### Approach

A state per session, derived and never self-reported:

| state | derived from |
| --- | --- |
| working | a request in flight at the broker, or a tool running |
| blocked | the harness asked something and nothing has answered |
| idle | neither, for longer than a threshold |

- ⛔ **Derived at the daemon, from what it can observe**, not from a field the
  agent sets;
- ⭐ **an idle session holding a slot is surfaced to its owner and to the
  operator**, because that is the shared cost;
- ⚠ **a threshold is a guess until it is measured.** State the default, state
  that it is a guess, and record what it was measured to be once there is
  traffic;
- ⛔ **nothing is reaped automatically in the first version.** Surfacing is
  reversible and killing somebody's work is not.

### Decision

**Whether an idle session eventually loses its slot.**

⭐ **Recommendation: yes, but not in this entry, and not on a timer alone.**
Surface it first, gather what the real distribution looks like, and let a later
entry set a policy against measured data. ⚠ **A reaper built against a guessed
threshold is a feature that deletes work at a rate nobody predicted.**

### Prove

```bash
cargo test -p box-session liveness::
```

Passing: exit 0, with named tests for each state derived from observed events
rather than a reported field; a session waiting on an answer reported as blocked
and not idle; and ⛔ **no code path that ends a session on the idle state**, which
is the assertion that keeps this entry's scope where the decision put it.

### Closing

Not closed.
