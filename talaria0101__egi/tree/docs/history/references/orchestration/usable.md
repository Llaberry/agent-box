# Sweep 4 usable lines: orchestration and adapters

Mechanisms with file and line for the session doing the work. Pins in
`../pins.md`. Read `findings.md` for verdicts.

## Backend options (T-025, T-003)

- agent-sandbox README "Architecture" plus "Desired Sandbox
  Characteristics": RuntimeClass delegation (gVisor/Kata under one
  Sandbox shape), stable identity, warm pools. Take the seam shape for
  T-025; implement nothing.
- agent-sandbox issue 1580: execution-scoped authorization (provision
  plus revoke per execution in reusable sandboxes). Take the lifetime
  rule for T-018 session tokens.
- CubeSandbox `docs/guide/network-policy.md` (750 lines) and
  `route-aware-egress.md` (270 lines): the L3/L4 underlay beneath L7
  rules. Read before writing T-001's namespace design.

## Egress rules (T-002)

- CubeSandbox `docs/guide/security-proxy.md` "Domain filtering": match
  fields scheme/port/sni/host/method/path, AND'd, absent wildcarded;
  `sni` and `host` exact or leading-`*.`; method OR within the list;
  path exact or single trailing `*` prefix. Take the field set; egi
  paths use OpenSandbox glob semantics (multi-`*`, greedy).
- Same guide "Credential injection" (lines 199-244): inject list after
  match, `format` default `"${SECRET}"`, inject dropped on deny rules,
  secret never in sandbox env/filesystem/process space. Take the order.
- Same guide "When the proxy isn't in the path" (lines 296-320): state
  what skips the broker. egi's architecture gains the same section for
  its namespace design.
- Issue 1697: bind match to the original destination IP, not Host/SNI
  alone. Same regression test as OpenSandbox 1758 (T-002).
- Issue 1458: validate policy field types at write; reject the PUT with
  the field name. T-002 gains a write-time validation test
  (`CubeEgress/lua/policy.lua:82-84` is the shape of the defect).

## Audit (T-001, T-012)

- Same guide "Access auditing" (lines 244-296): metadata default
  (timestamp, addrs, scheme/host/method/path, status, sizes, latency,
  TLS version plus cipher), bodies planned-not-promised, secrets
  scrubbed before write, `security_event` and `tls_handshake` shapes.
  Take the level discipline and the event split.

## Chat and env seams (T-013, T-004)

- flue `blueprints/sandbox.md`: one-file adapter, lazy creation keyed
  by instance id, timeout plus signal forwarding, no invented secrets.
  Take the thinness for T-003/T-013.
- flue sandboxes guide "The local sandbox": essential-only env pass,
  explicit per-variable opt-in, `...process.env` marked trusted-only.
  Take the discipline; egi passes no secret even opt-in.
- flue `blueprints/channel--discord.md`: HTTP interactions verify,
  REST via `@discordjs/rest`, no Gateway connection for outbound. Take
  the shape for T-013.
- flue tracker 630 (per-tool timeout), 670 (advisory is not enforced):
  every tool gets a timeout (T-012); every documented capability is
  enforced or documented as not (T-014).
