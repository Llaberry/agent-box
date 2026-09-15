# Tooling

The probe, the gate, the checks, and the miner. All Rust, all in `xtask`, none
of them shipped in the binary.

[`INDEX.md`](INDEX.md) is the list.
[`../docs/agent-tooling.md`](../docs/agent-tooling.md) is the catalogue these
entries fill in.

---

## ⛔ The contract every check here satisfies

Five points, and a check that misses one is not a check:

1. a header saying **what defect it catches**, not what it does;
2. ⛔ **exit 0 pass, 1 fail, 2 could not run.** A skip is a skip, never a pass;
3. a `--json` switch, so a runner can read it;
4. no dependence on the directory it runs from;
5. ⭐ **mutation-proved**: the defect it exists to catch is planted, the check is
   watched refusing it, and the closing records the planted diff and the exit
   code read unpiped.

⚠ **This is lens 2 of
[`../docs/methodology/reviews.md`](../docs/methodology/reviews.md)**, and its
worked example is the argument: a scan reported "no orphans" over the exact
orphan it existed to find, twice.

---

## T-050: `cargo xtask doctor`

**Source:** `Azathothas/TEMPLATE` `scripts/doctor/` (`ea26c7d`); `QaidVoid/bailey` `bailey doctor` (`d3c73f7`).
**Category:** tooling
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-011

---

### Problem

Every session, and the daemon at startup, needs to know what this host can
actually do. A stated environment and a measured one that disagree is a finding,
and it is cheaper to find at the start than at a gate.

### Premise

READ. The template ships a probe in two implementations, and its contract is
that it is **read-only, exits 0 whether or not anything is missing, and makes no
network call unless asked**. ⭐ A probe is not a gate.

### Approach

`xtask doctor` reports, and `agent-box doctor` calls the same code
([`../docs/conventions/code.md`](../docs/conventions/code.md): one read path):

- the host, the kernel version, the architecture;
- ⛔ **every capability from T-011, by attempting it**, with what the attempt
  returned;
- which backends can run here, and at what version;
- the toolchain versions, and ⚠ a note when the local one is behind what CI
  installs;
- what the repository declares: the workspace members, whether the tree is
  clean, the current commit.

⛔ **Read-only. Exits 0 whether or not anything is missing.** A host with no
podman is not a failing build.

### Prove

```bash
cargo xtask doctor --json | python3 -c 'import json,sys; json.load(sys.stdin)'
```

⚠ **That command pipes, which hides the exit code.** Run it for the schema, then
read the code from the process that produced it:

```bash
cargo xtask doctor --json > /dev/null; echo $?
```

Passing: valid JSON, exit 0 on a host missing a backend, and this entry's
closing carrying the actual output from the machine it ran on with its kernel
version.

### Closing

Not closed.

---

## T-051: `cargo xtask gate`

**Source:** `Azathothas/TEMPLATE` `scripts/common/check-gate.sh` (`ea26c7d`); [`../docs/methodology/gate.md`](../docs/methodology/gate.md).
**Category:** tooling
**Priority:** P1
**Effort:** S
**Status:** open
**Blocked by:** T-052, T-053, T-054

---

### Problem

Part (a) of the gate is a **list**, and a list run by hand is run in the order
somebody recalls it. One session ran its gate five times and typed a different
subset each time: nothing failed, and it was not the same gate twice.

### Premise

READ. The template's runner prints a skipped check as a skip on its own line and
carries a `--strict` switch that turns a skip into a failure, on the stated
ground that CI installs the tools on purpose so a skip there means the install
broke.

⭐ **And zero passes is red.** A runner that finds nothing to run and exits 0 is
the shape of
[`../docs/conventions/forbidden-patterns.md`](../docs/conventions/forbidden-patterns.md)'s
"a step that exits 0 having done nothing it was asked to do".

### Approach

`xtask gate` runs, in this order, and prints one verdict:

```
fmt   clippy   test   check docs   check markers   check one-home
check control-bytes   check secrets   check attribution   check record
check layering   check spawn-cwd
```

- ⛔ **every exit code is read from the process that produced it, unpiped**
  ([`../docs/conventions/shell.md`](../docs/conventions/shell.md) section 2);
- ⛔ **a skipped check prints as a skip, never as a pass**;
- ⛔ **zero checks run is a failure**;
- `--strict` turns a skip into a failure;
- ⭐ **the test count is compared against the test files on disk.**
  [`../docs/methodology/gate.md`](../docs/methodology/gate.md) says what that
  catches.

