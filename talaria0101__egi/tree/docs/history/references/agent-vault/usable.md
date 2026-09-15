# Sweep 2 usable lines: agent-vault

Mechanisms with file and line for the session doing the work. Pin
`f0cdfac` (2026-09-15). Read `findings.md` for verdicts.

## Proxy ingress (T-001)

- `internal/mitm/proxy.go:1-45`: one listener, CONNECT plus
  absolute-form, HTTP/1.1 only, `https://` on the forward path refused
  (no silent TLS-stripping). Port the split and the refusal.
- `internal/mitm/proxy.go:82-100`: upstream transport, strict
  system-trust verification, TLS 1.2 minimum, no HTTP/2. Port the
  posture.
- `internal/mitm/connect.go:22`: `mitmIPKey` uses peer IP. Do NOT copy;
  key limits on session/token identity (issues 380, 381).
- `internal/mitm/connect.go:163-196`: auth-failure recording and 407
  challenge writers. Port the challenge shape.
- `internal/mitm/forward.go:60-83`: absolute-form detection and forward
  handling. Port the detection.
- `internal/mitm/forward.go:308`: compressed bodies skip substitution.
  Port the skip (same as OpenSandbox: compressed and multipart skipped).

## Auth and injection (T-001, T-002)

- `internal/brokercore/proxyauth.go:31-70`: `ParseProxyAuth`, Bearer or
  Basic userinfo, token plus vault hint. Port the shape.
- `internal/brokercore/credential.go:22-33`: `UnmatchedHostPolicy` with
  strict deny. egi defaults to deny.
- `internal/brokercore/credential.go:112-117`: missing row falls to
  policy, any other store error fails closed. Port the distinction.
- `internal/brokercore/credential.go:164-212`: per-key memoization.
  Port it.
- `internal/brokercore/credential.go:219-241`: substitutions before
  auth, errors never expose resolved values. Port the order and the
  rule.
- `internal/broker/broker.go:511-586`: `MatchScore` tuple and selection
  order. Port for cross-tier ranking; keep ambiguity-refusal for
  same-precedence overlaps.
- `internal/broker/broker.go:588-610`: wildcard matches exactly one
  subdomain level. ⚠ Differs from errand's `hostAllowed` (multi-level).
  T-001 keeps errand's multi-level rule; record this divergence here so
  no session "fixes" it by copying agent-vault.
- `internal/broker/broker.go:623-653`: path globs, `*` greedy across
  `/`. Port.

## CA and SDK (T-001, T-018)

- `internal/ca/soft.go:284-398`: root load-or-generate, `RootPEM`,
  `MintLeaf` per SNI, LRU. Port the lifecycle; hold the key in the
  broker process only.
- `sdks/sdk-typescript` README lines 39-47: `caCertificate` plus the
  six-variable trust bundle (`SSL_CERT_FILE`, `NODE_EXTRA_CA_CERTS`,
  `REQUESTS_CA_BUNDLE`, `CURL_CA_BUNDLE`, `GIT_SSL_CAINFO`,
  `DENO_CERT`). Port the bundle.

## Test cases the tracker demands (prove lists)

- axios `https://` on the forward path (issue 366).
- 401 with substitution-bearing service relays body intact (issue 362).
- method restriction per binding (issue 323, T-002 field).
- IPv6 CONNECT (issue 320).
- auxiliary provider hosts in bindings (issue 274).
- GitHub App installation tokens (issue 255, T-018 default).
