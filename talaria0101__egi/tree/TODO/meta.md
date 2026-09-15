# Meta

## T-019: xtask with ascii, todo-count, and catalog guards

**Source:** kage `xtask/src/main.rs` (`6ad2708`), errand `scripts/check_ascii.ts` (`634b7b9`).
**Category:** meta
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

Three guards need Rust to run: ASCII-only source, TODO counts that agree
with rows, and the catalog freshness check. Shell cannot hold them cleanly.

### Premise

READ, not measured. kage runs codegen and ASCII guards through xtask;
errand checks ASCII through a Deno script. egi unifies them in xtask.

### Approach

Create an `xtask` crate (not published, not in the default workspace
members for builds that matter). Kage's `xtask/src/main.rs` (line 8: fetch
`https://models.dev/api.json`; line 31: `refresh-models`) is the pattern:
subcommands as named functions, `--check` shapes beside the writers.

- `cargo xtask check-ascii`: ASCII only in source, config, docs, commit
  messages surface, log output. Mirror errand's rule (ASCII in source,
  config, docs, commit messages, logs; no em dashes; chat emoji only
  from the enumerated set). Read errand `scripts/check_ascii.ts` first.
- `cargo xtask check-todo`: re-derive every count in `TODO/INDEX.md`
  from the rows and refuse on disagreement; assert no status disagrees
  between the index and the entry, no row names a missing entry, no
  entry lacks a row. This is the reader half of the todo model's
  writer/reader pair (`docs/methodology/work-todo.md`).
- `cargo xtask refresh-models --check`: the catalog freshness half of
  T-009 (offline, against a committed fixture).
- Wire all three into `check-gate.sh` as rows once they exist, and into
  `gates.yml` through `cargo test` neighbours. Until then RULES.md says
  what covers each.

⛔ What it must not do: fetch at check time (offline guards only);
auto-fix (a check repairs nothing by default).

### Prove

```bash
cargo xtask check-ascii && cargo xtask check-todo
```

Passing: both green; mutation-proved (plant a non-ASCII byte, plant a
wrong count, watch each refuse, remove the plant).

### Closing

Not closed.

---

## T-020: experiments directory with first containment probes

**Source:** `docs/methodology/experiments.md`, malaria `experiments/windows/battery.ps1` (`499e897`).
**Category:** meta
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

The design rests on claims (internal networks hold, flags close the host
path, ABI parsing refuses correctly) that must be measured, not cited.

### Premise

READ, not measured. malaria's battery is the model: committed scripts,
committed results, conditions printed on the way out.

### Approach

- Create `experiments/` with numbered scripts per
  `docs/methodology/experiments.md`: `10-probe-the-host.sh` (doctor
  plus container engine versions), `20-internal-network.sh` (prove a
  podman `--internal` network reaches nothing off-host: DNS, IPv6,
  link-local, host loopback, the containers alias), `30-abi-parse.sh`
  (Landlock ABI fixtures through the parser from T-006).
- Each script carries a header stating its question, pins every input,
  prints conditions on the way out, exits 0 measured / 1 thing failed /
  2 could not run, and resolves paths from its own location.
- Negative results commit like successes. A dead end with no record is
  re-attempted by whoever has the idea next.
- Do NOT re-fetch malaria's results; re-run the questions on this
  project's hosts and commit the runs here.

⛔ What it must not do: clean up its own output; quote a number without
its conditions; measure from inside the thing it measures without saying
so.

### Prove

```bash
sh experiments/10-probe-the-host.sh
```

Passing: exit 0 with conditions printed; results committed under
`experiments/results/`; README in the directory naming each script's
question.

### Closing

Not closed.

---

## T-021: CHANGELOG with entries for the first shipped units

**Source:** `docs/conventions/docs.md` changelog rules, template CHANGELOG skeleton.
**Category:** meta
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

