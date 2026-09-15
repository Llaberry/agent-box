# prior-attempt.md

A previous attempt at this same work exists at `talaria0101/egi`, commit
`708027481a7886d5eb1e4061a22f37e005b07aa9`, under 0BSD. It was read for what it
missed, so this project does not miss the same things.

⚠ **This page is about a tree, not about anybody.** It records technical gaps
in a piece of work so they are not repeated. Everything
[`../conventions/prose.md`](../conventions/prose.md) says about defensive
framing and about characterising other people's projects applies here, and the
rows below are limited to what a reader can check by opening the file.

---

## What it is

The same starting template, the same todo model, the same five reference
subjects, and a Rust workspace with three source files under one crate. 26
entries in its index, one closed.

⭐ **It got the shape right**, and the shape is worth confirming rather than
re-deriving: the todo model, entries carrying file-and-line citations into the
references, a pins table, and sweeps filed under the history directory.

---

## The gaps, and what this project does instead

| gap | evidence | what is done here instead |
| --- | --- | --- |
| ⛔ **the tracker was read shallowly or not at all for the subject that mattered** | its `docs/history/references/pins.md` records `QaidVoid/errand` as "full tree plus tracker (2 items)". The tracker carries three items, and the one it does not account for is the host code execution report. | [`references/errand/findings.md`](references/errand/findings.md) opens with that item, and [`references/README.md`](references/README.md) names it as the finding the sweep was worth |
| ⛔ **the corpus was not kept** | its `pins.md` says the trees live at a scratch path on the mining machine, "NOT tracked and will not survive the machine" | the corpus is on the `references` branch. [`references/pins.md`](references/pins.md) has the command that reaches it. |
| **the premises were read, never measured** | every entry sampled carries "READ, not measured" | the same is true here for most entries, and it is stated the same way. ⭐ **The difference is that this project also names the instrument each premise needs**, and T-064 owes the first one. |
| **no entry addressed the questions that make this project hard** | its 26 entries cover the broker, the backends, the provider and the daemon. None covers exfiltration through authorized channels, execution-scoped authorization, SNI-versus-`Host` binding, or the host-side-tool class. | those four are T-028, T-021, T-027 and T-030, and three of them are recorded as open questions rather than as solved work |
| **the Windows answer was one entry** | one entry in its own index, titled "Windows/WSL2 backend report and refusal rules". ⚠ **Its identifier is deliberately not written here**: an identifier in this project's shape that names somebody else's entry reads as a dangling reference, and a door sweep on 2026-09-15 found exactly that. | ⚠ Not obviously wrong as a size estimate. It is named here because the measured Windows material ([`references/windows/`](references/windows/)) carries four host checks and three documented limits, and one entry is unlikely to hold them. T-015 and T-063 split it. |
| **the shell scripts were copied wholesale** | its `scripts/common/` carries the template's shell checks verbatim, in a Rust project | this repository ships no scripts. The checks are `cargo xtask` subcommands that do not exist yet, and [`../agent-tooling.md`](../agent-tooling.md) names the entry that builds each one. |

---

## ⭐ What it confirms

Two things, and independent confirmation is worth more than a fresh derivation:

1. **The todo model fits this work.** Two attempts reached for it separately.
2. **Reference entries need file-and-line citations at a pinned commit, or the
   implementing session re-reads everything.** Its entries do this and they are
   noticeably easier to act on than prose would have been.

---

## ⚠ The correction this page owes about itself

⛔ **A tracker item count is not a measure of a sweep's depth**, and using one
as evidence above is the weakest step in this page. "Two items" against three
present is a real discrepancy and it is checkable; it is not proof that the
third was unread rather than unrecorded. The finding that stands without that
inference is the stronger one: **no entry in that tree addresses the report's
subject**, and the report is the most serious thing either sweep found.
