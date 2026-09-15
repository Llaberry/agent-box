# orchestration and extensibility: findings

Three references read for shape rather than for code: how a sandbox is
orchestrated (`kubernetes-sigs/agent-sandbox`), how an agent harness is made
extensible without bloating (`QaidVoid/kage`), and how a runtime defines what a
sandbox even is (`withastro/flue`).

[`usable.md`](usable.md) is the same sweep for the session doing the work.

---

## ⛔ What this sweep did NOT establish

- **Nothing was run.** No cluster, no agent, no plugin.
- **The `agent-sandbox` controller source was deleted from the corpus.** It was
  swept as an orchestrator design, not as code to port, and a citation into
  Go source cannot be checked without re-cloning.
- **`kage`'s agent loop was not swept**, only its crate graph, its plugin
  capability model and its provider catalogue layout.
- **`flue` was read as one documentation page plus a package listing.** Its
  runtime was not read.
- ⚠ **Only three passes were taken over these three**, and the fourth pass that
  [`../../../methodology/references.md`](../../../methodology/references.md)
  asks for ("what transfers, what must not") is thin here: these are shape
  references, and the shape is most of what they have to say.
- ⛔ **This is revision 1. Assume more claims are wrong than have been found.**

## ⚠ The claims here that are weakest

1. **"kage has no async runtime in core" is read from its architecture page**,
   not verified against `Cargo.toml` across every crate.
2. **The crate-graph recommendation below is this sweep's synthesis**, not
   anything kage or anyone else recommends for this project.

---

## ⭐ The finding that names this project's hardest problem

**`agent-sandbox` issue 1580, open: "Execution-Scoped Authorization for
Persistent and Reusable Sandboxes".**

It states the residual problem precisely enough to quote:

> This is an authorization-execution lifetime mismatch: the authorization can
> outlive the execution for which it was required.

The worked example is two executions in one reused sandbox, E1 needing object
storage and E2 needing a repository token. If authorization is provisioned at
sandbox creation, the effective grant becomes the **union**, so during E2 the
storage credential is present with no legitimate need for it, and code running
in E2 can misuse it.

⭐ **Brokering does not solve this by itself.** A broker moves the credential
out of the sandbox; it does not narrow what the sandbox may ask the broker to
do. An agent holding a nonce that the broker will honour for repository writes
can push a private tree to a public location without ever seeing a token.

⚠ **It is open, and it is open at the reference that is furthest along.** Read
that as evidence of difficulty, not as an oversight. The issue's own proposed
direction is an abstraction for authorization bound to an execution and revoked
when the execution completes, without destroying the sandbox.

⛔ **So this project must not claim brokering closes exfiltration.** What it
closes is credential theft. [`../../../limits.md`](../../../limits.md) carries
the distinction and it is the most important honest limit this project has.

---

## agent-sandbox: what the shape teaches

The project describes itself as a **sandbox orchestrator** and says so in a
scope note in its own README: it delegates low-level container isolation to
runtimes like gVisor and Kata by managing pods configured to use them.

⭐ **That separation is the one to copy, at a much smaller scale.** The thing
that decides what a sandbox may do, and the thing that enforces it, are
different components with a named contract between them. This project's version
is a backend trait with a capability report, which errand already has
([`../errand/usable.md`](../errand/usable.md)).

Three items worth naming:

| item | what it says |
| --- | --- |
| **1580**, open | the execution-scoped authorization problem above |
| **1644** and **1639**, an example and its predecessor | execution-scoped credentials for a reused sandbox, as a worked example rather than a platform feature. ⚠ Read as evidence that the answer today is per-deployment glue. |
| **1279**, closed | a multi-runtime benchmark study for gVisor and Kata. ⭐ This is the reference for "when does a shared kernel stop being enough", and it exists as a measurement rather than an opinion. It was **not** read in this sweep, because the corpus trim removed it; it is named here so the next session can fetch it. |

⚠ **The warm-pool, claim and template extensions are a scale this project does
not have.** A pool of pre-warmed sandboxes answers a latency problem that
appears with many tenants, and building it now would be machinery with one
caller.

---

## kage: extensibility without bloat, which is the brief

`QaidVoid/kage` is a coding agent in Rust with a Lua plugin system. Two things
transfer directly.

### The crate graph, and strict downward layering

`docs/reference/architecture.md`:

```
kage-core                                              (leaf)
kage-provider  kage-tools  kage-session  kage-sandbox  (depend on core)
kage-loop                                              (depends on provider + tools)
kage-mcp  kage-acp  kage-plugin  kage-tui              (depend on loop)
kage-cli   (binary)                                    (depends on everything it uses)
```

