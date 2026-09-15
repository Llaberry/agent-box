# Rules

How **this** repository is worked on. [`PROGRESS.md`](PROGRESS.md) is the
record and carries the work order; this file is the part that does not change
between sessions.

⭐ **Everything general is a link.** The rows below name where each rule lives,
and following the link is how you read it.

| topic | where it lives |
| --- | --- |
| what a session owes at its start and its end | [`../docs/methodology/sessions.md`](../docs/methodology/sessions.md) |
| the kickoff and resume prompts, and the summary table | the same file |
| what a unit of work passes before it is done | [`../docs/methodology/gate.md`](../docs/methodology/gate.md) |
| commit identity, and what may reach a remote | [`../docs/conventions/git.md`](../docs/conventions/git.md) |
| anything outside this machine | [`../docs/security/remote-ops.md`](../docs/security/remote-ops.md) |
| what a check must satisfy to be one | [`../scripts/README.md`](../scripts/README.md) |
| how documents are written | [`../docs/conventions/prose.md`](../docs/conventions/prose.md) |
| where superseded wording goes | [`../docs/methodology/history.md`](../docs/methodology/history.md) |

---

## 1. This project's specifics

| | |
| --- | --- |
| the one command that runs every gate | `sh scripts/common/check-gate.sh` plus `cargo test`, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings` |
| the record | [`PROGRESS.md`](PROGRESS.md) and [`INDEX.md`](INDEX.md) |
| the resume file | `RESUME.md` at the root, written at the START of a session |
| the toolchain | `rust-toolchain.toml` pins stable with rustfmt and clippy |
| ASCII discipline | ASCII only in source, config, docs, commit messages, log output. No em dashes. `cargo xtask check-ascii` once it exists; until then the marker check holds the line. |

⛔ **Re-measure the baseline rather than trusting the recorded one**, with the
commands above, before touching anything.

---

## 2. Git, for this project

Commit freely and locally, never push. Publishing is the operator's. Work on
`main`; the errand that runs this session pushes the branch when asked.

Everything else is [`git.md`](../docs/conventions/git.md) and
[`remote-ops.md`](../docs/security/remote-ops.md).

---

## 3. The tools this project has

⚠ **Reach for the purpose-built tool before the general one.** A general tool
used where a specific one exists produces answers that are plausible and wrong,
which is the hardest kind to catch.

| question | tool |
| --- | --- |
| what host is this | `sh scripts/doctor/doctor.sh` |
| is the tree green | `sh scripts/common/check-gate.sh` |
| does the record agree with itself | `cargo run -p xtask -- check-todo` once it exists; until then, count by hand and say so |
| an item closed, so the counts must move | edit `INDEX.md` in the same change. ⛔ Never retype a count without recounting the rows. |
| what has this session done, measured | `git log --oneline`, `git diff --stat` |
| commit | `git commit -F FILE` after `cargo test` passes, and nothing else |

⛔ **Every exit code above is read from the process that produced it.**
[`shell.md`](../docs/conventions/shell.md) section 2 says what that costs when
it is not.

---

## 4. The rules that bite most often, here

### The record is part of the change

⛔ [`PROGRESS.md`](PROGRESS.md), [`INDEX.md`](INDEX.md) and the item are edited
in the **same change** as the work, never after it. A session that fixes
something and leaves it saying the work is open has not finished; it has
published something false into the one file the next session reads first.

### Claims need evidence

⛔ A comparative claim without a committed benchmark does not ship. A flag that
does not move a number does not ship.

### A blocked item stays open

⛔ Nothing here closes because somebody else would have to fix it. A blocked
item keeps its status, names the blocker, and says what would unblock it.

### One fact, one home

Two documents stating the same fact drift. The technical reference
(`docs/architecture.md`) wins conflicts. New sentences of 12 words or more
must not duplicate an existing sentence; `check-one-home` enforces it.

### Tests pass before every commit

`cargo test` runs green before `git commit`. The operator twice received a
pushed red suite on a related project; egi does not repeat that.
