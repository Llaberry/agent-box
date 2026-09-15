# Provider

Which model a session talks to, whose subscription pays, and what happens when
every slot is taken.

[`INDEX.md`](INDEX.md) is the list. ⭐ **Read
[`../docs/history/references/harnesses/usable.md`](../docs/history/references/harnesses/usable.md)
before any entry here.**

---

## T-042: Provider slots, on top of a broker that already exists

**Source:** the operator, 2026-09-15; `can1357/oh-my-pi` `docs/auth-broker-gateway.md` (`6f2c14b`); `earendil-works/pi` `packages/coding-agent/docs/custom-provider.md` (`f9bcd35`).
**Category:** provider
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-021, T-026

---

### Problem

The deployment has two or three provider subscriptions and each allows a small
number of concurrent sessions. Something has to own which session is using
which subscription, refuse to exceed the limit, and hand the slot back.

### Premise

⭐ **RULED by the operator, 2026-09-15.** Two or three subscriptions, two
concurrent sessions per provider, automatic fallback to the next available
provider, manual selection by a user, and users wait their turn until a slot
frees or the provider's usage window resets.

⛔ **READ, and this entry was rewritten because a reference was missed.** A fork
of the harness ships an auth broker and an auth gateway that already do the
provider half: a credential vault, OAuth refreshes performed server-side, a
forward proxy that resolves the credential so that **clients never see the
access token**, per-credential rate-limit blocks, and usage APIs.
[`../docs/history/references/harnesses/findings.md`](../docs/history/references/harnesses/findings.md)
has the detail and the division of labour.

⚠ **Everything known about it is read from one documentation page.** Whether it
behaves as described, how it performs, and what it does under failure are all
unestablished. ⛔ **Establishing that is this entry's first job**, before any
code is written.

READ, for the seam: the harness declares a custom provider with a `baseUrl`, so
pointing it at anything needs **no patch to the harness**.

### Approach

⛔ **Evaluate before building.** Three questions, answered in this entry and
written down:

1. **does the existing broker do what its page says**, under this deployment's
   shape: several subscriptions, a small slot count, and a refresh while a
   session is mid-turn;
2. **what does it not do**, checked rather than assumed. The sweep's reading is
   that it brokers provider credentials and does not bound which hosts a session
   may reach at all, does not bind a credential to a proven peer identity, and
   ⛔ **leaves transport security between its own parts to the operator**;
3. **what does this project own after that.** The sweep's answer: the network
   boundary, every non-provider credential, the injection gate, and the slot
   accounting on top.

⭐ **If the answers hold, drive it and do not rebuild it.** The slot pool then
sits above it, and this entry builds only the part nobody else has:

In `box-broker`, a pool owned by the daemon and never visible to a session:

```
ProviderSlot { provider, index, credential, state }
  state: free | held(SessionId) | exhausted(until)
```

- ⛔ **A slot is the unit that carries a credential.** A session is assigned one
  slot and its nonce resolves to that slot alone. Two sessions on one provider
  hold two different nonces against two different slots, so revoking one does
  not touch the other.
- the count per provider is configuration, defaulting to what the operator
  states, ⛔ **never guessed from the provider**;
- a slot returns to `free` at teardown, on **every** path including a failed
  launch (T-032);
- ⛔ **a session never learns how many slots exist, which provider it got, or
  what any other session holds.** It gets a `baseUrl` and a nonce. Everything
  else is a fingerprint of the deployment.

⛔ **What it must not do:** put the provider's real name in the session's
environment where the choice was automatic. ⚠ A session that can read which
subscription it landed on can time its requests to starve another user's.

⛔ **And it must not leave the upstream broker reachable from anywhere this
project did not bound.** That page says transport security between its parts is
the operator's to provide. ⭐ **Here the operator is this project**, so the
broker binds loopback or a socket, and the session's namespace has no route to
it except through this project's own gate.

### Decision

**Drive the existing broker, or build one.**

⭐ **Recommendation: drive it, and keep the network boundary here.** It already
does the part that is easy to get wrong and hard to test, including the refresh
flow, and rebuilding it would be
[`../docs/conventions/forbidden-patterns.md`](../docs/conventions/forbidden-patterns.md)'s
"rebuilding something the tree already does", one repository over.

