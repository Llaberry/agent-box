# AGENTS.md

egi runs coding agents from chat, sandboxed, with secrets brokered at the
boundary. Public Rust workspace, 0BSD. A message opens a thread and a
session; the agent works in one project directory and nowhere else.

**This file is a router.** It restates nothing that is written elsewhere, so
the two cannot fork. Everything binding is linked, and the link is the
authority. Reading a row in a table here is not reading the rule.

---

## Start here, every session

⭐ **Read [`TODO/PROGRESS.md`](../TODO/PROGRESS.md) first.** It carries the
baseline, what the last session did, what is in progress, and the work order.
Nothing else carries a work order.

Then run the probe, because a different machine or a moved tool changes what
this session can prove:

```bash
sh scripts/doctor/doctor.sh
```

Then read what **this task** routes you to, below. Not everything, and not
less.

---

## The routing table

⭐ **This table is the reason this file exists.** Find the row for the work in
front of you and read what it names, in full.

| the task | read, in this order |
| --- | --- |
| **Any session, before anything else** | [`TODO/PROGRESS.md`](../TODO/PROGRESS.md), [`methodology/sessions.md`](methodology/sessions.md) |
| **Implementing an approved TODO entry** | the one entry, [`TODO/PROGRESS.md`](../TODO/PROGRESS.md), [`methodology/gate.md`](methodology/gate.md), [`conventions/code.md`](conventions/code.md), [`conventions/forbidden-patterns.md`](conventions/forbidden-patterns.md) |
| **Authoring new work from an intake** | [`methodology/authoring.md`](methodology/authoring.md), [`architecture.md`](architecture.md), ⛔ do not implement |
| **Fixing a defect** | [`methodology/authoring.md`](methodology/authoring.md), the code the defect is in, [`conventions/forbidden-patterns.md`](conventions/forbidden-patterns.md) |
| **Resuming an interrupted session** | [`methodology/sessions.md`](methodology/sessions.md) resuming section, `RESUME.md` at the root, ⛔ the tree and the running system, never the old conversation |
| **Touching anything remote** | [`security/remote-ops.md`](security/remote-ops.md) |
| **Anything involving a credential** | [`security/secrets.md`](security/secrets.md) |
| **Writing or editing a document** | [`conventions/prose.md`](conventions/prose.md), [`conventions/docs.md`](conventions/docs.md) |
| **Committing** | [`conventions/git.md`](conventions/git.md) |
| **Anything crossing a shell, or a quoting problem** | [`conventions/shell.md`](conventions/shell.md) |
| **Studying an external repository** | [`methodology/references.md`](methodology/references.md) |
| **Reading a filed reference sweep** | [`history/references/pins.md`](history/references/pins.md) first, then the sweep's findings plus usable |
| ⭐ **Touching any vendored or third-party source** | [`methodology/vendoring.md`](methodology/vendoring.md). ⛔ Patch it here, and upstreaming is not a topic |
| **Taking a measurement** | [`methodology/experiments.md`](methodology/experiments.md) |
| **Recording something superseded** | [`methodology/history.md`](methodology/history.md). ⛔ Not into the page it supersedes |
| **Running anything on Windows or in WSL2** | [`windows.md`](windows.md) |
| **Reading the threat model or the credential story** | [`../SECURITY.md`](../SECURITY.md), [`credential-brokering.md`](credential-brokering.md) |
| **Reading what a sandbox backend enforces** | [`architecture.md`](architecture.md), [`windows.md`](windows.md) on Windows |
| ⭐ **Waiting for anything** | [`conventions/shell.md`](conventions/shell.md) section 10. ⛔ Never end the turn, and never a harness scheduler |
| **Closing out a session** | [`methodology/sessions.md`](methodology/sessions.md), [`methodology/reviews.md`](methodology/reviews.md) |

⛔ **Read what the row names in full.** Not grepped, not skimmed, not recalled
from a previous session, not replaced by a code-graph query.

⚠ **When two rows apply, read both.** The union, not the shorter one.

---

## The absolutes

Short enough to state here, and each has been broken before:

1. ⛔ **No tool is credited in a commit.** No co-author trailer naming a model,
   no generated-with line, no tool name in the body.
2. ⛔ **Commit freely and locally, never push.** Publishing is the operator's.
3. ⛔ **Every other repository is read-only.** Never open an issue, a pull
   request, a discussion, a comment, a review, a fork or a star anywhere else,
   under any framing.
4. ⛔ **A secret never enters the tree, a log, a commit message or a handoff.**
   Not expired, not redacted-looking, not in an example.
5. ⛔ **An exit code is read from the process that produced it, unpiped.**
   Piping a check into anything reports the pipeline's status, so a guard that
   failed reads as green.
6. ⛔ **The record is part of the change, not a report about it.** It is edited
   in the same change as the work, never after.
7. ⛔ **A unit of work is done only when all three parts of the gate pass**,
   with commands actually run and output actually inspected.
   [`methodology/gate.md`](methodology/gate.md).
8. ⛔ **Tests pass before every commit.** `cargo test` is green, not assumed.

---

## The tree

| directory | what is in it |
| --- | --- |
| `crates/` | the Rust workspace. `egi-core` holds the contracts; backends land beside it |
| `docs/` | the conventions, the methodology, and the technical reference |
| `scripts/` | the gates, the checks, and the probe |
| `TODO/` | the record: progress, index, rules, and the entries |
| `experiments/` | committed measurement scripts and their results |
| `docs/history/` | superseded wording, sweeps, and reviews. Not the work order. |

---

## Reach for the tool that exists

⛔ **Before you install anything, write your own, or decide a job cannot be done
here, read [`agent-tooling.md`](agent-tooling.md).**

| you want to | use | not |
| --- | --- | --- |
| know what host this is and what is installed | `sh scripts/doctor/doctor.sh` | assuming |
| know what tool does a job | ⭐ [`agent-tooling.md`](agent-tooling.md) | installing something, or writing your own |
| run the whole local gate | `sh scripts/common/check-gate.sh` | a list typed from memory |
| run the Rust suite | `cargo test` | running some of it |
| check formatting and lints | `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings` | eyeballing it |
| mine an external repository | `sh scripts/common/mine-repo.sh OWNER/NAME` | a hand-written fetcher |

---

## What a session owes at its end

Specified in [`methodology/sessions.md`](methodology/sessions.md). The short
form, and none of it is conditional on the session having gone well:

- the record updated, in the same change as the work;
- the gate run, all three parts;
- the entry closed with its evidence, or the partial state recorded;
- ⭐ the **summary table**, printed in chat and saved;
- ⭐ the **next prompt**, printed in chat only, and it is a **resume** prompt if
  anything at all was left unfinished;
- anything created on a remote system, torn down.

---

## When you are unsure

In order: what the operator said in this session, what the linked rule says,
what the probe or the code measured, then ask the operator.

⛔ Never invent a fifth option silently, and never settle a contradiction
between two of these by taking the convenient one. A contradiction is a
finding, and a finding is reported.
