# Broker

## T-001: CONNECT broker that gates hosts and injects provider credentials

**Source:** errand `src/sandbox/broker.ts` (`634b7b9`), agent-vault mitm plus brokercore (`f0cdfac`, sweep 2), OpenSandbox credential vault (`8f9b616`, sweep 3).
**Category:** broker
**Priority:** P1
**Effort:** M
**Status:** open

---

## Problem

A session with a network can dial any host on an allowed port, so the model
provider is indistinguishable from anywhere else on 443. A userspace relay
turns that allowance into a two-way channel.

## Premise

READ, not measured. errand already builds this broker in Deno; the port is
mechanical. Reference pins live in `docs/history/references/pins.md`.

## Approach

Create `crates/egi-broker` depending on `egi-core`. Port the gate from
errand, keeping the seams at the same lines:

- `host_allowed` and `parse_connect` already live in `egi-core/src/sandbox.rs`.
  Reuse them; do not fork them. The ported tests there mirror errand
  `src/sandbox/broker.ts` lines 30-76 (`hostAllowed`, `ConnectTarget`,
  `parseConnect`, `ALLOWED_UPSTREAM_PORTS`).
- Port the `Broker` class at `src/sandbox/broker.ts` lines 132-290: bind a
  loopback listener, read the full CONNECT head before deciding
  (`readRequestHead`, lines 292-340), refuse non-CONNECT with 400, refuse
  off-allowlist with 403, write `200 Connection Established` and copy bytes
  both ways (`pipe`, lines 342-347).
- Port the provider terminating path at lines 139-189: longest-prefix route
  match, constant-time nonce check (`sameSecret`, lines 111-119), strip
  hop-by-hop headers (`HOP_BY_HOP`, lines 98-109), forward with the real
  credential, stream the answer back.
- Keep the byte-at-a-time head reader. Reading past the head strips TLS
  bytes from the tunnel; leaving header bytes in the socket corrupts the
  ClientHello. The comment at lines 292-303 states why.

From agent-vault (sweep 2, `docs/history/references/agent-vault/`): the dummy-value
ergonomics (sessions hold `__name__` placeholders), the strict deny mode
for unmatched hosts, short-lived per-session tokens, the matcher priority
tuple (`internal/broker/broker.go:511-586`), `ParseProxyAuth` token-plus-hint
(`internal/brokercore/proxyauth.go:31-70`), per-key memoization and
substitutions-before-auth (`internal/brokercore/credential.go:164-241`).
⚠ Wildcard divergence: agent-vault `*.` matches exactly ONE subdomain level
(`broker.go:588-610`); egi keeps errand's multi-level rule. Do not copy it.
From OpenSandbox (sweep 3, `docs/history/references/opensandbox/`): fail closed
on broker faults, never forward a request the broker could not decide on,
the snapshot plus ETag plus revision-ack protocol, SNI-gated interception as
default (OSEP-0023).

⛔ What it must not do: read inside established tunnels; log a credential or
a nonce; guess at a target it could not parse; key limits on peer IP (agent-vault
issues 380, 381: shared buckets behind NAT, silent denials); override NO_PROXY
silently (issue 315).

## Decision

Tokio for the async runtime. errand uses Deno TCP; the Rust broker needs an
async runtime and tokio is the standard one. Hyper is not needed for the
CONNECT path (raw sockets plus byte copy); the provider path may use a
minimal HTTP layer. Record the choice in the closing.

## Prove

```bash
cargo test -p egi-broker
```

Passing: all unit tests green, including a CONNECT allow test, a CONNECT
refuse test, a nonce-mismatch 401 test, a head-reader test proving no
byte past the blank line is consumed, an axios-shaped client test
(agent-vault issue 366), a 401-with-substitution relay test (issue 362),
and an IPv6 CONNECT test (issue 320, or a documented refusal).

---

## Closing

Not closed.

---

## T-002: Binding table with exact-match rules and fail-closed semantics

**Source:** OpenSandbox credential vault guide plus OSEP-0012 (`8f9b616`, sweep 3), agent-vault broker matcher (`f0cdfac`, sweep 2).
**Category:** broker
**Priority:** P1
**Effort:** M
**Status:** open

---

## Problem

A broker that only gates hosts cannot scope credentials: one key would ride
every request to a host, including paths and methods it was never meant for.

## Premise

READ, not measured. OpenSandbox documents this binding shape precisely;
the entry ports it. Reference pins live in
`docs/history/references/pins.md`.

## Approach

Build the binding table inside `crates/egi-broker`, beside T-001:

- Each binding names one credential, one match (scheme, host, method,
  path), and one rendering. Read `opensandbox-credential-vault.md`
  sections "How It Works" (the five-step flow), "Auth Types" (bearer,
  basic, apiKey, customHeaders, passthrough), and "Scoped Placeholder
  Substitutions" (the surface table: path, query, header, body).
- Exactly one binding must match. Ambiguity is refused. Port the rule from
  the "Binding Guidance" section: narrow path matches such as `/v1/*`,
  no overlapping bindings at the same precedence.
- Default-deny egress is required. Default-allow stays only as a
  compatibility mode that emits a warning, per the migration notice at the
  top of the guide.
- Port the substitution semantics exactly: disabled by default; exact,
  literal, case-sensitive; configured surfaces only; one pass over the
  original text so inserted values are not rescanned; `Content-Length`
  rewritten and `Transfer-Encoding` removed on body rewrite.
- Fail closed: a broker fault (bad snapshot, refused socket, malformed
  state) refuses the request before any upstream forwarding. Requests with
  bodies above the streaming threshold cannot receive a synthesized 403,
  so the connection drops instead. Either way nothing unjudged is
  forwarded.
- Plaintext values are write-only. `get` and `list` return sanitized
  metadata. Redact values from every response and log line.

From agent-vault: strict deny mode (`unmatched_host_policy=deny`)
rejecting unmatched requests with 403; requests matching no service
forward as plain proxy traffic only outside strict mode.

⛔ What it must not do: infer methods or paths from another platform's
tool definition; forward a secret-bearing URL outside the matched scope
(path traversal, encoded separators); persist real values in sandbox
metadata, env, snapshots, or logs; match on the Host header alone
(OpenSandbox issue 1758, CubeSandbox issue 1697: bind to the connection
destination, the CONNECT target or SNI, never to workload-supplied headers).

## Prove

```bash
cargo test -p egi-broker
```

Passing: route-match tests per auth type, an ambiguity-refuses test, a
no-rescan test (one secret's value containing another's placeholder is
not rewritten twice), a destination-binding regression test (benign Host
header with hostile destination refused; names OpenSandbox 1758 and
CubeSandbox 1697), a write-time validation test (wrong-typed policy
refused at write with the field name; CubeSandbox 1458), a method-scope
test (read-only binding; agent-vault 323), and a fail-closed test (fault
injection refuses without forwarding).

---

## Closing

Not closed.

---

## Closing

Not closed.