⚠ **The cost is a dependency inside the credential path**, which is the most
sensitive place to have one. ⛔ **So the evaluation above is not optional**, and
a negative answer to question 1 reopens this decision with evidence.

**Whether a user may name a provider, and what happens if it is busy.**

⭐ **Recommendation: yes, and it queues rather than falling back.** An explicit
choice is a statement that the other providers are not equivalent for this task.
Silently serving a different model than the one asked for produces work the user
then has to re-do, which costs more than the wait.

Automatic selection is the default and falls back freely.

### Prove

```bash
cargo test -p box-broker slots::
```

Passing: exit 0, with named tests for: two sessions on one provider getting
distinct nonces; a third refused or queued; a slot freed by a failed launch; a
nonce refused after its slot is released; and ⭐ **a session's environment and
its generated policy containing neither the slot count nor any other session's
identifier**, asserted by scanning both.

### Closing

Not closed.

---

## T-043: Admission, the queue, and telling a user where they are in it

**Source:** the operator, 2026-09-15; `QaidVoid/errand` `src/admission/scheduler.ts` (`58a178b`).
**Category:** provider
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-042

---

### Problem

More people ask than there are slots. Without a queue the answer is an error,
and a user who gets an error retries, which is how a burst becomes a rate-limit
ban for everybody.

### Premise

READ, not measured. The reference this project replaces has a scheduler at
`src/admission/scheduler.ts` (306 lines at the pinned commit), and its own
README states what it is for: "A queue bounds how much work reaches the model
provider at once, so a burst of messages cannot get an account rate limited."

⭐ **That reasoning is the entry's whole justification** and it is stronger in
this deployment than in that one, because here a rate-limit ban costs every user
of a shared subscription rather than one.

### Approach

In `box-session`:

