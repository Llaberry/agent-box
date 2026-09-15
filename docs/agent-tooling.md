# agent-tooling.md

⭐ **Read this before you install anything, write your own, or decide a job
cannot be done here.**

⛔ **It carries names and one line each, and nothing else.** No flags, no exit
codes, no worked invocations. Every one of those is the tool's own to change,
and a page that copies them becomes wrong without anybody editing it.

---

## The three reflexes this page exists to stop

| the reflex | what it costs |
| --- | --- |
| **installing something** | a system change nobody asked for, on somebody else's machine, that outlives the session |
| **writing your own** | a second implementation of a solved problem, with its own defects, that nobody else will ever fix |
| ⛔ **refusing, because a tool "is not available"** | the most expensive of the three. [`methodology/sessions.md`](methodology/sessions.md) is the rule: a missing tool closes one route, not the question. |

⚠ **A tool being absent is a measurement, not a verdict.** Run the probe, say
what is missing, then find another route. Three routes considered and rejected
is a finding; one route tried is a stop.

---

## What this repository will ship

⛔ **None of it exists yet.** Every row below names a command an entry in
[`../TODO/INDEX.md`](../TODO/INDEX.md) creates, at the path the entry names. A
session that needs one before it exists reads the entry, or does the job by
hand and says that it did.

⭐ Everything here runs with **no network**. A gate that has to fetch a check is
a gate that is red when somebody else's host is down, and a check fetched at
gate time is code nobody reviewed judging the tree.

| command | what it does | the entry that builds it |
| --- | --- | --- |
| `cargo xtask doctor` | the host probe: kernel, Landlock ABI, cgroup delegation, user namespaces, which backends can run here. A probe, not a gate. | T-050 |
| `cargo xtask gate` | runs every check below and prints one verdict | T-051 |
| `cargo xtask check docs` | links resolve, fenced blocks parse, orphan pages | T-052 |
| `cargo xtask check markers` | only the five defined characters, and not too many of them | T-052 |
| `cargo xtask check one-home` | one fact, one home: no long sentence in two documents | T-052 |
| `cargo xtask check control-bytes` | a literal control byte in a tracked text file | T-052 |
| `cargo xtask check secrets` | does anything in the tree carry something that must not be published | T-053 |
| `cargo xtask check attribution` | does any commit credit a tool | T-053 |
| `cargo xtask check record` | the TODO counts agree with the rows, and every cited path and line exists | T-054 |
| `cargo xtask mine OWNER/REPO` | fetch everything a reference sweep needs, and keep it. [`methodology/references.md`](methodology/references.md) is the procedure. | T-055 |

⚠ **One rule gets one enforcer.** Two checks holding one rule is two places for
it to be wrong, and they will be wrong differently.

---

## What is already on the machine

Measure rather than assume: `cargo xtask doctor` reports this once it exists,
and until then the commands in
[`../TODO/RULES.md`](../TODO/RULES.md) section 3 do it by hand.

| you want to | reach for |
| --- | --- |
| build, test, lint, format | `cargo`, with the toolchain `rust-toolchain.toml` pins |
| know what the kernel offers | `cargo xtask doctor`, and until it exists the checks in [`containers.md`](containers.md) |
| run something on a machine this one is not | [`containers.md`](containers.md) |
| know what a hosted session can and cannot prove | [`hosted-sessions.md`](hosted-sessions.md) |
| write a file whose content has quotes, backticks or a dollar sign in it | a file-writing tool, never a heredoc. ⚠ A heredoc is not reliably literal; [`conventions/shell.md`](conventions/shell.md) section 1 carries the measurement. |
| patch one exact string in a file | a patcher that declares how many matches it expects, never `sed -i`, which reports success over a no-op |

---

## ⛔ Before you add a dependency

Three questions, and a dependency that fails any of them does not go in:

1. **What does the standard library and what we already depend on not do?**
   Name it.
2. **What is its own dependency tree?** A crate pulling in an async runtime
   this project does not otherwise need is a large change wearing a small one.
3. **What happens when it is unmaintained?** This project's whole subject is a
   security boundary. A dependency inside that boundary is part of it.

⚠ **Vendoring is the answer when a dependency needs patching.**
[`methodology/vendoring.md`](methodology/vendoring.md) is the rule, and it also
settles that upstreaming a patch is not a topic here.
