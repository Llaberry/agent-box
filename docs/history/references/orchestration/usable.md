# orchestration and extensibility: usable

The shapes and the lines, for the session laying out the crates and the plugin
surface. [`findings.md`](findings.md) is the reasoning.

---

## ⭐ The crate graph, adapted

`kage`'s rule, from `docs/reference/architecture.md`: **layering is strict,
depend only downward, and the binary is the only crate that wires the whole
graph together.**

This project's version, which T-001 creates:

```
box-core                                     (leaf: types, errors, ids, cancel)
box-policy   box-broker   box-sandbox        (depend on core)
box-session                                  (depends on policy + sandbox)
box-plugin                                   (depends on session)
box-cli      xtask                           (binaries)
```

| crate | owns |
| --- | --- |
| `box-core` | identifiers, error types, the cancel flag, the wire types shared across the boundary. No IO. |
| `box-policy` | what a session may touch, as data. Generation, validation and serialization. ⛔ No secret ever enters a policy. |
| `box-broker` | the CONNECT proxy, the binding table, the injection gate, the certificate authority |
| `box-sandbox` | the backend trait, the capability report, and one module per backend |
| `box-session` | lifecycle: launch, supervise, tear down, discover orphans |
| `box-plugin` | the extension surface and the capability grants |
| `box-cli` | the binary. The only crate that knows about all of the above. |
| `xtask` | the gate, the probe, the miner. Not shipped. |

⭐ **Checkable rather than reviewed.** A dependency pointing upward is a failed
check, and T-052 owns the check.

⚠ **Two crates in that list are speculative and should be created only when
they have a second caller**: `box-plugin` before there is one plugin, and any
split inside `box-sandbox` before there is a second backend.
[`../../../conventions/code.md`](../../../conventions/code.md) is the rule.

---

## The capability model, as a specification

From `kage`'s `docs/plugins/capabilities.md`. ⭐ **Take all six properties. Each
of them closes a specific hole.**

| property | what it closes |
| --- | --- |
| **closed by default** | an extension granted nothing is exactly as confined as if the tier did not exist |
| **two-sided**: the operator grants it by name, and the extension asks for it and adapts to the answer | a grant that the extension does not ask for is unused; a request the operator did not grant is refused, not assumed |
| ⭐ **per-extension attachment**: the elevated interface is attached to the granting extension's own handle | another extension cannot reach it **even if it asks**. A grant to one is not a grant to the runtime. |
| **an unknown capability name raises** | a configuration typo is loud rather than silently false |
| **no shell** in a subprocess capability, and a working directory pinned under the host's | there is no quoting or injection surface to get wrong |
| ⭐ **privileged changes are host-applied, between turns, through a veto hook** | the extension requests; the host decides and can refuse |

The truthful-answer shape matters: the request returns a table of
`{ name = granted }` where a capability is present only if **that exact
extension** was granted it. An extension that degrades gracefully on a refusal
is the design; one that fails on a refusal is a dependency in disguise.

### ⚠ What it costs, from the tracker rather than the docs

All fourteen closed pull requests on `kage` are refactors or small features,
and six are "split X into submodules", including `runtime.rs` and `spec.rs`
inside the plugin crate itself.

⛔ **A plugin surface grows files faster than it grows features.** Budget for
that, and do not build the second extension point until the first has a second
caller.

---

## The provider catalogue

`crates/kage-provider/src/catalog/generated.rs:3` names the upstream and the
curation in one line:

> Source: `https://models.dev/api.json`, curated to kage's supported providers.

The rules that follow:

1. ⭐ **Generate at build time, not at run time.** A catalogue fetched at
   runtime puts somebody else's uptime inside this project's boundary, and a
   sandbox with no route off the host cannot fetch one anyway.
2. **Carry only what is supported.** The upstream lists every provider;
   generating all of them is a maintenance surface with no caller.
3. **Leave a declared path for things with no entry.**
   `crates/kage-provider/src/metadata.rs:19-31`: a provider registered by an
   extension declares its own metadata rather than depending on the catalogue.
4. ⚠ **Commit the generated file and the generator.** The generator is the
   instrument; the file is the evidence. Regeneration is a reviewable diff.

`crates/kage-provider/src/catalog/mod.rs:18-50` is the shape to port: a
provider record with its documented endpoint and the environment variable names
the upstream associates with it, and a model record with the context window,
the release date and the per-million-token pricing, each optional because the
upstream does not always publish them.

---

## The selection rule, one sentence

From `flue`'s sandboxes guide:

> Choose the narrowest environment that supports the task: expanding it expands
> what model-directed work can read, change, execute, and reach.

⭐ **That is the rule for every default in this project.** The default backend,
the default policy floor, the default egress allowlist, the default capability
set. Where a default is wider than the narrowest thing that works, the entry
that set it says why.

⚠ **And the corollary flue states about its own host-binding mode**: a
convenience mode that is not an isolation boundary says so, in its own
documentation, in the place somebody reaches for it. This project's equivalent
is any backend or flag that weakens the boundary, and
[`../../../limits.md`](../../../limits.md) is where each one is named.

---

## What not to build

| | why |
| --- | --- |
| warm pools, claims, templates | `agent-sandbox`'s extensions answer a latency problem that appears with many tenants. One caller today. |
| a cluster control plane | this project is one binary an operator runs |
| a second backend trait implementation before there is a second backend | the trait is justified by two backends existing, not by one plus a plan |
| an in-memory emulated shell backend | `flue`'s virtual sandbox is a genuinely cheaper confinement and a fourth shape. ⚠ Record that it exists; do not build it until something asks. |

---

## ⛔ The problem to write down rather than claim to have solved

`agent-sandbox` issue 1580: **authorization outliving the execution that
justified it.**

In this project's terms: a session holds a broker nonce for its whole life, and
the broker's binding table is per session rather than per turn. A task that
needed repository read in its first turn can still reach repository write in its
last one, and an agent that has been prompt-injected in between has whatever the
session was granted at the start.

Three routes, and none is free:

| route | what it costs |
| --- | --- |
| narrow the binding table per phase of a task | somebody has to say what the phases are, and a wrong answer blocks legitimate work |
| require an out-of-band approval for the widening operations | the operator is now in the loop, which is the point and also the cost |
| mint per-execution credentials at the broker and revoke at the end | needs the upstream to support short-lived minting; `agent-vault` 255 wants this and does not have it |

⛔ **Do not record this as solved, and do not record it as impossible.** T-028
carries it with the routes above, and
[`../../../limits.md`](../../../limits.md) states the limit in the place a user
will read it.
