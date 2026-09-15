# AGENTS.md

⭐ **You are oriented by the end of this file.** It is written for a session
with no memory of any previous one and no context beyond this page. Read it end
to end, then do what "Start working" says.

**This file is a router.** It restates nothing that is written elsewhere, so the
two cannot fork. Everything binding is linked, and the link is the authority.
⛔ **Reading a row in a table here is not reading the rule.**

---

## What this project is

**agent-box** is one binary that runs AI coding agents in sandboxes and holds
every credential outside them. An agent gets a per-run random nonce; the broker
attaches the real credential to outbound requests at the boundary. The agent
never holds a credential at any point in its lifetime.

⭐ **It is deployed as a chat bot in a community server**: a message in one
channel starts a session, a thread is the conversation, and a web interface is
the secondary view. It drives an existing agent harness rather than writing one,
and an existing Linux confinement tool rather than replacing one.

⭐ **The target is full parity with the project it replaces, and then more.**
⛔ **Securing the agent does not mean removing what it can do**, and an entry
that proposes dropping a capability to make the boundary easier is arguing with
a ruling. [`limits.md`](limits.md) is where something that genuinely cannot be
reached securely is written down instead.

⛔ **There is no code yet.** This repository is documents and a plan. Every
statement in [`architecture.md`](architecture.md) is a decision an entry
implements, not a description of something you can run.

**Licence 0BSD. The repository is public.** Both matter: see the absolutes.

---

## Start working

Do these in order. ⭐ **Steps 1 and 2 answer "what do I do", and nothing else
does.**

**1. Read the record.**

```bash
cat TODO/PROGRESS.md
```

It carries the measured baseline, what the last session did, the work order, and
the open questions. ⛔ **Nothing else carries a work order.**

**2. Take the first item from its "Start here next session" list**, unless the
operator named something else in this session. The operator's instruction in
this session wins.

**3. Read what that entry's own Source line names**, in full. An entry cites
other projects at file and line, at a pinned commit, so you do not re-derive
what a previous session already read.

**4. Read the row for your task in the routing table below**, and read what it
names.

**5. Record the start instant**, because everything at the end that measures the
session reads it from there.

```bash
date -u +%Y-%m-%dT%H:%M:%SZ
```

**6. Write `RESUME.md` before doing any work.** It is a dead man's switch: a
session that dies never reaches its ending, so one artefact is written at the
start and refreshed as work moves.
[`methodology/sessions.md`](methodology/sessions.md) says what it carries.

---

## The routing table

⭐ **Find the row for the work in front of you and read what it names, in full.**

