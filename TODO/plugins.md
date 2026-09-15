# Plugins

The extension surface, and the capability grants that bound it.

[`INDEX.md`](INDEX.md) is the list.

⛔ **Neither entry here starts until something needs extending.** A plugin
surface with no plugin is machinery with one caller, which
[`../docs/conventions/code.md`](../docs/conventions/code.md) forbids.

---

## T-070: The capability model, and ruling on the extension language

**Source:** `QaidVoid/kage` `docs/plugins/capabilities.md` (`6ad2708`). Read [`../docs/history/references/orchestration/usable.md`](../docs/history/references/orchestration/usable.md) first.
**Category:** plugins
**Priority:** P3
**Effort:** L
**Status:** open
**Blocked by:** T-040

---

### Problem

The brief is pluggable and endlessly extensible without bloating anything. Those
pull against each other, and the thing that reconciles them is not the plugin
interface: it is what a plugin is allowed to do by default.

### Premise

READ, not measured. kage's model is at `docs/plugins/capabilities.md`, and ⭐ its
opening sentence is the design:

> The plugin sandbox is closed by default: no subprocesses, no filesystem
> outside the workdir, no rewriting the live session.

Six properties, each closing a specific hole:

| property | closes |
| --- | --- |
| closed by default | an extension granted nothing is exactly as confined as if the tier did not exist |
| two-sided: the operator grants it by name, and the extension asks and adapts | a grant nobody asked for is unused; a request nobody granted is refused rather than assumed |
| ⭐ per-extension attachment | another extension cannot reach it **even if it asks** |
| an unknown capability name **raises** | a configuration typo is loud rather than silently false |
| no shell in a subprocess capability, working directory pinned | there is no quoting or injection surface to get wrong |
| ⭐ privileged changes host-applied, between turns, through a veto | the extension requests; the host decides and can refuse |

⛔ **And the cost, which comes from its tracker rather than its documentation.**
All fourteen closed pull requests are refactors or small features, and six are
"split X into submodules", including `runtime.rs` and `spec.rs` inside the
plugin crate itself. ⚠ **A plugin surface grows files faster than it grows
features.**

### Approach

Do not build until there is a second caller. When there is:

- `box-plugin`, created by this entry and not before;
- all six properties above, and ⛔ **the truthful-answer shape**: the request
  returns which capabilities were granted **to that extension**, so an extension
  degrades rather than assuming;
- ⛔ **an extension never sees a credential, a nonce, or the broker's binding
  table.** It is on the untrusted side of the same boundary the agent is on, and
  the entry that grants it otherwise has to argue for it;
- the capability set starts at exactly what the first real extension needs.

### Decision

**The extension language.**

| option | buys | costs |
| --- | --- | --- |
| **a line protocol over a pipe, language-agnostic** | ⭐ nothing runs in the trusted process. An extension is a subprocess this project already knows how to confine. | one process per extension, and a protocol to version |
| **an embedded scripting runtime** | cheap calls, and one process | ⛔ **a scripting runtime inside the trusted process**, in a project whose whole claim is about what runs where. It is also a dependency with its own defects inside the boundary. |
| **a compiled plugin interface** | fast | ⛔ arbitrary native code in the daemon. Not in this project. |

⭐ **Recommendation: the first.** This project already confines subprocesses; an
extension is one more, and the trusted process gains nothing new to run. It
costs a protocol, and a protocol is reviewable in a way an embedded runtime's
attack surface is not.

⚠ **Not ruled**, and the entry does not start under an assumption.

### Prove

```bash
cargo test -p box-plugin capabilities::
```

Passing: exit 0, and ⛔ **the acceptance is the refusals**:

| test | asserts |
| --- | --- |
| an extension granted nothing | every elevated call is absent, not present-and-failing |
| an extension asking for a capability granted to a different extension | ⭐ refused, and the interface is **not attached** |
| an unknown capability name in the configuration | raises at load, naming it |
| an extension reading a credential, a nonce, or a binding | no path exists |
| a privileged request | applied by the host between turns, and refusable by the veto |

### Closing

Not closed.

---

## T-071: Host-applied privileged requests, with a veto

**Source:** `QaidVoid/kage` `docs/plugins/capabilities.md` (`6ad2708`), the `session_write` section.
**Category:** plugins
**Priority:** P3
**Effort:** M
**Status:** open
**Blocked by:** T-070

---

### Problem

An extension that can change trusted state directly is trusted. One that can
only ask is not, and the difference is where the change is applied.

### Premise

READ, not measured. kage's elevated session calls **only request**; the host
applies the change between turns, after consulting a veto hook, so an extension
can confirm or block its own request.

⭐ **And the narrower detail is worth copying**: its entry listing returns
metadata only, ids, kinds and timestamps, with no message text, ⛔ **because it
is a navigation index rather than a transcript reader.** A capability that
returns more than its job needs is a capability with a second job nobody
reviewed.

### Approach

- every privileged operation an extension can reach is a **request**, queued;
- ⛔ **the host applies it, at a point the host chooses**, never inside the
  extension's call;
- a veto hook runs before application, and a refusal is ordinary;
- ⭐ **every capability returns the minimum its job needs.** An interface that
  returns content where an identifier would do is widened by default.

### Prove

```bash
cargo test -p box-plugin requests::
```

Passing: exit 0, with named tests for: a request not taking effect during the
extension's own call; the veto refusing and the state unchanged; and ⛔ a
capability's return value asserted to carry identifiers and not content.

### Closing

Not closed.
