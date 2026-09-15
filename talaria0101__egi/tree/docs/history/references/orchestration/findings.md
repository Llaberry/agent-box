# Sweep 4: orchestration and adapters (agent-sandbox, CubeSandbox, flue)

What was read, at which commit, what transfers to egi and what does not.
Pins: agent-sandbox `809d3ed`, CubeSandbox `eaddce3`, flue `a20ef15`
(all 2026-09-15). Trackers: agent-sandbox 1634 items (135 open plus 250
closed issues, 78 open plus 1171 closed PRs), CubeSandbox 200 items (41 open
plus 22 closed issues, 59 open plus 78 closed PRs), flue 138 items (34 open
plus 57 closed issues, 4 open plus 43 closed PRs); review comments and discussions
not fetched (see `../pins.md`). Corpus:
`/workspace/.scratch-mine/<owner>__<repo>/tree/`, untracked; re-fetch
with the commands in `../pins.md`.

Read `../pins.md` for what this sweep did NOT establish before reading
the verdicts below.

## Pass 1: what each is

agent-sandbox is a Kubernetes orchestrator (Sandbox CRD plus controller
plus warm pools plus router) delegating isolation to gVisor or Kata. It
is NOT a credential broker; its tracker carries the request for one.
CubeSandbox is a RustVMM/KVM microVM sandbox service with a host-level
transparent egress proxy (CubeEgress, OpenResty plus lua) doing domain
filtering, credential injection, and audit. flue is a TypeScript agent
framework with a sandbox-adapter seam (one factory file per provider),
channel blueprints (Discord over HTTP interactions), and a local sandbox
with an explicit env pass-through rule.

## Pass 2: the construction, at file and line

agent-sandbox README (284 lines): Sandbox CRD (stable identity,
persistent storage, lifecycle), extensions (Template, Claim, WarmPool),
RuntimeClass delegation to gVisor/Kata, router for runtimes without
port-forwarding. Issue 1045 (open): secretKeyRef env injection leaves
the key readable by the agent process; asks for a vault proxy, cites
OpenSandbox's MITM approach. Issue 1580 (open): execution-scoped
authorization (credentials outliving their execution in reusable
sandboxes). Issue 643 (open): core-level network policy without
per-claim NetworkPolicy scaling collapse. Issue 384 (open): env plus
secrets on SandboxClaim with warm-pool injection semantics.

CubeSandbox security-proxy guide (512 lines): CubeEgress TPROXY
listeners (HTTP 8080, HTTPS 8443), eBPF scheme stamping, leaf minting
against a template-baked root CA ("How it intercepts"); first-match-wins
rules with scheme/port/sni/host/method/path match fields ("Domain
filtering"); static header injection after match, `format` defaulting to
`${SECRET}`, inject dropped on deny rules ("Credential injection",
lines 199-244); three audit levels plus `security_event` and
`tls_handshake` shapes with secret scrubbing ("Access auditing", lines
244-296); bypass enumeration: sandbox-to-sandbox traffic, TCP/UDP with
no L7 rule, templates without the CA bake ("When the proxy isn't in the
path", lines 296-320); lua extension points ("Extending the proxy",
lines 321-512). Network-policy (750 lines) and route-aware-egress (270
lines) guides carry the L3/L4 underlay. Tracker: issue 1697 (open):
Host/SNI match does not bind the original destination IP
(`curl --resolve` steering); issue 1458 (open): untyped policy fields
accepted at write, 500 at request time (validate at write, fail the PUT
with the field name).

flue sandboxes guide (224 lines): one environment per agent,
`useSandbox` hook, virtual in-memory sandbox (just-bash, network
opt-in by prefix), `local()` with a short essential-only env pass
(`PATH`, `HOME`, `USER`, `LANG`, `TERM`, `TMPDIR`, never keys;
`env: {...process.env}` called out as trusted-only), provider adapters
created lazily in the factory keyed by agent instance id,
sandbox-provided tool replacement, conditional attach/detach.
`blueprints/sandbox.md` (version 1, 2026-06-14): one-file adapter
contract (`SandboxFactory`/`SandboxDriver`), timeout plus signal
forwarding rules, no-secret-invention rule. `blueprints/channel--discord.md`:
HTTP interactions verification, REST via `@discordjs/rest`, no Gateway
long-lived connection for outbound calls. Tracker (34 open issues):
substantive runtime defects (queued-note loss 679, zero-cost model
synthesis 671, unenforced `allowedTools` 670, unbounded stream growth
658, no per-tool timeout 630) worth egi avoiding by design; sandbox
coverage thin (skill files invisible to sandboxes 595, sandbox blueprint
prompt clarity 582).

## Pass 3: how each handles what egi finds hard

agent-sandbox answers T-025's "when is shared-kernel not enough": the
RuntimeClass seam, with gVisor/Kata runtimes under the same Sandbox
shape. egi defines (not implements) that backend behind its own trait.
Issues 1045 and 1580 confirm egi's broker-plus-execution-scoped-tokens
direction from an independent project.

CubeSandbox answers "what does host-level transparent egress look like
without a sidecar": TPROXY plus eBPF plus OpenResty, with the bypass
enumeration egi must mirror for its own namespace design (what skips
the broker, stated plainly). Issues 1697 and 1458 are the two defects
egi must not re-derive: destination binding and write-time validation.

flue answers "how thin is the adapter seam": one file, one factory,
lazy creation keyed by instance, published surface only. egi's chat
seam (T-013) and backend trait (T-003) take that thinness. The `local()`
env rule (essential-only, explicit opt-in per variable) is the shape
egi's session environment takes minus the secret exception (egi passes
no secret at all, not even opt-in).

## Pass 4: verdicts

- **adopt**: RuntimeClass-style backend seam for T-025 (shape only);
  execution-scoped credential lifetimes (T-018); CubeEgress match-field
  set (scheme/port/sni/host/method/path) for T-002; first-match-wins
  with default-deny for the underlay; audit-level discipline
  (metadata default, bodies planned-not-promised); bypass enumeration
  as a doc section; write-time policy validation; flue's one-file
  adapter thinness; `local()` env discipline; Discord HTTP-interactions
  shape for T-013.
- **confirms**: egi's single-host daemon scope (neither orchestrator
  model transfers); SNI-gated interception default.
- **anti-pattern exhibit**: Host/SNI-without-destination-binding
  (CubeSandbox 1697, same class as OpenSandbox 1758); accept-then-500
  on untyped policy (CubeSandbox 1458); secretKeyRef-into-agent-env
  (agent-sandbox 1045, same class as errand's token-in-env).
- **filed elsewhere**: agent-sandbox 1580 informs T-018 token
  lifetimes; flue 630 informs per-tool timeouts in T-012; flue 670
  informs enforcement (advisory is not enforced) in T-014.
- **refused**: the CRD plus controller plus warm-pool orchestration
  (egi is single-host); the microVM substrate as v1 (backend option,
  T-025); the in-memory virtual sandbox (no durable-work use here);
  KVM host deployment; mesh cohabitation.
