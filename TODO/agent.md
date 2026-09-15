# Agent

What runs inside the sandbox, and how it reaches a model.

[`INDEX.md`](INDEX.md) is the list.

---

## T-040: Drive the harness, own the protocol boundary

**Source:** the operator, ruled 2026-09-15; `badlogic/pi-mono` `packages/coding-agent/docs/` (`f9bcd35`).
**Category:** agent
**Priority:** P1
**Effort:** S
**Status:** done

---

### Problem

What runs inside the sandbox decides the protocol, the argv, the state
directory layout and whether this project has a provider surface at all. Three
entries were shaped by the answer and could not start without it.

### Premise

⭐ **RULED by the operator, 2026-09-15: the harness is the interface layer, and
this project drives it.**

READ, documentation only, and it settles the fork on its own. The harness's own
security page states:

> Pi does not include a built-in sandbox... A partial in-process sandbox would
> be easy to misunderstand as a security boundary while still depending on the
> host shell, filesystem, package managers, credentials, and extension code.
> Real isolation needs to come from the operating system or a
> virtualization/container boundary.

⭐ **And its containerization page is a table of four patterns, every row stating
its own credential cost, with one combination missing**: the whole process
isolated, the credential outside, self-hosted, no third-party gateway.
⛔ **That missing row is this project.**
[`../docs/history/references/harnesses/findings.md`](../docs/history/references/harnesses/findings.md).

Three properties make driving it rather than writing one the right call, and
none of them is "it is less work":

1. **it declares custom providers with a `baseUrl`**, so pointing it at the
   broker needs no patch, and
   [`../docs/methodology/vendoring.md`](../docs/methodology/vendoring.md) stays
   a topic this project never has to open;
2. **it has a headless line protocol** already specified, so the boundary is a
   document rather than an inference;
3. ⭐ **it is explicit that isolation is somebody else's job**, so the two
   projects are not competing for the same responsibility.

### The ruling

⭐ **Own the protocol boundary. Drive whatever speaks it. Ship with the harness
as the first implementation.**

| what this project owns | what it does not |
| --- | --- |
| the line protocol the sandbox boundary carries | the agent loop |
| the adapter that speaks it to a harness (T-047) | the harness's tools, providers or extensions |
| the settings it pins on that harness (T-046) | the harness's own defaults |
| the sandbox, the broker, the slots, the surfaces | anything a harness already does |

⚠ **What this costs, stated so it is not rediscovered as a surprise:** a
dependency inside the boundary whose flags and protocol move independently, and
a second thing to confine. ⛔ **The mitigation is the adapter**: one file that
knows the harness's shape, so a second harness is a second adapter rather than a
second project.

⚠ **Alternatives, recorded so they are not re-argued.** Writing the loop was
rejected: the nearest reference is a complete one in Rust whose own README opens
with a caution about the undertaking. Adopting a terminal runtime or a control
plane was rejected in
[`../docs/history/references/harnesses/findings.md`](../docs/history/references/harnesses/findings.md):
both solve a problem this project does not have.

### Prove

```bash
cargo xtask check record
```

Passing: exit 0, with the ruling in [`RULES.md`](RULES.md) section 5 carrying
its date, and every entry whose shape depends on it updated in the same change.

### Closing

**Closed 2026-09-15T06:46:11Z.** Ruled by the operator in the session that filed
it, which is why it closes without an implementation: ⭐ **the deliverable was a
decision, and the entries it unblocked are T-041, T-045, T-046, T-047 and
T-070.**

```text
No command was run. This entry's acceptance is the record agreeing with itself,
and `cargo xtask check record` does not exist until T-054. The counts in
INDEX.md were re-derived by hand from the rows, and this entry is the one `done`
in them.
```

⚠ **What was believed and is now settled**: the entry was filed saying the
recommendation was "not a ruling" and that it was blocked on the operator. It
was ruled the same day, in the operator's words, and the direction matched the
recommendation. ⛔ **The title kept its original wording until the ruling, and
now names what was decided**, because nothing had been referred to by the old
title yet.

---

## T-041: A generated model catalogue

**Source:** `QaidVoid/kage` `crates/kage-provider/src/catalog/` (`6ad2708`).
**Category:** agent
**Priority:** P2
**Effort:** S
**Status:** open
**Blocked by:** T-040

---

### Problem

The broker has to know where a provider actually is, to forward to it and to put
it on the egress allowlist. Hardcoding that is a table that goes stale;
fetching it at run time puts somebody else's uptime inside the boundary.

### Premise

READ, not measured. kage generates a static catalogue and names its source in
one line at `crates/kage-provider/src/catalog/generated.rs:3`:

> Source: `https://models.dev/api.json`, curated to kage's supported providers.

The record shapes are at `crates/kage-provider/src/catalog/mod.rs`:

| what | lines |
| --- | --- |
| `ProviderInfo`, with the documented endpoint and the environment variable names the upstream associates with it | 18-33 |
| `ModelInfo`, with the context window, the release date and per-million-token pricing, each optional because the upstream does not always publish them | 35-52 |
| the default-model selection | 116 and following |

⭐ **And the escape hatch**: `crates/kage-provider/src/metadata.rs:19-31`, where
something with no catalogue entry declares its own metadata rather than
depending on the catalogue.

⚠ **A session's namespace has no route to fetch a catalogue anyway**, which
settles the run-time option on its own.

### Approach

An `xtask` subcommand that fetches the upstream, curates it to the providers
this project supports, and writes a generated Rust module. Commit the generated
file **and** the generator.

- ⛔ **the generated file is committed**, so a build needs no network and a
  regeneration is a reviewable diff;
- the generator is the instrument
  ([`../docs/methodology/references.md`](../docs/methodology/references.md): the
  instrument is the deliverable);
- carry only what is supported. Generating every provider the upstream lists is
  a maintenance surface with no caller;
- ⭐ **leave a declared path for a provider with no entry**, so an operator with
  a private endpoint is not blocked on this project's curation;
- a check asserts the committed file matches what the generator produces from
  the committed input fixture.

⛔ **What it must not do:** fetch at run time, fetch during a build, or make the
gate depend on a network.

### Prove

```bash
cargo xtask generate catalog --check && cargo test -p box-broker catalog::
```

Passing: `--check` exits 0 when the committed file matches the generator's
output over the committed fixture, and non-zero with a diff when it does not.
The tests cover a provider absent from the catalogue resolving through the
declared path, and a provider's base URL reaching the egress allowlist.

### Closing

Not closed.
