# Rules

How **this** repository is worked on. [`PROGRESS.md`](PROGRESS.md) is the record
and carries the work order; this file is the part that does not change between
sessions.

⭐ **Everything general is a link.** The rows below name where each rule lives,
and following the link is how you read it. Reading a row is not reading a rule.

| topic | where it lives |
| --- | --- |
| what a session owes at its start and its end | [`../docs/methodology/sessions.md`](../docs/methodology/sessions.md) |
| the kickoff and resume prompts, and the summary table | the same file |
| what a unit of work passes before it is done | [`../docs/methodology/gate.md`](../docs/methodology/gate.md) |
| the three review lenses | [`../docs/methodology/reviews.md`](../docs/methodology/reviews.md) |
| how an entry is authored | [`../docs/methodology/authoring.md`](../docs/methodology/authoring.md) |
| commit identity, and what may reach a remote | [`../docs/conventions/git.md`](../docs/conventions/git.md) |
| anything outside this machine | [`../docs/security/remote-ops.md`](../docs/security/remote-ops.md) |
| how documents are written | [`../docs/conventions/prose.md`](../docs/conventions/prose.md) |
| where superseded wording goes | [`../docs/methodology/history.md`](../docs/methodology/history.md) |
| studying another project | [`../docs/methodology/references.md`](../docs/methodology/references.md) |

---

## 1. This project's specifics

| | |
| --- | --- |
| the one command that runs every gate | `cargo xtask gate` ⚠ **does not exist. T-051.** |
| the record | [`PROGRESS.md`](PROGRESS.md), and [`INDEX.md`](INDEX.md) |
| the resume file | `../RESUME.md`, written at the START of a session |
| the technical reference | [`../docs/architecture.md`](../docs/architecture.md), which wins any conflict |
| the corpus | the `references` branch. [`../docs/history/references/pins.md`](../docs/history/references/pins.md) has the command |

⛔ **Re-measure the baseline rather than trusting the recorded one.** Until
`cargo xtask gate` exists, that means:

```bash
cargo build --workspace --all-targets
```

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

```bash
cargo test --workspace
```

⚠ **All three currently fail, because there is no `Cargo.toml`.** T-001 is what
makes them meaningful, and a session before then measures the documents instead:
count the pages, check every link resolves by hand, and say so.

---

## 2. Git, for this project

⛔ **The branch is `main`.** Never `claude/`, never `agent/`, never any
tool-shaped name.

⛔ **The identity is the repository owner's**, read from the machine, never
hardcoded and never a tool's. [`../docs/conventions/git.md`](../docs/conventions/git.md)
section 1 is the rule and it is absolute: no co-author trailer naming a model,
no generated-with line, no tool name in the body.

```bash
git config user.name
```

```bash
git config user.email
```

**Push policy: `commit-and-push`**, to this project's own remote only, on
`main`. ⛔ **Every other repository is read-only**, under any framing.

⛔ **Arm the hook, once per checkout.** Hooks are not cloned, so a fresh
checkout has none.

```bash
git config core.hooksPath .githooks
```

**What it cost to learn:** nothing here yet. The rule is inherited, and
[`../docs/conventions/git.md`](../docs/conventions/git.md) carries what it cost
elsewhere. ⚠ A rule with no incident behind it is a preference, and this one is
kept because its incident is documented in the file it links to.

---

## 3. The tools this project has

⚠ **Almost none of them exist.** [`../docs/agent-tooling.md`](../docs/agent-tooling.md)
is the catalogue and names the entry that builds each one.

| question | tool | exists |
| --- | --- | --- |
| what host is this | `cargo xtask doctor` | ⚠ no, T-050 |
| is the tree green | `cargo xtask gate` | ⚠ no, T-051 |
| does the record agree with itself | `cargo xtask check record` | ⚠ no, T-054 |
| an entry closed, so the counts must move | `cargo xtask record set T-NNN done` | ⚠ no, T-054. ⛔ **Never retype a count**, and until this exists, re-derive every one of them by counting the rows. |
| study another project | `cargo xtask mine OWNER/REPO` | ⚠ no, T-055 |
| what can this sandbox actually not reach | `cargo xtask battery` | ⚠ no, T-064 |

⛔ **Every exit code above is read from the process that produced it, unpiped.**
[`../docs/conventions/shell.md`](../docs/conventions/shell.md) section 2 says
what that costs when it is not.

⭐ **A tool being absent is a measurement, not a verdict.** Name three routes
before recording anything as not-doable.
[`../docs/methodology/sessions.md`](../docs/methodology/sessions.md).

---

## 4. The rules that bite most often, here

### ⛔ The record is part of the change

