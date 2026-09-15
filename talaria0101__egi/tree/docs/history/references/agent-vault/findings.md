# Sweep 2: agent-vault

What was read, at which commit, what transfers to egi and what does not.
Pin: `f0cdfac` (2026-09-15). Tracker: 171 items (30 open plus 4 closed
issues, 41 open plus 96 closed PRs, mostly dependency bumps and merged fixes). Code read:
`internal/mitm/proxy.go` (186 lines), `connect.go`, `forward.go`,
`brokercore/credential.go` (336 lines), `brokercore/proxyauth.go` (70
lines), `broker/broker.go` matcher (lines 505-660), `ca/` leaf minting,
SDK proxy-env builder. Corpus:
`/workspace/.scratch-mine/Infisical__agent-vault/tree/`, untracked;
re-fetch with the commands in `../pins.md`.

Read `../pins.md` for what this sweep did NOT establish (discussions and
review comments not fetched) before reading the verdicts below.

## Pass 1: what it is

A Go credential broker: one binary as vault plus MITM proxy. Agents hold
dummy values (`__name__`), route HTTP through it, and it substitutes real
credentials on matched requests. Self-hosted (SQLite default, Postgres
for production), with a commercial Agent Proxy sibling for Infisical
platform users.

## Pass 2: the construction, at file and line

Proxy ingress (`internal/mitm/proxy.go`): one listener serves CONNECT
for HTTPS upstreams and absolute-form forward-proxy requests for plain
HTTP on the same port (lines 1-45). HTTP/1.1 only, ALPN pinned (line
40). Upstream transport with strict system-trust verification, TLS 1.2
minimum, no HTTP/2 (lines 82-100). `dispatch` (line 172): CONNECT goes
to `handleConnect`, absolute-form to `handleForward`, everything else
(including `https://` URLs on the forward path, to avoid silent
TLS-stripping) gets 400.

Session auth (`internal/brokercore/proxyauth.go`, whole file): token
plus optional vault hint from `Proxy-Authorization` (Bearer, or Basic
over the proxy URL userinfo so `HTTPS_PROXY=http://token@host:port`
works). Missing or malformed maps to 407 through `ErrInvalidSession`.

Injection (`internal/brokercore/credential.go`, lines 22-260):
`UnmatchedHostPolicy` (`passthrough` default, `deny` strict, lines
22-33); `InjectResult` (lines 35-63, headers marked SECRET-never-log);
`CredentialProvider`/`CredentialStore`/`OAuthStore`/`DynamicCredentialResolver`
seams (lines 65-93); `StoreCredentialProvider.Inject` (lines 112-260):
missing broker row falls to the unmatched policy, any other store error
fails closed (lines 113-117); host/port split before matching (lines
136-141); deterministic matcher (line 144); per-key memoization so a
shared credential decrypts once (lines 164-212); substitutions resolved
before auth so passthrough services still surface missing-credential
errors (lines 222-241); error returns never expose resolved secrets
(comment at lines 219-221).

Matcher (`internal/broker/broker.go`, lines 505-660): `MatchScore`
(host tier, port specificity, literal path length, declaration order;
lines 511-545); selection order exact-beats-wildcard,
port-specific-beats-nil, longest-literal-path, declaration order
(lines 547-586). Wildcard `*.` matches exactly ONE subdomain level
(lines 588-610): `*.github.com` matches `api.github.com` but NOT
`a.b.github.com` or the bare apex. Path globs with `*` greedy across
`/` (lines 623-653).

CA (`internal/ca/soft.go`, lines 284-398): root load-or-generate,
`RootPEM` for distribution, `MintLeaf` per SNI with an LRU cache
(`lru.go`). CA key is a 32-byte process-held value; SDK ships the PEM
to the sandbox for trust (`sdks/sdk-typescript` README lines 39-47:
`caCertificate` plus `buildProxyEnv` setting `SSL_CERT_FILE`,
`NODE_EXTRA_CA_CERTS`, `REQUESTS_CA_BUNDLE`, `CURL_CA_BUNDLE`,
`GIT_SSL_CAINFO`, `DENO_CERT`).

