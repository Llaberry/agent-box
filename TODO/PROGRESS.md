# Progress

⭐ **Read this first.** It is the only file the kickoff prompt tells a session to
read, so everything that changes from session to session is here: the baseline,
what the last session did, and the work order.

It carries no history. Every session rewrites it.

How this repository is worked on: [`RULES.md`](RULES.md).
Every entry, one line each: [`INDEX.md`](INDEX.md).
Routing for an agent: [`../docs/AGENTS.md`](../docs/AGENTS.md).

---

## State

- **Last session:** 2026-09-15T05:45:00Z, attended.
- **Tree:** clean on `main`. ⚠ **No source code exists.**
- **Deployed:** not deployed, and there is nothing to deploy.
- **CI:** none yet. It arrives with the gate, T-051.

## Baseline, as measured this session

⛔ **Re-measure rather than trusting the numbers below.** They were true once.

| check | result | at the start |
| --- | --- | --- |
| `cargo build --workspace` | ⛔ **could not run**: no `Cargo.toml`. T-001. | could not run |
| `cargo clippy` | ⛔ could not run, same reason | could not run |
| `cargo test --workspace` | ⛔ could not run, same reason | could not run |
| `cargo xtask gate` | ⛔ could not run: does not exist. T-051. | could not run |
| every relative link in the tree resolves | pass | - |
| characters outside the five, em dashes, control bytes | pass | - |
| marker density under 30 per 100 non-blank lines | pass | - |
| one fact one home | pass | - |
| pages nothing links to | pass | - |
| the commit-message hook refuses a tool-crediting message | pass, mutation-proved four ways | ⛔ **was broken** |

⚠ **"Could not run" is not "pass".** Four of the rows above are the same fact
stated four ways: this project has documents and no code. ⭐ **The six that do
pass were run by a throwaway checker, not by this project's own gate**, which
does not exist. T-052 is what makes them repeatable.

**58 entries: 2 done, 0 partial, 0 blocked, 56 open.**

---

## What this session did

**Set the repository up, mined the references, filed the plan, reviewed it, and
then took a scope ruling that roughly doubled it.**

- Took the methodology, the conventions and the security rules from the template
  in [`../docs/history/references/pins.md`](../docs/history/references/pins.md).
  ⛔ **Did not take its shell scripts**: the checks are Rust here.
- Mined fourteen references. Eleven with tree and tracker; ⭐ **three with
  documentation only, by decision**, because they are tools this project drives
  rather than designs it ports.
- Wrote six sweeps under
  [`../docs/history/references/`](../docs/history/references/), each opening with
  what it did not establish and its weakest claims.
- Wrote the technical reference, the broker design, the sandbox model and the
  limits page.
- Filed 58 entries across twelve categories.
- Ran the three review lenses. ⭐ **Findings below.**

### ⛔ Premises a measurement disproved, or a reading corrected

Seven. The first three changed the plan; the last three were found by reviewing
this session's own work.

1. **"The project this replaces leaks secrets to agents by design" is half
   right.** The provider credential has been brokered behind a per-run nonce, by
   default, for some time. ⭐ **The repository token is not brokered at all** and
   its code says so in a comment.
2. ⭐ **The most serious finding was not in any source file.** It came from that
   project's tracker: a daemon running host-side `git` against the agent's own
   writable directory is host code execution as the daemon's user. T-030.
3. ⭐ **The harness loads code out of the repository it is working on.** Its own
   security page: extensions are TypeScript modules running with the harness's
   permissions, and the headless modes decide by a setting rather than a prompt.
   T-046, and it is the third P0.
4. **Short-lived per-execution credential minting is not a solved problem to
   port.** Open at the two references furthest along. T-028 says so.
5. ⛔ **The orphan-page check did not fire on a planted orphan.** A link to a
   directory was marking every file inside it as linked. Found by lens 2, which
   is exactly the failure that lens exists to catch, and fixed.
6. ⛔ **The commit-message hook was broken.** Copied from the template, it called
   a shell script this repository does not ship, so it exited 2 on every message:
   no attribution was ever checked and the repository could not be committed to
   while it was armed. Rewritten self-contained and mutation-proved four ways.
