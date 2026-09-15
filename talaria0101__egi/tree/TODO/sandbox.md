# Sandbox

## T-003: Sandbox backend trait plus capability probing

**Source:** errand `src/sandbox/backend.ts` (`634b7b9`), bailey probe plus doctor (`d3c73f7`), malaria battery (`499e897`).
**Category:** sandbox
**Priority:** P1
**Effort:** S
**Status:** open

---

### Problem

Backends differ (host processes, containers, microVMs) but the daemon must
treat them alike: start, probe, stop, discover orphans. Without one trait,
every caller branches per backend and a new backend rewrites the daemon.

### Premise

READ, not measured. errand states this trait at
`src/sandbox/backend.ts` lines 133-154 (`Sandbox`: `probe`, `launch`,
`listOrphans`, `removeOrphans`), with `SandboxLaunch` at lines 50-75,
`SandboxHandle` at lines 77-99, `CapabilityReport` at lines 101-111, and
the error types at lines 113-131. Reference pins live in
`docs/history/references/pins.md`.

### Approach

Create `crates/egi-sandbox` depending on `egi-core`. Port the trait:

- `Sandbox`, `SandboxLaunch`, `SandboxHandle`, `CapabilityReport`,
  `SandboxUnavailableError`, `SandboxLaunchError` from
  `src/sandbox/backend.ts` lines 50-154.
- Path constants `WORKSPACE_PATH`, `STATE_PATH`, `AGENT_HOME` from lines
  20-38. Labels and `sandboxName` from lines 41-58 already live in
  `egi-core`; reuse them.
- The agent command builder from lines 167-189 (`agentCommand`): provider
  flag, model flag, system-prompt append, resume flag. In egi the command
  starts the in-daemon agent loop's sandbox endpoint rather than `pi`,
  but the argv shape (provider, model, prompt file, resume) carries over.
- Capability probing reports gaps, never guesses. The bailey `parseDoctor`
  shape at `src/sandbox/bailey.ts` lines 100-131 is the model: parse the
  tool's own report, do not hardcode what it does.

⛔ What it must not do: fall back to another backend or to running
unconfined; present a weaker boundary as a stronger one.

### Prove

```bash
cargo test -p egi-sandbox
```

Passing: trait-object test (a fake backend implements the trait),
gap-report test (a probe fixture with missing delegation reports the gap
and nothing else).

### Closing

Not closed.

---

## T-004: Bailey backend driver with policy generation

**Source:** errand `src/sandbox/bailey.ts` plus `policy.ts` (`634b7b9`), bailey enforce plus profiles (`d3c73f7`).
**Category:** sandbox
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

Sessions on Linux should use the host's own tools under Landlock, seccomp,
and namespaces, with no image to build. The daemon must generate the
policy, build the environment, and refuse when the host cannot hold it.

### Premise

READ, not measured. errand implements this driver in Deno; the port keeps
the policy shape and the probe parsing.

### Approach

Build on T-003 in `crates/egi-sandbox`:

- Port `baileyArgs` at `src/sandbox/bailey.ts` lines 233-268: profile
  selection, policy file placement, read-only root, tmpfs, environment.
  Read the whole file (617 lines) when implementing; the key seams are
  `sessionEnvironment` (lines 77-98), `parseDoctor` (lines 100-131),
  `EGRESS_MAP_ADDRESS` and provider prefix helpers (lines 133-158),
  `providerConfig` (lines 158-190), `BaileyOptions` and
  `ProviderBrokering` (lines 192-231).
- Port the policy generator at `src/sandbox/policy.ts` lines 105-220
  (`policyContents`): project and state placement, profile floor,
  additive `policyExtra` only, absolute paths only. Read
  `AGENT_PROFILE`, `OFFLINE_PROFILE`, `POLICY_FILENAME`,
  `RESOLV_FILENAME`, `RESOLV_CONF` at lines 22-103.
- Read bailey's own `ai-agent.toml` profile
  (`crates/bailey/src/profiles/ai-agent.toml` in QaidVoid/bailey) and the
  `untrusted` floor. Note the documented cost in that file: a partial
  egress allowance gives up the network namespace, so the session shares
  the host's, where UDP, DNS, and loopback stay reachable and only TCP
  ports are enforced.
- Keep the startup report shape from errand `src/daemon.ts` lines 88-130
  (`renderStartupReport`): backend, guarantees, gaps stated plainly, every
  additive grant named, writable grants called out separately.

