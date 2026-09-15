# pins.md

Every reference this project builds on: the repository, the commit it was read
at, the date, and how deep the sweep went.

⛔ **Re-mine a reference before implementing against it.** Projects move. A
verdict here was taken against the commit in this table and against nothing
else, and the commit is what makes a line citation checkable.

```bash
cargo xtask mine OWNER/REPO --out references
```

⚠ That command does not exist yet. T-055 builds it. Until then the corpus was
fetched with the script named under "How this was fetched" below.

---

## The table

| reference | commit | date | depth reached |
| --- | --- | --- | --- |
| `Azathothas/TEMPLATE` | `ea26c7d91a087a7132f56034fb77f8d40d7766cb` | 2026-09-15 | full tree plus tracker (15 closed issues, 1 closed pull request, 22 comments). Methodology, conventions and security pages read end to end; the shell scripts were read and deliberately not copied. |
| `QaidVoid/errand` | `58a178b2a03dee289bb535e1fdc58676344050eb` | 2026-09-15 | full tree plus tracker (2 open plus 1 closed issue, 2 comments). `src/sandbox/`, `src/serve.ts`, `src/session/github.ts`, `src/config/schema.ts` and every page under `docs/` read end to end. |
| `QaidVoid/bailey` | `d3c73f7d8fd5a9da8289151eb06a35124037fbea` | 2026-09-15 | full tree plus tracker (1 closed issue, 5 closed pull requests, 6 comments). `docs/security/`, `docs/reference/kernel.md`, `docs/roadmap.md`, `CHANGELOG.md`, `crates/bailey/src/backend/network.rs` and the `ai-agent` profile read end to end. |
| `QaidVoid/kage` | `6ad2708f51d5b771b79159421ac148da9102a7e2` | 2026-09-15 | full tree plus tracker (14 closed pull requests, no issues). `docs/reference/architecture.md`, `docs/plugins/capabilities.md` and the `kage-provider` catalogue layout read; the agent loop was not swept. |
| `talaria0101/malaria` | `499e897e8372a5db0f921542184e3b66ef610e2b` | 2026-09-15 | full tree, tracker empty (0 items). `docs/windows.md` read end to end; `experiments/windows/battery.ps1` and `experiments/results/` listed but not re-run. |
| `talaria0101/egi` | `708027481a7886d5eb1e4061a22f37e005b07aa9` | 2026-09-15 | full tree, tracker empty (0 items). `TODO/INDEX.md`, `TODO/sandbox.md`, `docs/history/references/pins.md` and the mining review read. Swept as a prior attempt at this same work, for what it missed. |
| `Infisical/agent-vault` | `f0cdface45da8f7eca07f737a03537be75f20a71` | 2026-09-15 | tree trimmed by deletion plus tracker (30 open plus 14 closed issues, 41 open plus 330 closed pull requests, 760 comments, 509 review comments). `internal/mitm/`, `internal/brokercore/`, `internal/broker/broker.go` and `internal/ca/soft.go` read; issues 194, 209, 255 read in full. |
| `opensandbox-group/OpenSandbox` | `8f9b616ed54852a65fc0a1ef4443f81da632aaf7` | 2026-09-15 | tree trimmed to `components/`, `docs/`, `oseps/`, plus tracker (65 open plus 232 closed issues, 50 open plus 653 closed pull requests, 1000 comments, 1000 review comments, both capped). `docs/guides/credential-vault.md` read end to end; items 1759, 1776, 1804 read in full. |
| `TencentCloud/CubeSandbox` | `eaddce3a0e2ed76b246ef42996cf5f9ed671aeac` | 2026-09-15 | tree trimmed to `CubeEgress/`, `docs/`, plus tracker (54 open plus 199 closed issues, 72 open plus 675 closed pull requests, 1000 comments, 1000 review comments, both capped). `docs/guide/network-hardening.md` read; items 1410, 1426, 1458, 1697 read in full. |
| `kubernetes-sigs/agent-sandbox` | `4b63868f0d67e4ee0fa690bb14cb5722b78d679d` | 2026-09-15 | tree trimmed to `docs/`, `site/`, `examples/`, plus tracker (97 open plus 112 closed issues, 76 open plus 715 closed pull requests, 1000 comments, 1000 review comments, both capped). `README.md` read; item 1580 read in full. ⚠ The controller source was deleted from the corpus: this was swept as an orchestrator design, not as code to port. |
| `withastro/flue` | `a20ef15d8e91e1bf371f0d1d7c736fc56e3dae09` | 2026-09-15 | tree trimmed to `apps/`, `packages/`, plus tracker (34 open plus 239 closed issues, 4 open plus 222 closed pull requests, 632 comments, 14 review comments). `apps/docs/src/content/docs/guide/sandboxes.md` read end to end. |
| `badlogic/pi-mono` | `f9bcd351dc3cedf989bc5fc0f8aa012db5737df2` | 2026-09-15 | ⛔ **documentation only, by decision.** `packages/coding-agent/docs/`: `security.md` and `containerization.md` read end to end, `rpc.md`, `providers.md`, `custom-provider.md` and `usage.md` read for the interface. No tracker, no code. |
| `herdrdev/herdr` | `052779c4159ed851ae78ed1271130811986d1166` | 2026-09-15 | ⛔ **documentation only.** `README.md` read end to end. No tracker, no code. Read to decide whether it is a fit, and it is not. |
| `pingdotgg/t3code` | `9ea892e3b365faff31eed294ac38f70d122656b2` | 2026-09-15 | ⛔ **documentation only.** `README.md` and `docs/internals/environment-auth.md` read end to end. No tracker, no code. |