`check-changelog` exits 2 with no CHANGELOG.md, which keeps the gate red
under `--strict` and CI. More than that: without a changelog the story of
each fix lands in reference pages and rots them.

### Premise

No premise to check. The gate measures this today: `check-changelog`
reports "no CHANGELOG.md in this repository" and exits 2.

### Approach

- Write `CHANGELOG.md` from the template skeleton: what egi versions
  (daemon plus crates together, one version), newest-first entries,
  every heading dated with a full ISO 8601 UTC stamp, every entry
  naming its record and saying whether it deployed.
- First entries: the skeleton commit (this one), with the commit hash
  as the record and "no version bump and no deploy" as the deploy line.
- Keep `check-changelog` green from then on: every shipping entry
  updates it in the same change as the work.

⛔ What it must not do: tidy old entries in a shipping commit; delete an
entry (amend in place with a dated note).

### Prove

```bash
sh scripts/common/check-changelog.sh
```

Passing: exit 0, entry count reported, order and record and deploy
lines present.

### Closing

Not closed.

---

## T-022: Reference sweeps filed under docs/history/references

**Source:** `docs/methodology/references.md`, T-026 review.
**Category:** meta
**Priority:** P1
**Effort:** M
**Status:** done

---

### Problem

The entries cited references at pinned commits, but no sweep was
filed: no findings file, no usable-lines file, no corpus a later session
could re-check without re-fetching.

### Premise

MINED 2026-09-15. Nine repos at the pins in `docs/history/references/pins.md`.
Methodology deviation, stated honestly: review comments and discussions on
the five outer repos were not fetched (rate limits on the first pass), and
the five outer trees are `--depth 1` clones. The corpus lives untracked at
`/workspace/.scratch-mine/<owner>__<repo>/tree/` with tracker JSON under
`api/`; `pins.md` records the re-fetch commands. Kage's tracker carries PRs
only (14 closed, no issues); malaria's tracker is empty; both verified, not
assumed.

### Approach

Run one sweep per reference group with `scripts/common/mine-repo.sh`,
following `docs/methodology/references.md` end to end (fetch with the
script, three passes over code, tracker in both states with comments
and review comments, keep the corpus, four-part delivery):

1. errand plus malaria (the port): adopt the broker, the backend trait,
   the protocol, the scheduler; refuse the CLI-driven agent and the
   token-in-env session shape.
2. agent-vault plus OpenSandbox credential vault: adopt dummy-value
   ergonomics, binding shape, fail-closed semantics; refuse the SaaS
   control plane and default-allow.
3. kage: adopt the provider registry, the SSE and catalog shapes, the
   xtask pattern; refuse the local-unsandboxed default.
4. agent-sandbox plus CubeSandbox plus flue: adopt the orchestration
   seam (T-025) and the adapter thinness; refuse the CRD model and the
   in-memory sandbox for this daemon's use.

File each under `docs/history/references/<name>/` with findings,
usable lines with file-and-line citations, the instrument where a
claim was measured, and the corpus (or the side-branch pointer plus
the re-fetch command). Open with what was NOT established. Record pins
in `docs/history/references/pins.md` with repo, commit, date, and
depth reached.

⛔ What it must not do: delegate a reference's reading to a sub-agent;
believe a document over its code; silently skip a reference; let a
citation become evidence that egi does what the reference does.

### Prove

```bash
ls docs/history/references/
```

Passing: four sweep directories plus `pins.md`; every entry's citation
resolves to a file and line in the corpus at the pinned commit;
`check-docs` still green (sweeps linked from the history README).

---

### Closing

**Closed 2026-09-15T05:30:00Z.** Four sweep directories plus `pins.md`
filed under `docs/history/references/`, each with findings plus usable
lines at file-and-line depth. `check-docs` green (56 files, 249 links).
Known gaps (review comments, discussions, shallow outer trees, untracked
corpus path) recorded in `pins.md` under "What was not fetched".
INDEX and PROGRESS counts moved in the same change.