### Prove

```bash
cargo xtask gate --strict
```

Passing: exit 0 on a clean tree. ⛔ **And mutation-proved**, four ways, each
recorded in the closing with its exit code:

| plant | expected |
| --- | --- |
| a failing test | exit 1, naming it |
| a check binary made unavailable, without `--strict` | exit 0, and the skip printed on its own line |
| the same, with `--strict` | exit 1 |
| every check filtered out | ⛔ exit 1, not 0 |

### Closing

Not closed.

---

## T-052: The document and structure checks

**Source:** `Azathothas/TEMPLATE` `scripts/common/check-docs.sh`, `check-markers.sh`, `check-one-home.sh`, `check-control-bytes.sh` (`ea26c7d`).
**Category:** tooling
**Priority:** P1
**Effort:** L
**Status:** open
**Blocked by:** T-001

---

### Problem

Documents rot silently. A broken link, a page nothing links to, a character
outside the allowed set, a sentence that now lives in two files: none of these
fails anything, and each is how a document set stops being trusted.

### Premise

READ, and two of the numbers are measured by the source.

⭐ **The marker ceiling is 30 markers per 100 non-blank lines**, and
`../docs/conventions/prose.md` carries the measurement behind it over three
trees on 2026-08-28: the tree that read worst scored 38.6, the template 9.0, the
one that read best 8.6. ⚠ **Two of those trees had been ranked by eye before
anything was counted, and the ranking came out in that order.**

⛔ **The allowlist covers every tracked text file, not only markdown.** On the
day it was widened over that template, its own scripts held **2290** characters
outside the five across 22 files. ⚠ Nothing like it has been measured here.

⭐ **`check-one-home` refuses a sentence of 12 words or more appearing in two
documents**, and its first run over that template found **42** duplicates.

### Approach

`xtask check <name>`, five subcommands sharing one file walker:

| subcommand | refuses |
| --- | --- |
| `docs` | a relative link that does not resolve, a cited path that does not exist, a fenced shell block that does not parse, an angle-bracket placeholder inside a shell block, a page under `docs/` that nothing links to |
| `markers` | a character outside `⛔ ⭐ ⚠ ✅ ❌`, an em dash, and more than 30 markers per 100 non-blank lines in any tracked text file. ⭐ A specimen inside a code span or a fenced block is permitted, and must be: a page banning a character cannot otherwise show which one |
| `one-home` | a sentence of 12 or more words in two documents. ⚠ Two exemptions, both narrow: the entry-point routers are exempt from each other, and `docs/history/` is exempt entirely |
| `control-bytes` | a literal control byte in a tracked text file |
| `layering` | a workspace dependency pointing upward, and `box-core` importing `std::fs`, `std::net` or `std::process` |

⛔ **And one this project needs that the template does not have**, from T-030:

| `spawn-cwd` | a spawn constructed with a working directory under the project root or the session state root |

