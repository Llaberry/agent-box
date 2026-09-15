# HUMAN.md

The operator's side: what only you can do, what to paste, and how to check a
session's work.

⚠ **There is no code yet**, so most of this is about running sessions rather
than running the program. [`README.md`](README.md) says what the project is.

---

## Starting a session

Paste this, and nothing else:

```text
Read ./docs/AGENTS.md and follow it.
```

⭐ **That file is standalone.** It tells the session what the project is, what to
read, what the absolutes are, and what to do first. It routes to the record,
which carries the work order.

**To point a session at something specific**, add one line after it:

```text
Read ./docs/AGENTS.md and follow it.
Work on T-001 this session. Nothing else.
```

⛔ **Your instruction in the session wins over the record's work order**, and the
router says so. That is the one place where something outside the tree takes
precedence.

---

## What only you can do

| | why |
| --- | --- |
| ⛔ **rule on the open questions** | four of them are in [`TODO/PROGRESS.md`](TODO/PROGRESS.md), and two block work. A session that guesses has designed something you did not agree to. |
| ⛔ **supply every credential** | an agent never asks for a value. It says where one goes; you put it there. [`docs/security/secrets.md`](docs/security/secrets.md). |
| ⛔ **authorise a history rewrite** | destructive, breaks every clone, and it is not the fix for a leaked secret. Rotation is. |
| **decide what is public** | this repository is public, and [`docs/public/README.md`](docs/public/README.md) is what changes because of it |
| **push, if you have not granted the policy** | the current policy is `commit-and-push` to this project's own remote on `main`. [`TODO/RULES.md`](TODO/RULES.md) section 2. |

---

## The four open questions, and what each costs you

⭐ **Two of these block work, and answering them is a sentence each.** They are
in [`TODO/PROGRESS.md`](TODO/PROGRESS.md) with the recommendations.

1. ⛔ **Async or synchronous, and where?** Recommended: the broker async,
   everything else synchronous. **T-020 cannot start without this.**
2. ⛔ **The agent loop question.** Recommended: own the protocol, drive an
   existing binary through an adapter first. **It shapes three entries and the
   state directory layout.**
3. **Does this project ever own a way to start a session?** Recommended: no, and
   a chat gateway is a separate program. T-090 is blocked on it.
4. **Should the destination-address check default on?** Recommended: off until
   its false-denial rate is measured. ⚠ **This is the weakest recommendation in
   the plan** and a reviewer who disagrees has a real case.

---

## Checking what a session did

⭐ **The session owes you a summary table, and it has to be able to report that
nothing moved.** [`docs/methodology/sessions.md`](docs/methodology/sessions.md)
says why that property is the one that matters.

Four things to check, in this order, and none of them is reading the summary:

**1. Does the record agree with itself?**

```bash
cargo xtask check record
```

⚠ **That does not exist yet (T-054).** Until it does, count the rows in
[`TODO/INDEX.md`](TODO/INDEX.md) and compare them to its counts by hand. It is
about a minute and it is the check most likely to catch something.

**2. Does the tree match the claim?**

```bash
git log --oneline -15
```

```bash
git status --short
```

⛔ **A file a summary claims was written, that is not on disk, means the write
never landed.** The claim was not real.

**3. Did the gate actually run?**

```bash
cargo xtask gate --strict
```

⚠ **That does not exist yet (T-051).** Until it does, ask what was run and read
the output, and ⛔ **treat "could not run" as different from "pass"**. Four of
the six baseline rows in [`TODO/PROGRESS.md`](TODO/PROGRESS.md) are "could not
run" right now, which is honest and is not green.

**4. Did the reviews find anything?**

⛔ **Read [`docs/methodology/reviews.md`](docs/methodology/reviews.md) for what
a real pass looks like.** The part to check for yourself: where a pass found
nothing, the session owes you **what would have had to be true for it to fire**.
That sentence is the evidence the pass happened at all.

---

## Setting the machine up

Nothing to set up yet. When there is, it will be:

| | |
| --- | --- |
| a Rust toolchain | pinned by `rust-toolchain.toml`, which T-001 creates |
| a Linux confinement backend | ⛔ **version 0.1.4 or later.** Earlier versions carry either a seccomp bypass or a working-directory regression. [`docs/history/references/bailey/usable.md`](docs/history/references/bailey/usable.md) has the evidence. |
| `pasta` | required, and the backend refuses without it rather than degrading |
| a delegated cgroup | optional. Without one, per-session limits are skipped and the run says so. |
| rootless podman | for the container backend, and required on Windows |

⭐ **`cargo xtask doctor` will tell you which of those this host has**, by
attempting each rather than reading a configuration file. That distinction is
not pedantry: a kernel has been measured listing a feature in its module list,
answering a version query with an error, and publishing nothing. T-050.

---

## Arming the commit hook

⛔ **Once per checkout.** Hooks are not cloned, so a fresh clone has none.

```bash
git config core.hooksPath .githooks
```

It refuses a commit message that credits a tool, before the commit exists.
[`docs/conventions/git.md`](docs/conventions/git.md) is the rule.

---

## Reaching the research corpus

The trees and the trackers behind every claim in
[`docs/history/references/`](docs/history/references/) are on their own branch,
so a normal clone stays small.

```bash
git fetch origin references
```

```bash
git worktree add ../agent-box-references references
```

⭐ **Every citation in an entry resolves there**, at the commit
[`docs/history/references/pins.md`](docs/history/references/pins.md) names.
