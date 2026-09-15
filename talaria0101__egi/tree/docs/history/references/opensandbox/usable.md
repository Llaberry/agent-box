# Sweep 3 usable lines: OpenSandbox

Mechanisms with file and line for the session doing the work. Pin
`8f9b616` (2026-09-15). Read `findings.md` for verdicts.

## Binding table (T-002)

- Guide "Auth Types": bearer / basic / apiKey / customHeaders /
  passthrough. Port the five renderings.
- Guide "Scoped Placeholder Substitutions" surface table: path (encode,
  traversal-reject), query (encode), header (exclude hop-by-hop and
  security-sensitive, run before auth-header injection), body (JSON
  string / form encoding, skip compressed and multipart). Port exactly.
- Guide "Binding Guidance": default-deny, narrow paths, no overlaps,
  403 for small bodies and connection-drop for streaming on refusal.
  Port the refusal split.
- OSEP-0012 lines 105-158: transparent-proxy lifecycle (create, enable,
  write vault, intercept, match-one, inject, redact). Port the order.
- OSEP-0012 risk table (lines 169-193): bypass, log leak, echo,
  over-permission, drift, redirect, cleartext, CA trust, mutation
  disagreement, sandbox-reached mutation API, IPv6, TLS-verify-off,
  multi-match, long-lived plaintext. Each row is a T-001/T-002 test or
  doc line; walk the table when writing the prove lists.
- `REQUIRE_SCOPED_MATCH` (guide env table): methods plus paths
  mandatory, no host-wide `/*`. Default ON in egi.

## State consistency (T-001, T-012)

- Guide "How It Works" snapshot protocol: opaque ETag,
  `If-None-Match`, 304 reuse, 200 replace, tag independent of public
  revision, 404 clears. Port for broker binding revisions.
- Mutations succeed only after proxy acknowledgement; previous revision
  stays active on failure. Port the transaction rule.
- Private Unix socket for active state, unreachable from the workload
  path. Port the placement (broker state off the session netns).
- Socket faults fail closed for all intercepted traffic; small bodies
  503, streaming drops. Port the split.

## Interception scope (T-001)

- OSEP-0023 summary (lines 48-60): decrypt only when SNI matches a
  binding; other hosts opaque. egi default posture.
- OSEP-0023 "SNI, ECH, and Destination Identity" (lines 713-747): ECH
  hides the decision input. egi documents the bound and refuses
  ECH-obscured credential hosts rather than decrypting blindly.

## The regression test the tracker demands (T-002)

- Issue 1758: match bound to the connection destination (CONNECT
  target / SNI), never to the Host header. The test offers a benign
  Host header with a hostile destination and asserts refusal. Name the
  issue in the test.