## Pass 3: how it handles what egi finds hard

Dummy-value ergonomics transfer whole: sessions hold placeholders, the
broker substitutes. The strict-deny flip (`unmatched_host_policy=deny`,
403) is the posture egi wants as default. Short-lived per-session
tokens minted by an orchestrator (README SDK section) is the lifecycle
T-018 needs. The matcher priority tuple is a better answer than
OpenSandbox's ambiguity-refusal for overlapping rules, but egi keeps
ambiguity-refusal for same-precedence overlaps (T-002) and takes the
tuple for cross-tier ranking.

## Tracker: what broke, what was refused, what is still open

Open and load-bearing for egi:

- 407 (RFC): per-service MITM request filter sidecar (inspect a push
  body, ask a protection API, 403 or proceed). egi's T-024 approval
  gating is the same seam. Adopt the shape (filter after match, before
  injection), not the code.
- 382: split management and MITM listeners across interfaces/hosts so a
  harness container cannot reach the management port. egi's broker must
  bind the management surface to loopback or a separate interface from
  day one. Adopt.
- 380 plus 381: flood gate keyed on peer IP collapses behind NAT/ingress
  (one client 429s everyone), and denials are silent (no log, no metric,
  no request row). egi keys limits on session/token identity, never peer
  IP, and logs every denial. Adopt both lessons.
- 366: axios sends `https://` URLs on the forward path and gets 400.
  Client coverage matrix needed: git, gh CLI, SDK CA bundles, axios,
  undici. egi's T-001 prove list gains an axios-shaped test.
- 362: 401 relay drops the body on substitution-bearing services
  (Content-Length kept, zero bytes). egi relays status plus headers plus
  body atomically; test the 401-with-substitution case explicitly.
- 323: per-service HTTP method restriction (read-only brokering). egi
  takes this as a binding field from day one (T-002 match includes
  methods), not a later addition.
- 320: no dual-stack outbound (IPv6 CONNECT fails). egi tests IPv6 from
  day one or documents the refusal.
- 315: wrapper overrides NO_PROXY with no opt-out. egi never overrides
  operator proxy env silently.
- 306: Set-Cookie stripping not configurable. egi strips auth-bearing
  response headers and documents the set.
- 274: Claude Code Remote Control fails 403 under brokered OAuth
  (bridge host not in bindings). egi's binding checklist gains the
  provider's auxiliary hosts, not just the API host.
- 258 plus 304: redact brokered credentials from response bodies. egi
  redacts values from headers and bodies it can parse; documents that an
  echoing upstream is unsupported (same line as OpenSandbox).
- 255: GitHub App credential type for proxy-side installation-token
  injection. egi's T-018 takes installation tokens, not PATs, as the
  default.

Closed and useful: 272 (substitution keys in credential-usage checks);
268 (dynamic secrets as leased credentials, the `DynamicCredentialResolver`
seam); 260 (in-place credential store switch).

## Pass 4: verdicts

- **adopt**: dummy-value ergonomics; strict-deny default posture;
  per-session token lifecycle; `Proxy-Authorization` token-plus-hint
  shape; matcher priority tuple; fail-closed store errors; per-key
  memoization; substitutions-before-auth; CA PEM distribution plus the
  six-variable trust bundle; bind-management-off-reach; identity-keyed
  limits with logged denials; method restriction as a binding field.
- **confirms**: egi's HOP_BY_HOP strip and constant-time nonce check
  (same shapes exist here).
- **anti-pattern exhibit**: peer-IP rate limiting behind NAT (issues 380,
  381). Recorded so egi never re-derives it.
- **filed elsewhere**: 407 sidecar-filter RFC informs T-024; axios
  coverage informs T-001 tests; GitHub App tokens inform T-018.
- **refused**: the management UI and commercial Agent Proxy (SaaS control
  plane holding tokens violates egi's self-hosting rule); HTTP/2 and
  WebSocket gaps stay refused until a binding needs them.
