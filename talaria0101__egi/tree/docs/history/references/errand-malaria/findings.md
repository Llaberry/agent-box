# Sweep 1: errand and malaria

What was read, at which commit, what transfers to egi and what does not.
Pins: errand `634b7b9` (2026-09-15), malaria `499e897` (2026-09-15).
Tracker: errand carries 2 items (both closed); malaria carries none.
Corpus: `/workspace/.scratch-mine/QaidVoid__errand/tree/` and
`/workspace/.scratch-mine/talaria0101__malaria/tree/`, untracked; re-fetch
with the commands in `../pins.md`.

Read `../pins.md` for what this sweep did NOT establish before reading the
verdicts below.

## Pass 1: what each is

errand is a Deno daemon that runs a coding agent from a chat channel: one
channel, one thread per session, several sessions at once, sandboxed
sessions, an admission queue, a transcript journal, and service
definitions. malaria is a fork-port of errand to Windows: the daemon
cross-compiled to `errand.exe`, containers in a WSL2 podman machine, with
committed experiments and a five-pass review record.

## Pass 2: the construction, at file and line

Broker (`src/sandbox/broker.ts`, 347 lines): `hostAllowed` (line 30),
`ConnectTarget` (line 46), `parseConnect` (line 59),
`ALLOWED_UPSTREAM_PORTS` (line 76), `ProviderRoute` (line 87),
`HOP_BY_HOP` (lines 98-109), `sameSecret` (lines 111-119), `Broker`
(lines 132-290), `readRequestHead` (lines 292-340), `pipe` (lines
342-347). egi-core already ports the pure half; the `Broker` class is
T-001.

Backend trait (`src/sandbox/backend.ts`, 189 lines): path constants
(lines 20-38), labels and `sandboxName` (lines 41-58), `SandboxLaunch`
(lines 50-75), `SandboxHandle` (lines 77-99), `CapabilityReport` (lines
101-111), errors (lines 113-131), `Sandbox` (lines 133-154),
`agentCommand` (lines 167-189). T-003 ports this.

Bailey driver (`src/sandbox/bailey.ts`, 617 lines): `sessionEnvironment`
(lines 77-98, built env, never inherited), `parseDoctor` (lines
100-131), `EGRESS_MAP_ADDRESS` and provider prefix helpers (lines
133-158), `providerConfig` (lines 158-190), `BaileyOptions` and
`ProviderBrokering` (lines 192-231), `baileyArgs` (lines 233-268),
`BaileySandbox` (line 270 to end). Policy generator
(`src/sandbox/policy.ts`, 220 lines): profile constants and filenames
(lines 22-103), `policyContents` (lines 119-220). T-004 ports both.

Podman driver (`src/sandbox/podman.ts`, 257 lines):
`RESTRICTED_NETWORK` (line 47,
`pasta:--map-host-loopback,none,--map-guest-addr,none`),
`FORBIDDEN_ARGS` (lines 50-60), `podmanArgs` (lines 81-147),
`PodmanSandbox` (line 149 to end). T-005 ports this.

Protocol (`src/agent/protocol.ts`, lines 10-208; `framing.ts`;
`client.ts`, lines 46-175): records, commands, dialogs, settlement from
`agent_settled` not `agent_end`, `AgentState`, `AgentProcess`,
`AgentHandlers`. T-011 ports this.

Scheduler (`src/admission/scheduler.ts`, 306 lines): `Clock`, `Ticket`,
`SubmitOutcome`, `QueueEntry`, provider-backoff pause, `Scheduler`. T-016
ports this.

Daemon (`src/daemon.ts`, lines 35-202): `EnforcementGapError`,
`createSandbox` (never falls back), `inertSettings`, `renderStartupReport`,
`probeSandbox`. Config (`src/config/schema.ts`, lines 12-470;
`load.ts`, lines 19-126; `validate.ts`; `redact.ts`, lines 15-77).
Session ops (`src/session/github.ts`, lines 20-176; `pr.ts`, lines
34-456; `manager.ts`; `registry.ts`; `record.ts`; `transcript.ts`;
`disk.ts`; `files.ts`). Chat (`src/chat/gateway.ts`, lines 32-90;
`threads.ts`; `commands.ts`; `inbound.ts`; `outbox.ts`; `render.ts`;
`diff.ts`; `chars.ts`). Delegate (`src/provider/ask.ts`; `delegate.ts`;
`delegation.ts`). Web (`src/web/address.ts`, lines 14-63; `server.ts`;
`view.ts`; `src/serve.ts`). T-010 and T-013 through T-018 port these.

malaria evidence (`docs/windows.md`, 209 lines; `research/wsl-kernel.md`,
62 lines; `research/podman-machine.md`, 73 lines;
`research/runtimes.md`, 64 lines; `experiments/windows/battery.ps1` with
results under `experiments/results/`; `docs/reviews.md`, 113 lines):
restricted flags close the machine hop, Windows hop stays a reported gap;
plain `rw` mounts; keep-id maps to uid 1000; OOM relays 137; drive mount
about 11x slower (69 MB/s against 767 MB/s); WSL binfmt exec fails closed;
Deno kept over Bun for the permission posture. T-006 ports the report
rules.

## Pass 3: how each handles what egi finds hard

Credential handling is the reason for the rewrite: errand hands the agent
the provider credential in the session environment (`SandboxLaunch.env`,
backend.ts line 64; bailey `sessionEnvironment` merges `launchEnv`,
bailey.ts lines 77-98; podman `--env` loop, podman.ts lines 128-134) and
a GitHub token the same way (`TOKEN_VARIABLE = "GH_TOKEN"`,
github.ts line 20). Scrubbing (`redact.ts`, `redacted.ts`) is damage
control the code comments admit a re-encoding defeats. egi brokers both
at egress instead (T-001, T-002, T-018).

The agent runtime is the second reason: errand shells to `pi --mode rpc`
(`agentCommand`, backend.ts lines 167-189). egi drives models.dev
endpoints directly (T-007 through T-010).

## Pass 4: verdicts

- **adopt**: the broker gate shape; the backend trait; the protocol and
  settlement rule; the scheduler; the startup probe/report/refuse
  sequence; the transcript journal; the redaction helper as damage
  control; the malaria Windows report rules and ABI-parse demand.
- **confirms**: nothing yet, egi has no running code beyond egi-core.
- **anti-pattern exhibit**: token-in-session-environment
  (`src/session/github.ts` line 20, `src/sandbox/backend.ts` line 64).
  Kept on purpose: it is the defect this rewrite exists to remove, and
  T-018 carries the proof (no-token-in-env test).
- **refused**: the spawned-CLI agent runtime; the Deno runtime itself
  (Rust per the locked decisions); output masking as a boundary.
