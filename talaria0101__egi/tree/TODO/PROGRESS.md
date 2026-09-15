# Progress

⭐ **Read this first.** It carries the baseline, what the last session did,
and the work order. It carries no history: for history, read the git log,
the entries, and the handoffs. Rewritten every session.

How this repository is worked on: [`RULES.md`](RULES.md).
Every entry, one line each: [`INDEX.md`](INDEX.md).
Routing for an agent: [`../docs/AGENTS.md`](../docs/AGENTS.md).
Filed reference sweeps: [`../docs/history/references/pins.md`](../docs/history/references/pins.md)
first, then the sweep's findings plus usable.

---

## State

- **Last session:** 2026-09-15T04:39:33Z, unattended.
- **Tree:** clean at f9c5c73.
- **Deployed:** not deployed.
- **CI:** workflow files present, no runs yet.

## Baseline, as measured this session

⛔ Re-measure rather than trusting the number below. It was true once.

| check | result | at the start |
| --- | --- | --- |
| `cargo test` | 8 passed, 0 failed | 8 passed |
| `cargo fmt --all --check` | clean | clean |
| `cargo clippy --all-targets` | clean | clean |
| `check-gate.sh` | 9 passed, 0 failed, 0 skipped | 9 passed |
| `check-docs` | 57 files, 251 links, 83 blocks | 43 files |
| `check-markers` | densest 26/100, ceiling 30 | same |

**26 entries: 1 done (T-022), 0 partial, 0 blocked, 25 open.**

---

## What this session did

- Mined all nine references at their pins with `mine-repo.sh` (4 full
  trees plus tracker, 5 shallow trees plus issues both states). Corpus
  untracked at `/workspace/.scratch-mine/<owner>__<repo>/`; pins plus
  re-fetch commands in `docs/history/references/pins.md`.
- Filed four sweep pairs (findings plus usable): errand-malaria,
  agent-vault, opensandbox, orchestration. Load-bearing new findings:
  Host-header-only binding match (OpenSandbox 1758, CubeSandbox 1697),
  peer-IP rate limits behind NAT (agent-vault 380/381), method-scoping
  (323), SNI-gated interception default (OSEP-0023), execution-scoped
  tokens (agent-sandbox 1580), GitHub App tokens (agent-vault 255),
  write-time policy validation (CubeSandbox 1458).
- Updated entries with sweep citations: broker (matcher tuple,
  proxyauth, memo order, destination binding, new prove cases),
  provider (SSE pieces, catalog, xtask pattern), sandbox and hardening
  (bailey Capabilities lines, WSL numbers), chat (flue Discord shape),
  daemon (1580 lifetimes, 255 token kind), meta (T-022 closed).
- Fixed credential-brokering doc (destination binding, SSH refusal,
  transfer table) and added the AGENTS router row for filed sweeps.
- Ran the three-lens review
  (`docs/history/reviews/mining-2026-09-15.md`): fixed one router gap
  and three wrong tracker denominators (full four-way splits now).
  Guards mutation-proved; one-home exemption for `docs/history/`
  confirmed by design.
- Closed T-022 in place with the acceptance evidence. INDEX counts
  moved in the same change (1 done, 25 open).

## What is in progress

Nothing. The mining change is committed at f9c5c73 and pushed. Follow the work order.

---

## Start here next session

⭐ **This is the work order and it lives nowhere else.**

1. T-021, `TODO/meta.md`: write CHANGELOG entries for the skeleton
   and mining commits so the log matches the file.
2. T-019, `TODO/meta.md`: xtask with ascii, todo-count, catalog guards.
3. T-001, `TODO/broker.md`: the CONNECT broker crate. Read sweep 2
   usable plus sweep 3 usable first; the prove list carries the
   tracker-demanded cases.
4. T-002, `TODO/broker.md`: the binding table. Read the 1758/1697
   regression note first.
5. T-003, `TODO/sandbox.md`: the backend trait crate.

Each implementation entry names its sweep files and exact lines. No
further mining is needed before building; re-mine a reference only if
its pin moves.

---

## Open questions for the operator

1. Repo home: `egi` lives at `talaria0101/egi` because the bot account
   cannot create under `QaidVoid`. Transfer it to the org, or keep it
   where it is? Recommendation: transfer.
2. Push policy: this session commits locally and pushes to the bot
   repo when the errand asks. Confirm. Recommendation: keep pushing
   per chunk (adopt-fork discipline).
3. Corpus: the mining trees live untracked at `/workspace/.scratch-mine/`
   on this machine only. Move them onto a side branch in the repo, or
   accept re-fetch via `pins.md`? Recommendation: accept re-fetch; the
   corpus is 130M+ and mostly other projects' code.
4. Chat platform: Discord first assumed. Confirm, or name another.
   Recommendation: Discord first.

---

## Settled, and not to be raised again

- Work model is todo: skeleton exists, work is a backlog. 2026-09-15.
- No PowerShell twins: the Windows daemon runs inside WSL2. 2026-09-15.
- Rust 1.86 MSRV, edition 2021, 0BSD licence. 2026-09-15.
- Reference pins are short SHAs in tree; full SHAs in corpus
  PROVENANCE plus re-fetch commands. 2026-09-15.
- Wildcard rule is errand multi-level, not agent-vault one-level.
  2026-09-15.
- Binding match binds the connection destination, never the Host
  header. 2026-09-15.
