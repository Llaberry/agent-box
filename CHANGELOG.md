# Changelog

What shipped, when, and where the evidence is.

⛔ **Newest first, always.** Every heading carries an ISO 8601 UTC stamp, every
entry names its record, and every entry says whether it deployed. Silence is not
an answer.

⚠ **This file is also where the story goes** when a documentation pass removes
it: what broke, what a sentence used to say. So it is expected to grow, and its
length is not a defect.
[`docs/conventions/docs.md`](docs/conventions/docs.md) has the four rules.

---

## 2026-09-15T07:20:00Z: the scope ruling, and the surfaces

**Record:** [`TODO/PROGRESS.md`](TODO/PROGRESS.md).
**Review:** [`docs/history/reviews/2026-09-15.md`](docs/history/reviews/2026-09-15.md).
**Deployed:** no version bump and no deploy. There is nothing to deploy.

The operator ruled on six open questions, which roughly doubled the scope.

- ⭐ **Full parity with the project this replaces, then more.** Securing the
  agent does not mean removing what it can do. T-090 closed with the ruling.
- ⭐ **This project drives an agent harness rather than writing one**, and owns
  the protocol boundary so the harness is replaceable. T-040 closed.
- **Users in the channel are mutually untrusted**: full isolation, no shared
  writable cache.
- **A logged-in user drives exactly the sessions they started.**
- **The provider pool is a small fixed number of slots**, with a queue in front
  of it rather than an error.
- **The container question is answered by a measurement**, not by a design
  argument. T-017.

Filed 18 entries: provider slots, admission and usage windows (T-042 to T-044);
a vendor tool as a provider (T-045); the harness's trust setting and its adapter
(T-046, T-047); interactive devices and the container measurement (T-016,
T-017); one bootstrap command (T-056); the chat surface (T-100 to T-102); the
web surface (T-110 to T-112); and metrics (T-120 to T-122). 58 entries total.

Swept three more references, ⛔ **documentation only, by decision**: they are
tools this project drives, not designs it ports.
[`docs/history/references/harnesses/`](docs/history/references/harnesses/).

### The finding that became the third P0

⭐ **The agent harness loads code out of the repository it is working on.** Its
extension directory holds modules that run with the harness's own permissions,
and the headless modes this project uses decide by a setting rather than by a
prompt. The default is safe and one setting away from not being. T-046 pins it
rather than inheriting it.

⭐ **And the harness names the gap this project fills.** Its containerization
page is a table of four patterns, each stating its own credential cost, and one
combination is missing: the whole process isolated, the credential outside,
self-hosted, no third-party gateway.

### ⚠ What did not happen

- **No code was written**, again and deliberately.
- **No tracker and no source was read for the three new references**, by
  decision. ⛔ Recorded as a gap in
  [`docs/history/references/pins.md`](docs/history/references/pins.md): a
  published document is evidence of intent, never of behaviour.

---

## 2026-09-15T06:46:11Z: the repository, the research, and the plan

**Record:** [`TODO/PROGRESS.md`](TODO/PROGRESS.md).
**Review:** [`docs/history/reviews/2026-09-15.md`](docs/history/reviews/2026-09-15.md).
**Deployed:** no version bump and no deploy. There is nothing to deploy.

Set the repository up and did the reference mining the plan is built on.

- Took the methodology, the conventions and the security rules from the template
  named in [`docs/history/references/pins.md`](docs/history/references/pins.md).
  ⛔ **Did not take its shell scripts.** The checks are Rust here; entries T-052
  to T-055 build them, and every reference to a script was rewritten to the
  command that will exist.
- Mined eleven references, tree and tracker alike, with the depth per reference
  recorded in [`docs/history/references/pins.md`](docs/history/references/pins.md).
  ⚠ The direct API answered 403
  from this session's egress policy and a public text proxy answered 401 on
  address reputation; the credential-free route in the template's miner is the
  third one tried and the one that worked.
- Kept the corpus, on the `references` branch.
- Wrote five sweeps under
  [`docs/history/references/`](docs/history/references/), each opening with what
  it did not establish and its weakest claims.
- Wrote [`docs/architecture.md`](docs/architecture.md),
  [`docs/credential-brokering.md`](docs/credential-brokering.md),
  [`docs/sandbox-model.md`](docs/sandbox-model.md) and
  [`docs/limits.md`](docs/limits.md).
- Filed 40 entries across nine categories.

### What the research changed

- ⭐ **The most serious finding was not in any source file.** A daemon running
  host-side `git` against the agent's own writable directory is host code
  execution as the daemon's user, with no sandbox escape. It came from a
  tracker. Entries T-030 and T-031 are the two P0 items and both exist because
  of it.
- **The claim that started this project was half right.** One credential was
  already brokered and had been for some time; the repository token was not,
  and its own code said so in a comment.
  [`TODO/PROGRESS.md`](TODO/PROGRESS.md) has the detail.
- **Only the TLS SNI is an endpoint identity a peer must prove.** Two separate
  projects shipped `Host`-based credential binding selection and fixed it.
- **A container network flag is not a containment claim across a hypervisor
  hop.**
- **Nobody has solved short-lived per-execution credential minting.** It is open
  at the two projects furthest along, so T-028 is writing something new rather
  than porting, and says so.

### ⚠ What did not happen

- **No code was written.** Deliberately: [`TODO/RULES.md`](TODO/RULES.md) and
  [`docs/methodology/authoring.md`](docs/methodology/authoring.md) both say
  authoring and implementing are different sessions.
- **No gate ran**, because there is nothing to gate and no gate to run with.
  Four of the six baseline rows in [`TODO/PROGRESS.md`](TODO/PROGRESS.md) read
  "could not run", which is honest and is not green.
- **Discussions were not fetched for any reference.** The credential-free route
  is REST and discussions are GraphQL only. ⛔ Recorded as a real gap in
  [`docs/history/references/pins.md`](docs/history/references/pins.md).
- **Comments and review comments were capped at 1000 records** on three of the
  larger references. Where a count reads exactly 1000 the source is truncated.