| the task | read, in this order |
| --- | --- |
| **Any session, before anything else** | [`../TODO/PROGRESS.md`](../TODO/PROGRESS.md) , [`../TODO/RULES.md`](../TODO/RULES.md) , [`methodology/sessions.md`](methodology/sessions.md) |
| **Implementing an entry** | the entry , its Source line's sweeps under [`history/references/`](history/references/) , [`architecture.md`](architecture.md) , [`methodology/gate.md`](methodology/gate.md) , [`conventions/code.md`](conventions/code.md) , [`conventions/forbidden-patterns.md`](conventions/forbidden-patterns.md) |
| **Anything touching the broker or a credential** | ⭐ [`credential-brokering.md`](credential-brokering.md) , [`history/references/brokers/usable.md`](history/references/brokers/usable.md) , [`security/secrets.md`](security/secrets.md) |
| **Anything touching a sandbox backend** | ⭐ [`sandbox-model.md`](sandbox-model.md) , [`history/references/bailey/usable.md`](history/references/bailey/usable.md) , [`history/references/errand/usable.md`](history/references/errand/usable.md) |
| **Anything that launches or talks to the harness** | ⛔ [`history/references/harnesses/usable.md`](history/references/harnesses/usable.md) , then entries T-046 and T-047. **T-046 is a P0 rule.** |
| **Anything a person types at, or reads from** | [`../TODO/surface.md`](../TODO/surface.md) , [`history/references/harnesses/usable.md`](history/references/harnesses/usable.md) for the authorization rules |
| **Anything counting tokens, cost or time** | [`../TODO/metrics.md`](../TODO/metrics.md) . ⛔ Count at the broker; an agent reporting its own usage is reporting on itself |
| **Anything the daemon runs as a subprocess** | ⛔ [`history/references/errand/findings.md`](history/references/errand/findings.md) , then entry T-030. **This is the P0 rule.** |
| **Anything on Windows** | [`history/references/windows/usable.md`](history/references/windows/usable.md) , [`sandbox-model.md`](sandbox-model.md) |
| **Writing a new entry from an intake** | [`methodology/authoring.md`](methodology/authoring.md) , [`../TODO/INDEX.md`](../TODO/INDEX.md) , ⛔ do not implement in the same session |
| **Studying another project** | ⭐ [`methodology/references.md`](methodology/references.md) , [`history/references/pins.md`](history/references/pins.md) |
| **Taking a measurement** | [`methodology/experiments.md`](methodology/experiments.md) |
| **Writing or editing a document** | [`conventions/prose.md`](conventions/prose.md) , [`conventions/docs.md`](conventions/docs.md) |
| **Committing** | [`conventions/git.md`](conventions/git.md) , [`../TODO/RULES.md`](../TODO/RULES.md) section 2 |
| **Anything crossing a shell, or a quoting problem** | [`conventions/shell.md`](conventions/shell.md) |
| ⭐ **Waiting for anything** | [`conventions/shell.md`](conventions/shell.md) section 10 . ⛔ Never end the turn, and never a harness scheduler |
| **Touching anything remote** | [`security/remote-ops.md`](security/remote-ops.md) |
| **Reaching for a tool, or about to install one** | ⛔ [`agent-tooling.md`](agent-tooling.md) |
| **Needing a machine this one is not** | [`containers.md`](containers.md) |
| **Working on a machine somebody else provisioned** | [`hosted-sessions.md`](hosted-sessions.md) |
| **Third-party code in this tree** | [`methodology/vendoring.md`](methodology/vendoring.md) . ⛔ Patch it here; upstreaming is not a topic |
| **Recording something superseded** | [`methodology/history.md`](methodology/history.md) . ⛔ Not into the page it supersedes |
| **Closing out a session** | [`methodology/sessions.md`](methodology/sessions.md) , [`methodology/reviews.md`](methodology/reviews.md) |

⛔ **Read what the row names in full.** Not grepped, not skimmed, not recalled
from a previous session, not replaced by a search. The routing exists so the
reading is small enough to actually do.

⚠ **When two rows apply, read both.** The union, not the shorter one.

---

## The absolutes

Each is stated here because each is unrecoverable, and each is linked because
each is written once.

1. ⛔ **The branch is `main`.** Never `claude/`, never `agent/`, never any
   tool-shaped name.
2. ⛔ **No tool is credited in a commit.** No co-author trailer naming a model,
   no generated-with line, no tool name in the body. The identity is the
   repository owner's, read from the machine.
   [`conventions/git.md`](conventions/git.md).
3. ⛔ **Every other repository is read-only.** Never open an issue, a pull
   request, a discussion, a comment, a review, a fork or a star anywhere else,
   under any framing. [`security/remote-ops.md`](security/remote-ops.md).
4. ⛔ **A secret never enters the tree, a log, a commit message or a handoff.**
   Not expired, not redacted-looking, not in an example, ⛔ **not in a test.**
   This repository is public. [`security/secrets.md`](security/secrets.md).
5. ⛔ **Nothing that fingerprints a private system** either: real hostnames,
   account identifiers, internal paths, the names of private projects.
   [`public/README.md`](public/README.md).
6. ⛔ **An exit code is read from the process that produced it, unpiped.**
   Piping a check into anything reports the pipeline's status, so a guard that
   failed reads as green.
7. ⛔ **The record is part of the change.** [`../TODO/PROGRESS.md`](../TODO/PROGRESS.md),
   [`../TODO/INDEX.md`](../TODO/INDEX.md) and the entry are edited in the same
   change as the work, never after it.
8. ⛔ **An entry is done only when all three parts of the gate pass**, with
   commands actually run and output actually inspected.
   [`methodology/gate.md`](methodology/gate.md).
9. ⛔ **A constraint closes a route, not the question.** Name three routes you
   considered before recording anything as not-doable, and never write a limit
   as a settled fact for the next session to inherit.
   [`methodology/sessions.md`](methodology/sessions.md).