- one queue per provider, plus one for "any provider";
- a request in the "any" queue is admitted by whichever slot frees first;
- a request naming a provider waits for that provider (T-042's ruling);
- ⛔ **the queue is bounded.** An unbounded queue is a memory leak with a
  politeness problem: a user waiting behind two hundred others has not been
  served, they have been ignored. Past the bound, refuse and say so.
- ⭐ **position and an estimate are reported, not just "please wait"**: where in
  the queue, and either the time a slot is expected to free or the provider's
  usage-reset instant from T-044;
- a user may withdraw a queued request.

⛔ **What it must not do:** retry a rate limit without honouring its stated
delay, or without a cap.
[`../docs/conventions/forbidden-patterns.md`](../docs/conventions/forbidden-patterns.md)
names the spiral it causes.

### Prove

```bash
cargo test -p box-session admission::
```

Passing: exit 0, with named tests for: a request admitted when a slot frees; a
provider-named request waiting past a free slot on another provider; the bound
refusing with a reason; a withdrawal removing exactly one entry; ⭐ **and the
reported position matching the actual admission order** over a shuffled set,
which is the assertion that catches a queue reporting a number it does not use.

⚠ **Do not assert a scheduling outcome the test does not control.**
[`../docs/methodology/authoring.md`](../docs/methodology/authoring.md) section 6:
make each slot the only supplier of something and wait on the condition between
stages, rather than asserting that two things both happened.

### Closing

Not closed.

---

## T-044: Usage windows, and knowing when one resets

**Source:** the operator, 2026-09-15; `QaidVoid/errand` `docs/reference/commands.md` `!usage` (`58a178b`).
**Category:** provider
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-042, T-120

---

### Problem

A subscription has a usage window. When it is exhausted the slot is not free
and not busy: it is unavailable until a time that only the provider knows. A
queue that cannot tell those apart tells users to wait for something that is not
coming.

### Premise

READ, not measured. The reference this project replaces exposes `!usage`, which
its own documentation describes as reporting how much of the provider's usage
window is left and when it resets, so the information is reachable from the
provider's own responses.

⚠ **Where it comes from differs per provider** and was not established here:
some publish it in response headers, some in a body field, some only as an
error. ⛔ **This entry's first job is to find out per provider and write down
which**, because a model that assumes one shape gets the others wrong silently.

### Approach

In `box-broker`, at the same place T-120 counts usage, because both read the
same responses:

- per slot: what the provider said about remaining allowance and reset instant,
  with **the time it was observed**;
- ⛔ **an exhausted slot enters `exhausted(until)` rather than `free`**, so the
  queue does not hand it out;
- ⛔ **an unknown reset instant is a dash, not a guess.**
  [`../docs/conventions/prose.md`](../docs/conventions/prose.md): a wrong number
  is worse than no number, because a blank gets checked and a number gets used;
- a slot leaves `exhausted` on an observed reset or on a successful request,
  ⚠ **never on a timer alone**, because a timer derived from a guessed window is
  a retry storm with a schedule.

### Prove

```bash
cargo test -p box-broker usage_window::
```

Passing: exit 0, with named tests per provider shape (header, body, error-only),
each against a committed fixture of a real response with its credential removed;
an unknown reset reported as unknown rather than defaulted; and ⛔ a slot in
`exhausted` never admitted by the queue.

⚠ **The fixtures are the deliverable here.** A per-provider parser with no
committed sample is a parser nobody can check when the provider changes shape.

### Closing

Not closed.

---

## T-045: A vendor CLI as a provider, in its own sandbox

**Source:** the operator, 2026-09-15; `earendil-works/pi` `packages/coding-agent/docs/providers.md` (`f9bcd35`); `pingdotgg/t3code` `README.md` (`9ea892e`).
**Category:** provider
**Priority:** P2
**Effort:** L
**Status:** open
**Blocked by:** T-042, T-013

---

### Problem

The best rate is often a subscription with no API, reachable only through a
vendor's own command-line tool. Running that tool means holding its credential,
and a subscription token is worth more than an API key: long-lived, refreshable,
tied to a person's account, and harder to rotate.

### Premise

READ, documentation only, and ⛔ **no vendor tool was installed or run.**

- The harness supports subscription providers via OAuth and API-key providers
  via an environment variable or an `auth.json` file, and `/login` manages both
  (`providers.md`).
- One control surface drives Codex, Claude Code, Cursor, Grok Build, OpenCode
  and Antigravity **using the subscriptions already authenticated on the
  machine** (`t3code` `README.md`). ⚠ That establishes the pattern is real and
  says nothing about how any of them authenticates.
- A custom provider is declared with a `baseUrl` (`custom-provider.md`), so a
  local endpoint is reachable without patching anything.

⛔ **What is not established:** which vendors can be brokered at all. A tool
that pins its certificates, or authenticates inside a protocol message rather
than a header, is out of reach.
[`../docs/limits.md`](../docs/limits.md) carries both shapes.

### Approach

⭐ **The vendor CLI runs in its own sandbox, not the agent's.**

```
  agent sandbox            provider sandbox            the vendor
    the harness              the vendor CLI
    baseUrl -------------->  a shim speaking
                             the model API
                                   |
                                   +-- the broker --> the real endpoint,
                                       with the real subscription credential
```

⛔ **Four rules:**

1. **its own sandbox.** The vendor CLI is a different trust level from the agent
   and from the daemon. Putting it in the agent's sandbox hands the agent
   whatever it holds;
2. **its egress is brokered to that vendor and nothing else**, under the full
   gate in [`../docs/credential-brokering.md`](../docs/credential-brokering.md).
   ⭐ The SNI condition matters most here, because a subscription endpoint and a
   general API endpoint are often the same host;
3. ⛔ **the refresh flow is brokered too.** A subscription credential is an
   access token, a refresh token and a token endpoint. A broker that injects the
   access token and lets the tool keep the refresh token has brokered the cheap
   half. ⭐ **T-042's broker already does this**, with a sentinel standing in for
   every refresh token in the snapshot a client loads and the refresh performed
   server-side. Drive that rather than writing it;
4. **the shim speaks one model API**, so the agent side is unchanged whichever
   vendor is behind it.

⛔ **What it must not do:** mount the operator's own vendor configuration
directory into any sandbox. The harness's own containerization page warns
against exactly that for its own directory, and the warning generalises.

### Decision

**Which vendor to try first.**

⭐ **Recommendation: whichever the operator already pays for, and exactly one.**
The entry's deliverable is a working shim for one vendor plus **a written list
of what was tried and what refused to be brokered**. ⚠ A generic adapter written
against zero measured vendors is a guess with an interface.

### Prove

```bash
cargo test -p box-broker vendor_cli:: && cargo xtask battery inject --expect tests/expected/vendor-cli.json
```

Passing: `cargo test` exits 0 against a fake vendor tool in a fixture. ⛔ **And
the battery is the real acceptance**, asserted from outside: a request reaches
the vendor carrying the real credential, the provider sandbox's own filesystem
holds no credential value, and a refresh happens without the tool ever seeing
the refresh token.

⚠ `cargo xtask battery` is T-064.

### Closing

Not closed.

---

## T-046: Pin the harness's project-trust setting

**Source:** `earendil-works/pi` `packages/coding-agent/docs/security.md` (`f9bcd35`).
**Category:** provider
**Priority:** P0
**Effort:** S
**Status:** open
**Blocked by:** T-040

---

### Problem

⛔ **A repository can ship code that the harness loads and runs with the
harness's permissions.**

### Premise

READ, from the harness's own security page, and ⚠ **not verified against its
code.**

It loads project-local settings, extensions, skills, prompt templates, themes
and system-prompt files from a `.pi/` directory in the working directory, and
`.agents/skills` from the working directory **or any ancestor**. ⭐ **Extensions
are TypeScript modules that run with the harness process's permissions.**

Interactively it asks and stores the answer per directory. ⛔ **The headless
modes do not ask:**

> Non-interactive modes (`-p`, `--mode json`, and `--mode rpc`) do not show a
> trust prompt. Without an applicable saved trust decision,
> `defaultProjectTrust: "ask"` and `"never"` ignore such resources, while
> `"always"` trusts them.

⭐ **So the default is safe and one setting away from not being.** A session
cloning an arbitrary repository, on a host where an operator set `always` to
stop being prompted, runs that repository's extension code as the agent.

⚠ **It is an input guard and not a sandbox**, and the harness says so: it "does
not make untrusted code, untrusted prompts, or untrusted model output safe".

### Approach

- ⛔ **Write `defaultProjectTrust: "never"` into the settings file this project
  places in the session's agent home.** Explicitly, every session, whatever the
  host has.
- ⛔ **Refuse an operator configuration that would set it otherwise**, with the
  reason. This is not a knob.
- Pass the explicit no-approve flag on the command line as well, so the setting
  and the invocation agree. ⚠ **Two mechanisms, because one of them is a file
  the harness reads and file-reading behaviour is what this entry distrusts.**
- ⭐ **The startup report names it**, beside the writable grants, so an operator
  reading what a session was given sees it.

⚠ **This does not make a cloned repository safe.** A skill file, an `AGENTS.md`
and a source comment are still read by the model and are still prompt injection.
The sandbox is what bounds that, not this entry.

### Prove

```bash
cargo test -p box-session harness_trust::
```

Passing: exit 0, and ⛔ **mutation-proved**: a fixture project carrying
`.pi/settings.json`, `.pi/extensions/`, and `.agents/skills` in a parent
directory, with an assertion that the launched command and the written settings
both refuse them, and that an operator configuration attempting to enable trust
is refused at load naming the key.

### Closing

Not closed.

---

## T-047: The harness adapter, and its framing

**Source:** `earendil-works/pi` `packages/coding-agent/docs/rpc.md` (`f9bcd35`).
**Category:** provider
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-031, T-040

---

### Problem

The daemon and the harness speak a line protocol. Getting the framing wrong
corrupts the stream on input a model can produce, and it looks like the other
side's fault.

### Premise

⭐ **READ, and the trap is named by the protocol's own documentation**, which is
the strongest kind of premise short of a measurement:

> RPC mode uses strict JSONL semantics with LF (`\n`) as the only record
> delimiter... Do not use generic line readers that treat Unicode separators as
> newlines. In particular, Node `readline` is not protocol-compliant for RPC
> mode because it also splits on `U+2028` and `U+2029`, which are valid inside
> JSON strings.

Commands go in on standard input as one JSON object per line; responses carry a
type and an optional correlation identifier; events stream out.

### Approach

In `box-session`:

- ⛔ **read bytes, split on `0x0A` only, strip at most one trailing `0x0D`.** Do
  not reach for a convenience lines iterator without reading what it treats as a
  line break;
- ⛔ **bound the record length.** A record with no newline is a record that grows
  until something dies, and the thing producing it is untrusted;
- correlate by the protocol's own identifier rather than by order;
- ⛔ **this channel is the trusted one, so T-031 applies**: it is not the
  harness's standard output, and every record carries the sequence the daemon
  checks;
- the harness's own standard output and error are captured separately and marked
  untrusted in everything the daemon writes.

⛔ **What it must not do:** parse the harness's human-readable output for
anything. There is a protocol; the display is not it.

### Prove

```bash
cargo test -p box-session framing::
```

Passing: exit 0, and ⭐ **the test that matters is a record whose JSON string
value contains `U+2028`**, asserted to survive the round trip as one record.
Plus: `\r\n` accepted, a record split across three reads reassembled, a record
over the bound refused rather than buffered, and out-of-order correlation
identifiers matched correctly.

### Closing

Not closed.

---

## T-048: Pin the harness's tool approval mode

**Source:** `can1357/oh-my-pi` `docs/approval-mode.md` (`6f2c14b`).
**Category:** provider
**Priority:** P0
**Effort:** S
**Status:** open
**Blocked by:** T-040

---

### Problem

⛔ **The harness decides for itself which tool calls need a human, and its
default is to need none.**

### Premise

READ, from the harness fork's own page, and ⚠ **not verified against its code.**

Approval has three tiers a tool may declare, `read`, `write` and `exec`, and
three modes:

| mode | auto-approves | prompts for |
| --- | --- | --- |
| `always-ask` | `read` | `write`, `exec` |
| `write` | `read`, `write` | `exec` |
| ⛔ **`yolo`, the default** | `read`, `write`, `exec` | **none** |

⭐ **One default on that page is safe and the other is not.** A tool that
declares no tier is treated as `exec`, which the page calls the safe default for
unknown custom tools and is right. The mode's own default auto-approves
everything, and two flags force it.

⚠ **For an interactive user at a terminal, `yolo` is a reasonable default.** The
person is watching, and every prompt they dismiss is friction. ⛔ **For a session
started by a message in a public channel, nobody is watching**, and the default
was not chosen for that case.

⭐ **This is the same shape as T-046**, one layer over: a setting the harness
reads, whose safe value this project must write rather than inherit.

### Approach

- ⛔ **Write the mode explicitly into the settings this project places in the
  session's agent home**, every session, whatever the host has.
- ⛔ **Refuse an operator configuration that sets it to the permissive mode**,
  and refuse the flags that force it. This is not a knob.
- ⭐ **The startup report names the mode in force**, beside the writable grants
  and the project-trust setting, so an operator reading what a session was given
  sees all three together.

⚠ **And this is where the ruling in T-042 and the sandbox meet.** A confined
session with no route off the host except a gated broker can be given a wider
approval mode than an unconfined one, ⛔ **but that is an argument the entry that
widens it has to make with evidence**, not a default anybody inherits.

### Decision

**Which mode a confined session runs in.**

⭐ **Recommendation: the middle one, auto-approving reads and writes and
prompting for execution, with the prompt routed to the thread.** In this
deployment the person who asked is in the thread, so a prompt has somewhere to
go, and execution is the tier where the sandbox is doing the most work.

⚠ **The alternative is the permissive mode plus the sandbox**, on the argument
that confinement is the real control and a prompt nobody reads is theatre.
⛔ **That argument is not wrong and it is not this entry's to settle**: it needs
the driven pass in T-064 first, so the sandbox's claims are measured rather than
assumed before anything is relaxed on the strength of them.

### Prove

```bash
cargo test -p box-session harness_approval::
```

Passing: exit 0, and ⛔ **mutation-proved**: an operator configuration setting
the permissive mode is refused at load naming the key; each forcing flag is
refused; the written settings carry the chosen mode; and the startup report
names it.

### Closing

Not closed.