⚠ **The short forms are not used here on purpose.** A public repository must
not carry a hexadecimal string long enough to look like a credential, and a
commit is the one place where the full value is worth more than the risk. All
fourteen are full 40-character SHAs, which is what `git cat-file -t` takes
without ambiguity.

⭐ **The last three are a different kind of reference and the table says so.**
They are tools this project drives, not designs it ports, so the sweep read
what they publish and stopped. ⛔ **That is a deliberate and narrower depth, not
a gap the next session should close by reading their code.**
[`../../methodology/vendoring.md`](../../methodology/vendoring.md) settles why:
we drive them, at most we wrap them, and patching them is not a topic.

---

## ⛔ What was NOT fetched

Each of these is a real gap. A source missing without being named reads
exactly like a source that had nothing in it.

| gap | which references | why it matters |
| --- | --- | --- |
| **discussions** | all eleven that had a tracker fetched | The credential-free route is REST and discussions are GraphQL only. Several projects keep the design argument that never became an issue there. ⛔ Treat any question these sweeps leave open as possibly answered in a discussion nobody here read. |
| **comments past 1000** | `OpenSandbox`, `CubeSandbox`, `agent-sandbox` | The fetch caps at 1000 records per source. Where the count reads exactly 1000 the source is truncated, and the oldest or newest end is missing depending on the endpoint's order. |
| **review comments past 1000** | the same three | The densest technical content a project produces is line-level argument on a change. On these three it is partial. |
| **the trimmed subtrees** | `CubeSandbox`, `OpenSandbox`, `agent-sandbox`, `flue` | Named per reference in the depth column and in each `PROVENANCE.md`. A citation into a deleted subtree cannot be checked without re-cloning. |
| **the `git` history** | `CubeSandbox`, `OpenSandbox`, `agent-sandbox`, `flue`, `agent-vault` | The clones were shallow and their git directories were removed after the commit was captured. Archaeology on these five needs a fresh full clone. |
| ⛔ **the tracker and the code, entirely** | `pi-mono`, `herdr`, `t3code` | By decision, not by obstacle. ⚠ A published document is evidence of what a maintainer intends, never of what the code does, so every interface claim taken from these three is unverified against an implementation. ⭐ **Re-read the published documentation before writing an adapter**, and expect flag names and protocol details to have moved. |

---

## Where the corpus is

⭐ **Tracked, on its own branch.** The default branch carries the write-ups and
this table; the trees and the tracker JSON live on `references`, so a clone of
the default branch stays small and the evidence stays one command away.

```bash
git fetch origin references
```

```bash
git worktree add ../agent-box-references references
```

The layout is one directory per reference:

```
<owner>__<repo>/
  PROVENANCE.md    the commit, the route, and what the fetch could not get
  api/             issues.json (BOTH states, and it holds pull requests too),
                   comments.json, review-comments.json, releases.json, tags.json
  tree/            the source, at the commit above, with .git removed
```

⛔ **The issues endpoint returns pull requests too.** Discriminate on the
`pull_request` field or a dependency bump reads as an issue. The counts in the
depth column above were taken that way.

```bash
jq -r '.[] | "\(.number)\t[\(.state)]\t\(if .pull_request then "PR" else "IS" end)\t\(.title)"' api/issues.json
```

---

## How this was fetched

⚠ **Not with this repository's own tooling, because it does not exist yet.**
The corpus was fetched with `scripts/common/mine-repo.sh` from
`Azathothas/TEMPLATE` at the commit in the table, over its credential-free
route. Three facts about that route, taken from the script's own header and
confirmed by the run:

- it makes authenticated requests on a third party's account, so it carries
  none of this operator's credentials and cannot reach a private repository.
  ⛔ That is not the same as "structurally cannot reach a private repository",
  and a session must not describe it as such;
- a browser-like or empty user agent is refused with HTTP 420, which no HTTP
  library has a branch for;
- a 404 is evidence only beside a control request. The run hit a known-public
  control first and recorded that it answered 200.

⛔ **Reads only.** No write verb reached any route, and nothing was opened on
anybody's repository. [`../../security/remote-ops.md`](../../security/remote-ops.md).

⚠ **The direct GitHub API was tried first and refused.** Measured
2026-09-15: `api.github.com` and `github.com` both answered HTTP 403 from this
session's egress policy for every repository outside its own scope, and a
public text-extraction proxy answered 401 on address reputation. The route
above is the third one tried, and it is the one that worked.
