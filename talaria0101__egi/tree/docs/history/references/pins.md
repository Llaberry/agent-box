# pins

Every reference egi builds on: the repo, the commit it was read at, the
date, and how deep the sweep went. Re-mine a reference when implementing
against it; projects move and a verdict was taken against a different
commit.

| reference | commit | date | depth reached |
| --- | --- | --- | --- |
| Azathothas/TEMPLATE | `ea26c7d` | 2026-09-15 | bootstrap, methodology, conventions, scripts; scripts copied, rest read |
| QaidVoid/errand | `634b7b9` | 2026-09-15 | full tree plus tracker (2 items); broker, backend, protocol, scheduler, daemon read end to end |
| QaidVoid/bailey | `d3c73f7` | 2026-09-15 | full tree plus tracker; enforce, network, probe, cli doctor, ai-agent profile read |
| QaidVoid/kage | `6ad2708` | 2026-09-15 | full tree plus tracker (14 PRs, no issues); provider registry, SSE, catalog, xtask, plugin API read |
| talaria0101/malaria | `499e897` | 2026-09-15 | full tree, no tracker items; windows guide, sandboxing, all three research notes, reviews read end to end |
| kubernetes-sigs/agent-sandbox | `809d3ed` | 2026-09-15 | shallow tree plus tracker (1634 items: 135 open plus 250 closed issues, 78 open plus 1171 closed PRs, comments); README, issues 1045, 1580, 643, 384 read; code not swept (out of scope, orchestrator not daemon) |
| Infisical/agent-vault | `f0cdfac` | 2026-09-15 | shallow tree plus tracker (171 items: 30 open plus 4 closed issues, 41 open plus 96 closed PRs); README, mitm proxy, brokercore inject, broker matcher, CA, tracker issues read |
| opensandbox-group/OpenSandbox | `8f9b616` | 2026-09-15 | shallow tree plus tracker (200 items: 57 open plus 16 closed issues, 43 open plus 84 closed PRs); credential vault guide, OSEP-0012, OSEP-0023, vault package layout, tracker issues read |
| withastro/flue | `a20ef15` | 2026-09-15 | shallow tree plus tracker (138 items: 34 open plus 57 closed issues, 4 open plus 43 closed PRs); sandboxes guide, sandbox blueprint, discord blueprint, package layout read |
| TencentCloud/CubeSandbox | `eaddce3` | 2026-09-15 | shallow tree plus tracker (200 items: 41 open plus 22 closed issues, 59 open plus 78 closed PRs); security-proxy guide, network-policy guide, CubeEgress layout, tracker issues read |

## What was not fetched

- Discussions on agent-vault, OpenSandbox, flue, CubeSandbox: the miner's
  gh route hit rate limits on the first pass; the second pass used the
  issues endpoint (both states) which needs no GraphQL. Discussions carry
  design argument that never became an issue; treat this as a real gap and
  re-fetch before implementing anything those projects' docs leave unclear.
- Review comments on the same four: same cause, same gap. Issue bodies and
  titles were read; per-line review argument was not.
- agent-sandbox releases and tags: the API returned empty on the first
  pass; versions cited come from the README install block (v1.0.2) instead.
- Full git history on the five shallow clones: `--depth 1` keeps the tree
  at the pinned commit. History archaeology on those five needs a full
  clone.

## Corpus

The trees live at `/workspace/.scratch-mine/<owner>__<repo>/tree/` on the
mining machine, with tracker JSON beside them under `api/`. That path is
NOT tracked and will not survive the machine. Before it goes, either move
the corpus onto a side branch per `docs/methodology/references.md`
section 4, or accept that the next session re-fetches with the commands
below. The pins above plus the commands are the honest trade.

```bash
git clone --depth 1 https://github.com/OWNER/REPO.git
```

```bash
gh api "repos/OWNER/REPO/issues?state=open&per_page=100"
```

```bash
gh api "repos/OWNER/REPO/issues?state=closed&per_page=100"
```
x