⚠ **The allowlist applies to the matched characters, not to the line.** The
template's own forbidden-patterns table carries the exhibit: `grep -nP <banned>
| grep -vP <allowed>` passed a line reading "never use <banned>", because
`grep -v` drops lines rather than characters.

### Prove

```bash
cargo xtask check docs && cargo xtask check markers && cargo xtask check one-home && cargo xtask check control-bytes && cargo xtask check layering && cargo xtask check spawn-cwd
```

Passing: every one exits 0 over this tree. ⛔ **And each is mutation-proved**,
with the plant and the exit code in the closing:

| plant | in | expected |
| --- | --- | --- |
| a link to a file that does not exist | any page | `docs` exits 1, naming file and line |
| a new page nothing links to | `docs/` | `docs` exits 1 |
| an em dash | any tracked text file | `markers` exits 1 |
| a marker density over the ceiling | one page | `markers` exits 1 |
| a 15-word sentence duplicated into a second document | two pages | `one-home` exits 1 |
| the same duplicated into `docs/history/` | | ⭐ `one-home` exits **0**, correctly |
| a literal escape byte | any tracked text file | `control-bytes` exits 1 |
| `box-policy` depending on `box-session` | `Cargo.toml` | `layering` exits 1 |
| a spawn with a project-root working directory | `box-session` | `spawn-cwd` exits 1 |

### Closing

Not closed.

---

## T-053: The secret and attribution checks

**Source:** `Azathothas/TEMPLATE` `scripts/common/check-no-secrets.sh`, `check-attribution.sh`, `dotfiles/githooks/commit-msg` (`ea26c7d`); [`../docs/security/secrets.md`](../docs/security/secrets.md).
**Category:** tooling
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-001

---

### Problem

⛔ **This repository is public.** A credential, or a fingerprint of a private
system, that reaches it is published, cached and possibly indexed, and a history
rewrite does not un-publish it.

And a commit crediting a tool arrives from a default nobody chose, so it arrives
on every commit of a session rather than on one.

### Premise

READ. The template holds both, and the attribution rule is held in **two**
places that speak at different times: a `commit-msg` hook that refuses **before
the commit exists**, and a check over `git log` that reports the ones that got
in. ⚠ **The check cannot prevent what it names**, and that asymmetry is
structural rather than a defect: every session's procedure is to run the gate
and then commit, so the commit being made does not exist while the check runs.

⛔ **Hooks are not cloned**, so a fresh checkout has none. The check prints the
one command that arms them when it finds the hook missing.

### Approach

| subcommand | refuses |
| --- | --- |
| `secrets` | a credential shape, a private-system fingerprint, or an address, in tracked **and untracked** files. ⚠ It narrows the reading; it does not replace it. A password that looks like a word and a hostname that looks like prose are not findable this way, and the check's own output says so. |
| `attribution` | a commit in `git log` crediting a tool: a co-author trailer naming a model or a vendor, a generated-with line, a tool name in the body |

⛔ **`secrets` scans untracked files too.** A credential in an untracked file is
one `git add -A` from being published.

⛔ **Neither check ever prints a matched value**, only the file, the line and
the shape that matched.

`attribution` prints the arming command when the hook is missing:

```bash
git config core.hooksPath .githooks
```

⚠ **Refuse the commit rather than rewriting the message.** Editing somebody's
commit message on their behalf is worse than declining to make the commit.

### Prove

```bash
cargo xtask check secrets && cargo xtask check attribution
```

Passing: both exit 0 over this tree and this history. ⛔ **Mutation-proved:**

| plant | expected |
| --- | --- |
| a long hexadecimal string in a tracked file | `secrets` exits 1, ⭐ **printing the file and line and not the value** |
| the same in an untracked file | `secrets` exits 1 |
| an address in a tracked file | `secrets` exits 1 |
| a commit with a co-author trailer naming a model | `attribution` exits 1 |
| the same message through the hook | ⭐ the commit is **refused before it exists** |
| the hook unarmed | `attribution` prints the arming command |

⚠ **The planted secret is a fabricated string that grants nothing**, and the
closing says so. [`../docs/security/secrets.md`](../docs/security/secrets.md) is
absolute that a real one never enters the tree, ⛔ **including for a test**.

### Closing

Not closed.

---

## T-054: `cargo xtask check record`

**Source:** [`../docs/methodology/work-todo.md`](../docs/methodology/work-todo.md), "The counts have to agree with the rows".
**Category:** tooling
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-001

---

### Problem

⭐ **The todo model's one mechanical hazard.** Closing one entry moves the index
total, the open and done figures beside it, one row of the priority table, that
row's total, the overall row, and the record's own count lines.

### Premise

READ, with a cost recorded in
[`../docs/methodology/work-todo.md`](../docs/methodology/work-todo.md). ⛔ **Its
conclusion is this entry's whole reason to exist:** nothing was wrong with any
single file, and what was missing was anything that compared two of them.

### Approach

Two subcommands, and they are different jobs:

| subcommand | does |
| --- | --- |
| `xtask record set T-NNN <status>` | moves a status in the entry and re-derives **every** count from the rows. ⛔ Never retype a count. |
| `xtask check record` | asserts, independently, that the counts agree with the rows |

`check record` asserts all of these:

- the counts in `INDEX.md` agree with its own rows;
- the priority table's rows and totals agree with the entries;
- no status disagrees between the index row and the entry;
- no index row names a missing entry, and no entry lacks a row;
- every relative link in `TODO/` resolves;
- ⭐ **every cited path exists, and every cited line range is within that
  file's length**, for citations into this tree. ⚠ Citations into the corpus are
  checked only when the `references` worktree is present, and reported as
  skipped when it is not;
- ⛔ **a quoted claim is matched with whitespace normalized.** ⚠ **Found on
  2026-09-15**: a claim audit reported four quotations absent that were present
  and verbatim, because the source wraps them across lines. ⭐ **A citation check
  that matches raw text produces false failures, and a reviewer who trusts it
  edits a correct quotation into a wrong one.** Collapse runs of whitespace on
  both sides before comparing;
- ⛔ **every entry identifier written anywhere in the tree resolves to an entry.**
  ⚠ **Found by a door sweep on 2026-09-15**: two identifiers in the shape of this
  project's own referred to a different project's entries, in a page about that
  project and in inherited prose. Both read as dangling references here. ⭐ **The
  rule that follows is for writers, not for the check: never write a bare foreign
  identifier.** Name whose it is, or do not use the shape;
- `PROGRESS.md` carries a state line with an ISO 8601 UTC instant, and its
  counts match the index.

⚠ **Write a quoted number as a fixed line the checker parses**, rather than as
prose that reads better. The prose version is the one that goes stale silently.

### Prove

```bash
cargo xtask check record
```

Passing: exit 0 over this tree. ⛔ **Mutation-proved**, with each plant and exit
code in the closing:

| plant | expected |
| --- | --- |
| an index count off by one | exit 1, naming the count and the derived value |
| an entry marked done with its index row saying open | exit 1, naming both |
| an index row naming an entry that does not exist | exit 1 |
| an entry with no index row | exit 1 |
| a citation to a line past the end of a file in this tree | exit 1 |
| the `references` worktree absent | ⭐ corpus citations reported as **skipped**, exit 0 |

### Closing

Not closed.

---

## T-055: `cargo xtask mine`

**Source:** `Azathothas/TEMPLATE` `scripts/common/mine-repo.sh` (`ea26c7d`). Its header is required reading before this entry starts.
**Category:** tooling
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-001

---

### Problem

Studying another project needs its tree **and** its tracker, kept where a later
session can find them. Two sweeps have lost the same work in opposite ways:
one kept eleven conclusions and deleted eleven clones; one wrote its own
fetchers, ran them, and deleted both the data and the fetchers because they
lived in session-local scratch.

### Premise

⭐ **MEASURED by the source, and three of its measurements contradict what its
route's description is usually quoted as saying.** From its header, taken on
2026-08-28:

- ⛔ **the credential-free route is not unauthenticated.** It makes
  authenticated requests on a third party's account. What it gives is a route
  carrying none of **your** credentials. ⚠ That is not the same as
  "structurally cannot reach a private repository", and a session must not tell
  an operator that it is;
- ⚠ its route set is wider than `/repos/*`: `/users/*`, `/orgs/*`, `/search/*`
  and `/rate_limit` all answer 200, and `/user` is refused. **The boundary is
  the caller's identity, not the path prefix**;
- ⛔ **a browser-like or empty user agent is refused with HTTP 420**, which no
  HTTP library has a branch for, so a client special-casing 401, 403 and 404
  reports it as a network error;
- ⭐ **a 404 is evidence only beside a control.** Neither route can see a
  private repository, so a 404 means "not public" and equally means "the route
  is down". Hit a known-public control in the same run.

And one defect its own history carries: an earlier page joiner concatenated
pages and recovered array bounds by counting brackets over raw text, which
counts brackets inside string values. ⛔ **Comment bodies are markdown, so they
carry brackets in links and in pasted logs**, and an entire comment corpus was
discarded for months while every check passed, because nothing compared it.

⭐ **This project verified the route works**: it was used to fetch the corpus
behind [`../docs/history/references/`](../docs/history/references/) on
2026-09-15, after the direct API answered 403 and a public text proxy answered
401.

### Approach

`xtask mine OWNER/REPO --out references`, writing per reference:

```
<owner>__<repo>/
  PROVENANCE.md    the commit, the date, the route, the control's answer,
                   and ⛔ what it could NOT get
  api/             issues.json (BOTH states, holding pull requests too),
                   comments.json, review-comments.json, releases.json, tags.json
  tree/            the clone, with the commit captured BEFORE anything is stripped
```

⛔ **Four rules, each with a cost behind it:**

1. **capture the commit before stripping**; once the git directory is gone it is
   unrecoverable and every citation becomes uncheckable;
2. **trim by deleting, never by moving**; a trim that rewrites paths invalidates
   every citation already written, including the ones in the write-up still
   being written;
3. ⛔ **join pages with a real parser**, never by counting brackets;
4. **hit a control in the same run**, and write which it was into the
   provenance.

⛔ **Reads only.** No write verb on either route, ever
([`../docs/security/remote-ops.md`](../docs/security/remote-ops.md)).

⚠ **Record what it could not get**, including the pagination cap. The corpus
this project already has was capped at 1000 comments on three references, and
that is written down rather than left to look like completeness.

### Prove

```bash
cargo xtask mine --selftest
```

Passing: exit 0. ⭐ **The self-test runs offline and proves the page joiner**,
which is the part with a known defect: a record whose body contains an unmatched
bracket in a fenced block joins correctly across three pages, an empty page over
records is refused, and an empty page over nothing is accepted.

⛔ **And a live run against one public repository**, whose `PROVENANCE.md` names
the control's answer and at least one real gap. That output goes in the closing.

### Closing

Not closed.

---

## T-056: One bootstrap command that prepares a session's environment

**Source:** the operator, 2026-09-15.
**Category:** tooling
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

A session starts on a machine that may have nothing: no toolchain, no backend,
no corpus. Every session doing that setup by hand does it differently, and the
differences are invisible until something behaves unlike the last session.

### Premise

⭐ **RULED by the operator, 2026-09-15**: one entry point that a session runs at
its start, wiring together the per-concern pieces.

⚠ **This is the one place shell is correct in this repository**, and the reason
is specific rather than a preference: the bootstrap runs **before** the
toolchain exists, so it cannot be the Rust `xtask` that every other check is.
⛔ **The boundary is exact**: bootstrap is shell because it precedes `cargo`;
every check is Rust because it does not.
[`RULES.md`](RULES.md) section 5 records the decision.

### Approach

```
bootstrap/
  bootstrap.sh          the one entry point. POSIX sh.
  bootstrap.ps1         its twin, for Windows hosts
  parts/
    10-toolchain.sh     the pinned Rust toolchain, if absent
    20-backend.sh       the Linux confinement backend, at or above the floor
    30-corpus.sh        the references worktree, if wanted
    40-hooks.sh         git config core.hooksPath .githooks
```

⛔ **Six rules, and each has a cost behind it somewhere in this repository:**

1. **Idempotent.** Running it twice changes nothing the second time. A session
   that cannot safely re-run it will avoid running it.
2. ⛔ **Report, never guess.** It prints what it found and what it installed. A
   part that could not run exits 2 and says so, and ⛔ **"could not run" is never
   reported as a pass**
   ([`../docs/methodology/gate.md`](../docs/methodology/gate.md)).
3. ⛔ **Nothing is installed silently outside the repository** without saying so
   first. [`../docs/agent-tooling.md`](../docs/agent-tooling.md): an install is a
   system change nobody asked for, on somebody else's machine, that outlives the
   session.
4. ⛔ **`--check` does nothing and reports what would happen.** That is the mode
   a session runs first.
5. ⛔ **The backend version floor is enforced here too**, not only at the probe.
   Installing a version this project refuses is worse than installing none.
6. ⚠ **Both halves stay in step.** Two implementations of one procedure drift,
   and this repository has no twin-checking guard because it deliberately ships
   only one implementation of everything else. ⭐ **So the parts are data where
   they can be**, and the two entry points read the same manifest rather than
   each carrying their own list.

⛔ **What it must not do:** fetch anything from a moving reference. Pin a
release tag or a commit; a moving reference runs code nobody reviewed
([`../docs/containers.md`](../docs/containers.md)).

### Decision

**Whether the bootstrap installs a toolchain, or only reports it is missing.**

⭐ **Recommendation: report by default, install behind an explicit flag.** A
session on an operator's own machine must not silently install a toolchain; a
session on a throwaway host wants exactly that. The flag makes the difference
the caller's decision rather than this script's guess.

### Prove

```bash
sh bootstrap/bootstrap.sh --check
```

Passing: exit 0 on a host that has everything, exit 2 naming each missing part
on a host that does not, and ⛔ **nothing changed on disk in either case**,
asserted by comparing a file listing and the git status before and after.

⛔ **And mutation-proved:** a part made to fail makes the entry point exit
non-zero naming that part; a backend below the floor is refused rather than
accepted; a second run reports no change.

### Closing

Not closed.