⛔ What it must not do: accept a whole policy file from config (the report
would claim guarantees the file does not make); inherit the environment
(the session environment is built, not inherited).

### Prove

```bash
cargo test -p egi-sandbox
```

Passing: golden test on the generated policy for a fixture config;
probe-parse test against a real `bailey doctor` fixture; forbidden-grant
test (relative path refused, whole-policy-file refused).

### Closing

Not closed.

---

## T-005: Podman backend driver with measured network flags

**Source:** errand `src/sandbox/podman.ts` (`634b7b9`), malaria `research/podman-machine.md` plus battery (`499e897`).
**Category:** sandbox
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

Sessions that need an assembled filesystem rather than the host's run in
rootless containers. The network flags must actually close the host path,
not merely look like they do.

### Premise

READ, not measured. errand's flags were measured on podman 5.8.2 with
pasta 2025.12.15; malaria re-derived them for the WSL2 machine context.

### Approach

Build on T-003 in `crates/egi-sandbox`:

- Port `podmanArgs` at `src/sandbox/podman.ts` lines 81-147: `--userns=keep-id`,
  `--read-only` with tmpfs at `/tmp`, `:Z` volume suffix on Linux,
  `--workdir`, `--cap-drop=ALL`, `--security-opt no-new-privileges`,
  network selection, memory/cpus/pids limits, fsize ulimit, environment
  with daemon-set names winning. Read the network comment at lines 40-47
  and `FORBIDDEN_ARGS` at lines 50-60.
- The restricted network is
  `pasta:--map-host-loopback,none,--map-guest-addr,none`
  (`RESTRICTED_NETWORK`, line 47). The legacy `--no-map-gw` spelling does
  not close the path. Keep that comment with the constant.
- Read malaria `research/podman-machine.md` (73 lines) before touching the
  Windows path: in the machine context the flags close the distro hop and
  say nothing about the Windows hop across the WSL NAT; the Hyper-V
  firewall decides that and stays reported as a gap.
- Orphan discovery by label (`egi.system=true`, `egi.session=<id>`),
  mirroring errand's `listOrphans`/`removeOrphans`.

⛔ What it must not do: pass a forbidden flag through; let config add raw
container args; claim the Windows hop closed when only the machine hop did.

### Prove

```bash
cargo test -p egi-sandbox
```

Passing: exact-argv golden test for a fixture launch; forbidden-flag test
(each of `FORBIDDEN_ARGS` refused); orphan-label test.

### Closing

Not closed.

---

## T-006: Windows/WSL2 backend report and refusal rules

**Source:** malaria `docs/windows.md` (209 lines), `research/wsl-kernel.md`,
`experiments/windows/battery.ps1`.
**Category:** sandbox
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

The Windows stack differs in three load-bearing ways (firewall hop,
no relabel, bailey ABI), and the daemon must state each rather than guess.

### Premise

MEASURED, by malaria on a stock Windows Server 2025 runner (podman 6.1.1,
kernel 6.18), committed under `experiments/results/`. Re-run the battery
before changing any claim; conditions are in `docs/windows.md`.

### Approach

- Port the report lines from malaria `docs/windows.md` sections "What the
  sandbox can promise on this stack" and "bailey on Windows, and inside
  WSL": the Hyper-V firewall gap, plain `rw` mounts, keep-id mapping,
  OOM exit 137 relay.
- Refuse bailey inside WSL unless the kernel proves Landlock ABI 4 or
  newer. malaria `research/wsl-kernel.md` states the table: ABI 1 on every
  branch, ABI 4 needs 6.7+. A doctor line saying `landlock: ABI 2` passes
  a naive yes/no parse while the kernel silently skips network rules;
  parse the ABI number, do not match on yes/no.
- Document the two arrangements (daemon on Windows vs daemon inside WSL)
  per `docs/windows.md`, recommending project roots inside the distro
  filesystem.
- The binfmt question (WSL interop handler reachable from containers,
  measured failing closed) stays noted as a residual, not a guarantee.

⛔ What it must not do: take bailey's word for ABI support; guess at the
firewall; recommend drive-mount project roots without the measured throughput
cost (about eleven times slower through the drive mount: 69 MB/s against
767 MB/s in the malaria battery).

### Prove

```bash
cargo test -p egi-sandbox
```

Passing: ABI-parse tests (`ABI 2` refuses networked sessions, `ABI 4`
admits them, unparseable output refuses); report test naming the firewall
gap on a Windows fixture.

### Closing

Not closed.