[`PROGRESS.md`](PROGRESS.md), [`INDEX.md`](INDEX.md) and the entry are edited in
the **same change** as the work, never after it.

**What it cost, elsewhere:**
[`../docs/methodology/work-todo.md`](../docs/methodology/work-todo.md) carries
the incident in full. ⭐ **Its one-line form: nothing was wrong with any single
file, and what was missing was anything that compared two of them.**

⚠ **Enforced by nothing right now.** T-054 is what changes that, and until then
this is the rule most likely to be broken.

### ⛔ A premise that a measurement disproves keeps its title

The correction goes **underneath**, never as an edit to the premise. The title
is how the entry has always been referred to.

⚠ **Expect this to fire.** Every "MEASURED" premise in this index is somebody
else's measurement on a machine this project has never seen, and T-064 is going
to disagree with some of them.

### ⛔ Nothing closes as "won't fix", "upstream's problem" or "out of scope"

The blocker is named, and so is what would clear it. T-090 is the worked
example: it is `blocked`, it names the ruling it waits on,
and it says the ruling is one sentence.

⚠ Where the blocker is code this project vendors, the answer is to patch it:
[`../docs/methodology/vendoring.md`](../docs/methodology/vendoring.md), which
also settles that upstreaming is not a topic.

### ⭐ A guard is mutation-proved before it is believed

Plant the defect the guard exists to catch, watch it refuse, and record the
plant and the exit code in the entry's closing.

**What it cost, elsewhere:** a scan reported "no orphans" over the exact orphan
it existed to find, twice, because its model of a reader was too narrow. It was
green, it was trusted, and it was theatre.

### ⚠ A number carries its conditions or it is not a number

The machine, the date, the versions, the sample count. A rate with none of those
cannot be compared to anything, which is worse than an absence because it
invites a comparison that means nothing.

⛔ **And never a fabricated one.** Where the value is unknown, write a dash.

---

## 5. Settled decisions, not to be relitigated

⭐ **Rewrite in place when one changes, and move the superseded wording to
[`../docs/history/`](../docs/history/).**

- **The work model is todo, and there is no stage model here.** 2026-09-15. The
  work is a set of independent items that can be done in several orders, and the
  valuable question is what matters most rather than what comes next. Migrating
  the other way means inventing a dependency order after the fact.
- **The licence is 0BSD.** 2026-09-15. Every condition is one more thing a
  coding agent asked to reuse a file can decline over. It stays OSI-approved and
  SPDX-listed, so it is recognised rather than argued about.
- **The checks are Rust, in `xtask`, not shell.** 2026-09-15. One
  implementation, not two that drift, and it runs on every platform this project
  targets. ⚠ The cost is that nothing is enforced until T-052 lands.
- **The corpus is kept, on the `references` branch.** 2026-09-15. A conclusion
  nobody can re-check is an opinion, and two prior sweeps lost their evidence in
  opposite ways.
- **Upstreaming a patch is not a topic.**
  [`../docs/methodology/vendoring.md`](../docs/methodology/vendoring.md).
- ⭐ **This project owns the protocol boundary and drives a harness that speaks
  it.** 2026-09-15. It does not write an agent loop. The harness it ships with
  states that it has no sandbox on purpose and that real isolation must come
  from outside it, so the two are not competing for one responsibility. T-040.
- ⭐ **Full parity with the project this replaces, then more.** 2026-09-15. The
  channel, the threads, the commands, the repository output, the interface and
  the metrics are all in scope. ⛔ **Securing the agent does not mean removing
  what it can do.** T-090.
- ⭐ **Users in the channel are mutually untrusted.** 2026-09-15. Full isolation:
  own workspace, own state, own slot, own bindings, and no shared writable
  cache. The cost is disk and warm-cache time.
- **A logged-in user drives exactly the sessions they started.** 2026-09-15. The
  operator role is configuration, never a chat-service role. T-110.
- **Two or three provider subscriptions, a small number of concurrent sessions
  each, automatic fallback or manual selection, and a queue.** 2026-09-15.
  T-042, T-043.
- ⚠ **Bootstrap is shell; every check is Rust.** 2026-09-15. The boundary is
  exact and it is not a preference: the bootstrap runs before the toolchain
  exists, so it cannot be the `xtask` that everything else is. T-056.

### ⚠ Not yet settled, and blocking work

| question | entry |
| --- | --- |
| async or synchronous, and where | T-001 |
| what the extension language is, if there is one | T-070 |
| whether a session may run containers | ⭐ T-017, and the operator ruled **measure it first**, so the entry produces a measurement and a recommendation rather than an implementation |
| T-027's default, on or off | T-027, and the recommendation is the weakest one in the index |