7. **Four numeric claims were wrong**, found by the claim audit: a line count off
   by 5,500, a file count off by two, a tracked-file count off by seventeen, and
   "two thirds" where the measurement is 56 per cent. All corrected against the
   corpus.

⚠ **Two citations were wrong** and were corrected against the corpus: one line
range off by four, one off by seventeen. ⭐ **Thirty-one other citations were
checked and confirmed.**

⚠ **No claim in this session's work has been checked a second time**, and
[`../docs/history/README.md`](../docs/history/README.md)'s withdrawn-claims table
is still empty.

### What the operator ruled, 2026-09-15

Six rulings, all in [`RULES.md`](RULES.md) section 5 with their dates:

- **full parity with the project this replaces, then more.** Securing the agent
  does not mean removing what it can do. T-090.
- **the harness is the interface layer**, and this project drives it rather than
  writing an agent loop. T-040.
- **users in the channel are mutually untrusted**: full isolation.
- **a logged-in user drives exactly the sessions they started.**
- **two or three provider subscriptions**, a small number of concurrent sessions
  each, fallback or manual selection, and a queue.
- **whether a session may run containers is measured before it is decided.**
  T-017.

## What is in progress

**Nothing.** The tree is coherent: documents and a plan, no half-written code.

---

## Start here next session

⭐ **This is the work order and it lives nowhere else.**

1. **T-001**, [`workspace.md`](workspace.md). The Cargo workspace. ⛔ **It
   carries the one decision still open** (async or synchronous, and where), and
   T-020 cannot start without it. ⚠ **The scope ruling changes the
   recommendation's premise**: with a chat gateway, a web interface and a broker
   all concurrent, the case for one async runtime throughout is stronger than it
   was when the entry was written. Re-read the decision before presenting it.
2. **T-056**, [`tooling.md`](tooling.md). The bootstrap command. ⭐ **Second,
   because every session after this one starts by running it**, and a session
   setting its machine up by hand sets it up differently each time.
3. **T-052**, [`tooling.md`](tooling.md). The document and structure checks.
   ⚠ **Every rule in [`../docs/`](../docs/) is currently enforced by a throwaway
   script that no longer exists**, and this file's counts are maintained by hand.
4. **T-054**, [`tooling.md`](tooling.md). The record check.
5. **T-010**, [`sandbox.md`](sandbox.md). The backend trait. Small, and five
   entries depend on it.
6. **T-030, T-031 and T-046**, the three P0 entries. ⚠ They are sixth rather than
   first because all three are rules that need somewhere to be enforced, and
   T-052 is what enforces two of them. ⛔ **Nothing that spawns a process, reads
   the agent's output, or launches a harness may be written before they land.**

⚠ **If a session has budget for exactly one thing, do T-001**, because nothing
else can start without it.

---

## Open questions for the operator

⭐ **One blocks work. The rest are recommendations a session will act on unless
told otherwise, so an unattended session is not stuck.**

1. ⛔ **Async or synchronous, and where?** T-001. **T-020 cannot start until this
   is ruled.** ⭐ **Recommendation, revised after the scope ruling: one async
   runtime throughout.** The broker serves many concurrent tunnels with long idle
   periods, and the chat gateway and the web interface are both long-lived
   concurrent connections. The original recommendation split it, which was
   right when the project was a broker and a sandbox and is now the more awkward
   answer.
2. ⚠ **T-027's default: on or off?** Recommendation: off until the false-denial
   rate is measured, and the measurement is part of the entry. ⛔ **This is the
   weakest recommendation in the index**: it optimises for adoption over
   strictness, and a reviewer who disagrees has a real case.
3. **T-070's extension language, if there is one.** Recommendation: a line
   protocol over a pipe, so nothing new runs in the trusted process. ⚠ Not
   urgent: the entry does not start until something needs extending.
4. **T-045's first vendor.** Recommendation: whichever subscription the operator
   already pays for, exactly one, and the entry's deliverable includes a written
   list of what refused to be brokered.

---

## Settled, and not to be raised again

[`RULES.md`](RULES.md) section 5 carries these with their dates. The short form:
the work model is todo, the licence is 0BSD, the checks are Rust in `xtask` and
the bootstrap is shell because it runs first, the corpus is kept on its own
branch, upstreaming a patch is not a topic, this project drives a harness rather
than writing one, the target is full parity and then more, and users are
mutually untrusted.
