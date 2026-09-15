# Sweep 3: OpenSandbox credential vault

What was read, at which commit, what transfers to egi and what does not.
Pin: `8f9b616` (2026-09-15). Tracker: 200 items (57 open plus 16 closed
issues, 43 open plus 84 closed PRs) read;
review comments and discussions not fetched (see `../pins.md`). Code
read: credential vault guide (513 lines), OSEP-0012 (1300 lines, proposal
plus risk table plus design), OSEP-0023 (1109 lines, summary plus goals
plus decision contract), `components/egress/pkg/credentialvault/`
layout (`vault.go` 1319 lines, `decision_snapshot.go`,
`active_socket.go`, `source*.go`, tests). Corpus:
`/workspace/.scratch-mine/opensandbox-group__OpenSandbox/tree/`,
untracked; re-fetch with the commands in `../pins.md`.

Read `../pins.md` for what this sweep did NOT establish before reading
the verdicts below.

## Pass 1: what it is

A Kubernetes sandbox platform whose egress sidecar brokers credentials
for sandbox workloads: host-written credentials and bindings, transparent
MITM interception, exact-match injection, response redaction. The
deepest prior art for egi's broker: a full proposal (OSEP-0012), a
follow-up narrowing TLS interception to credential-bound hosts
(OSEP-0023), and a shipped implementation with e2e tests in five SDKs.

## Pass 2: the construction, at file and line

Lifecycle (guide "How It Works", steps 1-6; OSEP-0012 lines 105-158):
sidecar attaches, SDK writes credentials plus bindings to the sidecar
API, workload runs with fake or empty env values, transparent MITM
inspects metadata, exactly one binding match injects, values redacted
from responses. Active vault served over a Unix socket inside the
sidecar; the workload cannot fetch it over the server proxy path.
Snapshot caching with opaque ETag (`If-None-Match`, 304 reuse, 200
replace); tag separate from public revision so delete-plus-recreate
cannot reuse plaintext; 404 clears the snapshot and continues without
injection.

Availability dependency (guide warning): the private socket is a hard
dependency for ALL intercepted traffic, not just matched flows. Timeout,
refused, EOF, 5xx, malformed snapshot, or bad ETag clears the cache and
fails closed: small buffered bodies get local 503, streaming bodies get
the connection dropped (mitmproxy cannot synthesize mid-stream).
Operators monitor the socket as data-plane availability.

Persistence (guide warning): vault entries are sidecar-process memory,
not filesystem. Kubernetes pause deletes the Pod, so resume starts with
an empty vault; re-inject from the control plane after `Running` before
credential-dependent work. Never persist real values in metadata, env,
snapshots, or logs.

Binding shape (guide "Auth Types" plus "Scoped Placeholder
Substitutions"): bearer, basic, apiKey, customHeaders, passthrough, each
with optional substitutions on path/query/header/body surfaces. Exact,
literal, case-sensitive, one pass over original text, no rescanning of
inserted values. Path rewrites URL-encoded, traversal-rejected; JSON
bodies rewritten as JSON strings; form bodies form-encoded; compressed
and multipart skipped. Placeholder plus every encoding lands in the
redaction set; substitution-miss logs without values.

Sidecar config (guide table): `REQUIRE_TLS`, `TRUSTED_PROXY_CIDRS`,
`REQUIRE_SCOPED_MATCH` (every binding must carry methods plus paths, no
host-wide `/*`). Scoped-match is a fail-closed integration guard.

Implementation (`components/egress/pkg/credentialvault/`): `vault.go`
(match plus inject plus redact), `decision_snapshot.go` (immutable
snapshot, ETag swap), `active_socket.go` (private Unix socket),
`source.go`/`source_inline.go` (MVP: inline values only, no cluster
secret access for sidecars). MITM scripts under
`components/egress/mitmscripts/` (`system.py` request hook,
`tls_shadow.py`, `revision_ipc.py`).

OSEP-0023 (credential-bound TLS interception): decrypt TLS only when the
SNI matches a binding in the active revision; other hosts forwarded as
opaque bytes without presenting a generated cert. Lifecycle, revision
acknowledgement, revocation, drain, and failure semantics defined;
staged opt-in behind current intercept-all default. SNI/ECH/destination
section: ECH hides the SNI the decision needs, which bounds the mode.

Requirements that gate activation (guide list): server `[egress].image`
set, `[egress].mode = "dns+nft"` (refuses DNS-only, direct-IP bypass),
outbound policy present with `defaultAction="deny"`, Credential Proxy
enabled, no second transparent mesh sidecar in the netns.

## Pass 3: how it handles what egi finds hard

The binding shape is the answer to T-002, field for field. The snapshot
plus ETag plus revision-acknowledgement protocol is the answer to
broker-state consistency (mutations succeed only after the proxy
acknowledges; previous revision stays active on failure). The
fail-closed-everywhere posture (socket faults, revision faults,
streaming limits) is the standard T-001/T-002 must meet. OSEP-0023 is
the answer to "MITM decrypts too much": egi takes SNI-gated
interception as the default posture, not the opt-in, since egi has no
legacy intercept-all users.

## Tracker: what broke and what is still open

- 1758 (open, the most load-bearing finding of the whole mining run):
  binding match uses the Host header alone (`pretty_host`), which is
  workload-controlled input. A workload steers an injected credential to
  a host it controls. egi MUST bind the match to the connection's real
  destination (CONNECT target / SNI / resolved peer), never to a
  workload-supplied header. T-002 gains a regression test naming this
  issue.
- 1776 (open): scoped-match enforcement default-off; a projection
  mistake silently creates host-wide injection bindings. egi defaults it
  ON (T-002).
- 1713 (open, now OSEP-0023): intercept-all decrypts non-credential
  traffic. egi takes credential-bound interception as default.
- 1594 (open RFC): egress policy plus vault persistence (on-disk policy,
  encrypted vault) because restart-plus-repush windows sit in
  deny-first outage. egi sessions are single-host and short-lived; the
  control plane re-pushes on start, but T-012 must define the
  broker-unavailable window behavior (fail closed, loud, bounded).
- 1831 (open): `docker commit` snapshot silently omits live files. egi
  does not snapshot containers in v1; if it ever does, verify file
  presence after restore, do not trust the commit exit code.
- 1376 (open): SSH credential injection has no answer (raw keys on the
  filesystem or nothing). egi documents SSH as refused-by-default with
  the same words; a future entry owns the agent-side SSH-cert approach.
- 1841, 1787, 1773, 1768, 1767, 1766: multi-tenancy, execd isolation,
  SDK audit, metrics. Out of egi's scope; noted, not chased.

## Pass 4: verdicts

- **adopt**: the binding shape (five auth types plus scoped
  substitutions); single-match rule; fail-closed socket/revision/
  streaming semantics; snapshot plus ETag plus revision-ack protocol;
  redaction-set discipline; scoped-match default-on; SNI-gated
  interception as default; activation requirements (deny-first policy,
  no second interceptor in the netns).
- **confirms**: egi's fail-closed and write-only-plaintext posture.
- **anti-pattern exhibit**: Host-header-only binding match (issue 1758).
  egi binds to the connection destination; the regression test names
  the issue.
- **filed elsewhere**: persistence-window behavior informs T-012; SSH
  refusal informs the architecture limits; snapshot verification
  informs a future entry, not v1.
- **refused**: the Kubernetes substrate, the sidecar-in-pod shape (egi
  is a single-host daemon; the broker is a host process, not a pod
  sidecar), mesh cohabitation, upstream echo rewriting beyond headers
  (document unsupported, same line).