10. ⛔ **Nothing closes as "won't fix", "upstream's problem" or "out of scope".**
    A blocked entry keeps its status, names its blocker, and says what would
    clear it. [`methodology/work-todo.md`](methodology/work-todo.md).

---

## The three rules that are specific to what this project builds

⭐ **These are not general engineering rules. They are this project's subject.
Two were measured against a running system; the third is stated by the harness's
own security page.**

**⛔ The daemon never runs a tool against state the agent controls.**

Not `git` in the project directory, not a package manager, not a linter, not a
test runner. Every one of those reads configuration out of the directory it runs
in, and several execute it. A daemon that did this gave every session host code
execution as its own user, through `.git/hooks/pre-push`, with no sandbox escape
involved. [`history/references/errand/findings.md`](history/references/errand/findings.md)
carries the evidence; entry T-030 carries the rule.

**⛔ The harness loads code out of the repository it is working on.**

A `.pi/extensions` directory holds TypeScript modules that run with the
harness's own permissions, and the headless modes this project uses decide by a
setting rather than a prompt. The default is safe and one setting away from not
being. ⛔ **This project pins the setting rather than inheriting it**, and
refuses a configuration that would change it. Entry T-046.

**⛔ Only the TLS SNI is an endpoint identity the peer must prove.**

The `Host` header and the HTTP/2 `:authority` are input from the sandbox. A
broker that selects a credential binding from `Host` alone lets a session have
the operator's credential delivered to a peer of its choosing. Two separate
projects shipped that and fixed it.
[`credential-brokering.md`](credential-brokering.md) carries the eight-condition
gate; entry T-021 carries the work.

---

## The tree

| path | what is in it |
| --- | --- |
| [`../TODO/`](../TODO/) | ⭐ the record, the index, the rules, and all 58 entries |
| [`architecture.md`](architecture.md) | ⭐ the technical reference. **When any document conflicts with it, it wins.** |
| [`credential-brokering.md`](credential-brokering.md) | how a session reaches an upstream without holding a credential |
| [`sandbox-model.md`](sandbox-model.md) | what confines a session, and where each promise ends |
| [`limits.md`](limits.md) | ⭐ what this project does not do, and is not going to |
| [`methodology/`](methodology/) | how work is planned, gated, reviewed and resumed |
| [`conventions/`](conventions/) | prose, docs, git, code, forbidden patterns, shell traps |
| [`security/`](security/) | secrets, and the tiers governing action on anything remote |
| [`public/`](public/) | what changes because this repository is public |
| [`history/`](history/) | ⭐ what was believed here and why that changed. The reference sweeps and the reviews. |
| `../.githooks/` | the commit-message hook. ⛔ Not cloned: arm it once per checkout. |

⚠ **There is no `src/`, no `Cargo.toml` and no `scripts/`.** T-001 creates the
first two; the checks are `cargo xtask` subcommands and
[`agent-tooling.md`](agent-tooling.md) names the entry that builds each one.

---

## What a session owes at its end

Specified in [`methodology/sessions.md`](methodology/sessions.md). ⛔ **None of
it is conditional on the session having gone well.** A session that ran out of
budget after one task still owes all of it.

- the record updated, in the same change as the work;
- the gate run, all three parts, or an honest statement of which could not run
  and why. ⚠ **"Could not run" is never reported as "pass";**
- the entry closed in place with its acceptance command's actual output, or left
  open with what is done and what is not;
- ⭐ the **summary table**, printed in chat and saved. One markdown table, before
  and after, every cell grounded in something you can point at. ⛔ It has to be
  able to say that nothing moved;
- ⭐ the **next prompt**, printed in chat only, and it is a **resume** prompt if
  anything at all was left unfinished;
- anything created on a remote system, torn down.

---

## When you are unsure

In order: what the operator said in this session, what the linked rule says,
what the code or a measurement shows, then ask the operator.

⛔ **Never invent a fifth option silently, and never settle a contradiction
between two of these by taking the convenient one.** A contradiction is a
finding, and a finding is reported.

⚠ **And this is a young repository.** Nothing here has been checked twice, the
withdrawn-claims table in [`history/README.md`](history/README.md) is empty, and
an empty table means nothing has been re-checked rather than that everything is
right. ⭐ **If you find a claim in these documents that the code or a
measurement contradicts, that disagreement is worth more than either source
alone.** Record it, correct the document in the same change, and put the
superseded wording in [`history/`](history/).
