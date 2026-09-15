# Scope

Things deliberately not built, recorded so they are not rediscovered as
oversights, and one question that is genuinely open.

[`INDEX.md`](INDEX.md) is the list. ⛔ **Nothing here closes as "out of scope".**
[`../docs/methodology/work-todo.md`](../docs/methodology/work-todo.md) is the
rule: an entry stays open with the blocker named and what would unblock it.

---

## T-090: Full parity with the project this replaces, then more

**Source:** the operator, ruled 2026-09-15; `QaidVoid/errand` tree shape (`58a178b`).
**Category:** scope
**Priority:** P1
**Effort:** S
**Status:** done

---

### Problem

⛔ **This entry was filed as "the task surface is not built, and this records
why", blocked on a ruling.** The ruling went the other way, so the entry's job
is now to record the scope that replaced it and point at the entries that carry
it.

### Premise

⭐ **RULED by the operator, 2026-09-15**, in their words: this replaces the
existing project "not by getting rid of its features but by having feature
parity, securely and then extending/adding even more on top of it", and the goal
is "to secure the agent without getting rid of features or lobotomizing it".

MEASURED over the corpus, by line count at the pinned commit: 35,471 lines
total. The task surface is `src/chat` 3,124, `src/session` 9,843, `src/web`
1,512, `src/cli` 439 and the separate `web/` interface 5,087: **20,005 lines, or
56 per cent**. The whole sandbox surface is 3,034 lines across 14 files in
`src/sandbox/`, and the broker is 347 of those.

⚠ **Line counts are a shape, not a measurement of anything.** They move with
every commit and some counters skip blank lines. What they establish is the
ratio, and the ratio is the point: **the part that confines and brokers is under
a tenth of that tree, and the other nine tenths is what people actually
touch.**

### What the ruling changed

⛔ **The scope roughly doubled**, and pretending otherwise would be the
"designing a ceiling" failure
[`../docs/methodology/authoring.md`](../docs/methodology/authoring.md) names.
The entries that now carry it:

| what | entries |
| --- | --- |
| the chat channel, threads, commands, work that lands in a repository | T-100, T-101, T-102 |
| the web interface, its login, its views, its dashboard | T-110, T-111, T-112 |
| provider slots, admission, the queue, usage windows | T-042, T-043, T-044 |
| a vendor tool as a provider | T-045 |
| metrics the agent cannot forge | T-120, T-121, T-122 |
| interactive devices, and the container question | T-016, T-017 |

⭐ **And the ordering did not change.** The confinement and the broker still come
first, because every surface above them is a way to reach a session and a
session that is not confined makes all of them worse. [`INDEX.md`](INDEX.md)
carries the argument in full.

### ⚠ What is still not being built

| | why |
| --- | --- |
| a hosted control plane | the operator holds their own credentials. That is the requirement this project exists to satisfy, not a feature gap. |
| warm pools, sandbox claims, cluster orchestration | machinery for a scale this project does not have |
| a terminal runtime that owns agent terminals across machines | [`../docs/history/references/harnesses/findings.md`](../docs/history/references/harnesses/findings.md). ⚠ If sessions ever need to survive a daemon restart, read that reference again before building one. |
| mobile and desktop clients | nothing has asked |

⛔ **None of those closes as "out of scope".** Each is a decision with a reason,
and a reason that stops being true reopens it.

### Prove

```bash
cargo xtask check record
```

Passing: exit 0, with every entry named above present in [`INDEX.md`](INDEX.md)
and the counts derived from the rows.

### Closing

**Closed 2026-09-15T06:46:11Z.** The ruling replaced the question, and the work
it named is filed.

```text
No command was run. `cargo xtask check record` does not exist until T-054.
Verified by hand: every entry named in the table above has a row in INDEX.md,
and the counts there were re-derived from the rows.
```

⛔ **The premise this entry was filed on was disproved by the ruling, and the
title changed with it.** What was believed: that the task surface was out of
scope and the entry existed to stop it being rediscovered as an oversight. What
was ruled: full parity, then more. ⚠ **The old title is recorded here rather
than erased**, because it is the only record of why the plan briefly had a hole
where the chat surface should have been: "The task surface is not built, and
this records why".

---

## T-091: When a shared kernel stops being enough

**Source:** `kubernetes-sigs/agent-sandbox` README scope note and item 1279 (`4b63868`).
**Category:** scope
**Priority:** P3
**Effort:** M
**Status:** open

---

### Problem

Every guarantee this project makes on Linux is the kernel's. A Landlock,
namespace or seccomp defect undoes all of it at once, and the design does not
say at what point that stops being an acceptable trade.

### Premise

READ, and ⚠ **the measurement that would answer it was not fetched.**

`agent-sandbox` describes itself as a sandbox orchestrator that **delegates**
low-level isolation to runtimes like gVisor and Kata, by managing pods
configured to use them through a runtime class. ⭐ That separation is the
relevant shape: the thing deciding what a sandbox may do and the thing enforcing
it are different components with a named contract, which is what this project's
backend trait already is.

⛔ **Item 1279, closed, is "Add multi-runtime benchmark study for gVisor and
Kata Containers", and it was not read.** The corpus trim removed the subtree it
lives in, so this premise is a title and a scope note, nothing more.

### Approach

⛔ **Fetch the measurement before designing anything.**

```bash
cargo xtask mine kubernetes-sigs/agent-sandbox --out references
```

Then read item 1279 and whatever it links, and answer three questions in this
entry:

1. **what does a stronger isolation tier cost**, in start latency and in
   throughput, measured rather than asserted;
2. **what does the two-network-plus-broker design look like under one?** ⭐ The
   expectation is that it is unchanged, because the broker is reached over the
   network and a stronger tier changes the kernel boundary rather than the
   network one. ⚠ **That expectation has not been checked and it is the thing to
   check first.**
3. **what is the trigger?** ⛔ Not "when it feels risky". A named condition:
   mutually distrusting tenants on one host, a workload holding data whose
   disclosure is not recoverable, or a published unprivileged escape in a
   mechanism this project depends on.

⛔ **What it must not do:** add a backend. This entry produces a document and a
trigger, and a backend is a different entry that this one would justify.

### Prove

```bash
cargo xtask check docs
```

Passing: exit 0, with a page under `docs/` carrying the three answers, each
sourced, and the trigger stated as a condition somebody can observe rather than
as a judgement. ⛔ **The cost figures carry their conditions or they are not
figures.**

### Closing

Not closed. **Blocked on:** nothing. ⚠ It is P3 because it changes nothing until
its trigger fires, and ⭐ **fetching item 1279 is a one-command start** that a
session with spare budget should take.