⭐ **"Layering is strict: depend only downward. The binary is the only crate
that wires the whole graph together."** That sentence is the whole discipline,
and it is checkable: a dependency that points upward fails a check rather than
a review.

### Capabilities: closed by default, two-sided, per-plugin

`docs/plugins/capabilities.md`. A plugin gets nothing beyond a closed default
unless **both** sides agree:

1. the operator grants it, to a named plugin, in configuration;
2. the plugin asks for it at load and adapts to the answer.

The properties that make it safe enough, in its own words:

- closed by default, so a plugin granted nothing is exactly as confined as
  before the tier existed;
- ⭐ **per-plugin attachment**: the elevated interface is attached to the
  granting plugin's own handle alone, and another plugin cannot see it even if
  it asks;
- an unknown capability name **raises** rather than resolving to false, so a
  configuration typo is loud;
- the subprocess capability spawns directly with no shell, so there is no
  quoting or injection surface, and the working directory is pinned under the
  host's;
- privileged state changes are **host-applied between turns** and pass a veto
  hook, so the plugin requests and the host decides.

⛔ **The tracker says something the design pages do not.** All fourteen closed
pull requests are refactors and two are features; six of them are "split X into
submodules". ⚠ **A plugin surface grows files faster than it grows features**,
and the maintainer paid for that in six separate splits. That is a cost
estimate this project gets for free.

### What the provider catalogue teaches

`crates/kage-provider/src/catalog/generated.rs:3` names its source as
`models.dev/api.json`, **curated to supported providers** and **generated**
rather than fetched at runtime. `crates/kage-provider/src/metadata.rs:19-31`
handles the case a catalogue cannot: a plugin-registered provider with no
catalogue entry declares its own metadata.

⭐ **Generate at build time, from a named upstream, carrying only what is
supported.** A catalogue fetched at runtime is a dependency on somebody else's
uptime inside a boundary that must not have one.

⚠ **kage's README opens with "It's not worth it."** That is the author's
judgement on writing a coding agent, not on the mechanisms above, and it is
worth carrying into the scope decision: this project brokers credentials and
confines sessions. Whether it also owns an agent loop is T-040's question, and
the reference's own author has an opinion about the cost.

---

## flue: what a sandbox is, from the harness side

`apps/docs/src/content/docs/guide/sandboxes.md`, one page, read end to end.

⭐ **The strongest sentence in any of the three references, and it is one line:**

> Choose the narrowest environment that supports the task: expanding it expands
> what model-directed work can read, change, execute, and reach.

The model is that an agent has **at most one** environment and attaching it
defines several capabilities at once: the file and shell tools appear, the
working directory is composed into the system prompt, workspace skills are
discovered, and subagents inherit it. Without one, the agent simply has none of
that.

⚠ **The honest note about its host binding is the part to copy.** Its `local()`
factory binds the agent to the real host with no isolation "by design", and the
page says plainly: do not use it as an isolation boundary for untrusted
requests or multiple tenants. ⭐ **A reference that names where its own
convenience mode stops being a boundary is a reference worth trusting
elsewhere.**

Its virtual sandbox is an in-memory filesystem with an emulated shell, no real
process spawned, and network opt-in by URL prefix. ⚠ That is a fourth backend
shape this project does not need and should know exists: it is the cheapest
possible confinement and it is genuinely enough for a large class of work.

Tracker item **51**, closed: "Docs should mention prompt injection." ⭐ A
documentation gap filed as a defect, which is the posture this project's own
prose rules already take.

---

## Verdicts

| subject | verdict | where it lands |
| --- | --- | --- |
| a strict downward crate graph, wired only in the binary | ⭐ **adopt** | T-001 |
| capabilities: closed by default, two-sided, per-plugin, unknown names raise | ⭐ **adopt** | T-070 |
| a privileged request applied by the host between turns, through a veto | **adopt** | T-071 |
| a generated provider catalogue from a named upstream | **adopt** | T-041 |
| a declared metadata path for things with no catalogue entry | **adopt** | T-041 |
| the orchestrator and the isolation runtime as separate components | **confirms** | errand already does this; no new work |
| "choose the narrowest environment that supports the task" | ⭐ **adopt**, as the default-selection rule | T-012 |
| naming where a convenience mode stops being a boundary | **adopt** | T-061 |
| the execution-scoped authorization problem | ⚠ **open question**, unsolved upstream | T-028, and stated as a limit |
| warm pools, claims, templates, a cluster control plane | **refused** | machinery for a scale this project does not have |
| a Lua runtime in the trusted process | ⚠ **open question** | T-070 rules on the language; the capability model transfers whatever it is |
| gVisor and Kata as a stronger isolation tier | **filed elsewhere** | T-091 records the question and names `agent-sandbox` 1279 as the measurement to fetch |
