# credential brokers: findings

Three shipped credential brokers, read for what they got wrong in production
and then fixed: `Infisical/agent-vault`, `opensandbox-group/OpenSandbox`
(Credential Vault) and `TencentCloud/CubeSandbox` (CubeEgress).

[`usable.md`](usable.md) is the same sweep for the session doing the work.
[`../pins.md`](../pins.md) is the commits and the depth.

---

## ⛔ What this sweep did NOT establish

- **Nothing was run.** No broker was deployed, no request was proxied, no
  reproduction was re-executed. Every defect below is reported as its reporter
  wrote it, with the named file opened at the pinned commit and confirmed to be
  the code described. ⚠ Whether any of them still reproduces was not tested.
- **Three of the four gaps in [`../pins.md`](../pins.md) bite hardest here.**
  Comments and review comments are capped at 1000 on both `OpenSandbox` and
  `CubeSandbox`, and discussions were not fetched at all. The densest argument
  a project produces is line-level review, and on these two it is partial.
- **The `CubeSandbox` and `OpenSandbox` trees are trimmed.** Only
  `CubeEgress/`, `components/`, `docs/` and `oseps/` survive in the corpus. A
  citation outside those cannot be checked without re-cloning.
- **No comparison was measured.** Nothing here ranks these three by
  performance, reliability or operational burden, because none was run.
- **This is revision 1.** ⛔ Assume more claims are wrong than have been found.

## ⚠ The claims here that are weakest

1. **"agent-vault has no GitHub App support" is a reading of an open issue**
   (255), not an exhaustive grep of the tree. It may exist under another name.
2. **The CubeSandbox plaintext-injection defect is reported closed**, and the
   fix pull request (1411) was read as a title, not as a diff.
3. **The severity ordering below is this sweep's judgement**, not anybody's
   published rating.

---

## ⭐ The one finding that reorders the design

**Only the TLS SNI is an endpoint identity the peer must prove. The `Host`
header and the HTTP/2 `:authority` are input from the sandbox.**

`OpenSandbox` item 1759 states it exactly: Credential Vault selected a binding
from `request.pretty_host`, which is the `Host` or `:authority` header, and
nothing checked it against the identity of the peer the request was delivered
to. A workload could therefore open a TLS session to **any host its egress
policy allowed**, send `Host: <bound host>`, and have that binding's credential
injected into a request delivered to a peer of its choosing.

The reasoning for the fix is the part worth copying:

> The SNI is the right anchor because mitmproxy propagates it to the upstream
> handshake and verifies the upstream certificate against it, so unlike the
> `Host` header it is an identity the peer must prove.

`CubeSandbox` item 1410 is the same defect from the other side, and it is more
instructive because it shows where the asymmetry leaks. The HTTPS path was safe
**by accident of a different control**: upstream certificate verification with
the original SNI meant an attacker IP could not present a valid certificate for
somebody else's name. The plaintext path had no certificate at all, so the same
code injected the operator's credential into a request delivered to an
attacker-chosen IP on the strength of a sandbox-supplied `Host` header alone.
The module's own comments claimed the gate was covered; it was covered on one
of two paths.

⭐ **That is the one-gated-door class, in the exact place this project puts a
credential.** Two paths into one operation, a control on one of them, and the
comment above the code saying it was handled.

---

## The defects, in severity order

Each is a shipped defect with a reporter, a reproduction and a named fix. The
verdict on all of them is **anti-pattern exhibit**: a defect somebody else paid
for is worth more than an absence.

### 1. A credential injected toward an unauthenticated destination

`CubeSandbox` 1410, closed, fixed by 1411 ("require an explicit opt-in before
injecting credentials over plaintext HTTP").

Cited at `CubeEgress/lua/access_phase.lua:215-219` (the only HTTP-path check is
that `Host` is non-empty) and `:296` (the destination IP is recorded for audit
and never compared against anything). The reporter's own driven output:

```
HTTPS, Host==SNI (legit)              allow=true  injected=<the real credential>
HTTP, spoofed Host -> attacker IP     allow=true  injected=<the real credential>
```

⚠ **The reachability note is the part that makes it critical rather than
theoretical**: the destination is only constrained if the network layer
restricts it, and that layer's `AllowInternetAccess` defaults to true when
unset. ⭐ **A broker's guarantee is conditional on the egress policy underneath
it, and a default-open egress policy silently removes it.**

### 2. Binding selection that does not bind the name to the destination

`CubeSandbox` 1697, open.

A matching `Host` or SNI does not establish that the original destination IP
belongs to that name. `curl --resolve` or a hosts entry preserves the allowed
name and changes where the bytes go. The reporter measured 200 for all four
HTTP and HTTPS cases against the baseline.

⭐ **The open design question in that item is the honest part, and this project
inherits it**: proxy-side DNS membership gives false denials under CDNs, split
DNS and rotation; using addresses the network layer already observed needs
provenance and expiry defined. ⛔ **There is no settled answer in any of the
three references.** [`usable.md`](usable.md) says what to do about that.

### 3. Certificate minted before authorization

`CubeSandbox` 1426, closed, fixed by 1427 ("refuse the TLS handshake before
signing when the sandbox is not authorized").

The handshake callback minted a leaf on a cache miss; the authorization gate
ran in a later phase. An unauthorized sandbox therefore forced an ECDSA keygen
and a CA signature per distinct SNI, and evicted every legitimately cached
certificate on the way. Cited at `CubeEgress/nginx.conf:149-171` and
`CubeEgress/lua/cert_signer.lua:97-158`, with the cache keyed on raw
unvalidated SNI at `:163`.

⚠ **Two costs, and the second is the one that is easy to miss**: CPU, and the
eviction of real traffic's certificates so legitimate requests pay a re-sign.

### 4. A policy accepted at admission that fails at request time

`CubeSandbox` 1458, open, with 1459 ("validate match field types, and make
`rule_matches` fail closed").

`validate_policy` checked that `match` was a table and never checked the types
inside it. A policy the admin API returned 200 for then caused an uncaught
error on every request from that sandbox. ⭐ **The reporter's own severity
judgement is the model to copy**: it fails closed, no traffic is allowed
through, so it is availability and diagnosability rather than a policy bypass.
⚠ The operator sees errors with no indication that the policy they installed is
malformed.

### 5. Scope that defaults to everything

`OpenSandbox` 1762 and 1776.

A binding that omits `match.methods` or `match.paths`, or carries the host-wide
`/*`, was normalized to all common methods and every path. The guard exists and
is **opt-in**; the open item asks for the default to flip. The reasoning is
transferable as written:

> That is an easy projection mistake with a costly blast radius, and deployments
> have to discover the env var to protect themselves.

### 6. Credentials in protocol messages, after the upgrade

`agent-vault` 209, closed, and 123 and 225 (the WebSocket substitution
surfaces).

Substitutions worked on `path`, `query` and `header` before the request was
forwarded, and the WebSocket path raw-copied bytes after the upgrade. A
credential sent inside a WebSocket frame, which is how the Discord Gateway
authenticates, was never substituted, so the placeholder reached the wire and
the gateway closed the connection.

⭐ **This is the general shape of "what breaks under interception":** a broker
that rewrites HTTP metadata cannot reach a credential carried in a protocol
message, and the failure surfaces as an authentication error from the upstream
rather than as a proxy error.

### 7. A client that rejects the proxy before the broker ever sees it

`agent-vault` 194, closed.

The Codex CLI's WebSocket transport refused the proxy URL scheme for
`wss://api.openai.com/v1/responses` and fell back to HTTP. The broker was
never reached and never had a chance to fail.

⛔ **The dangerous part is the fallback.** The tool worked, the output was
correct, and the only evidence was an error line in stderr. A broker deployment
that measures only "did the agent succeed" would have called this fine.

### 8. Address bindings that renew themselves

`OpenSandbox` 1804, closed.

Client-observed domain-to-address bindings needed revalidation, with the
invariant stated as: background lookups never add unobserved addresses, policy
replacement clears records and fences in-flight results, DNS failures do not
renew, and successful negative or rotation results stop renewal. Bounded to 128
domains, 64 addresses per domain, four workers, five seconds per lookup, a
20-second batch deadline.

⭐ **Every one of those is a fail-closed clause**, and they are worth reading as
a set: a cache that can be extended by anything other than fresh evidence is a
cache an attacker can pin.

---

## What agent-vault gets right, and the shape to take

⭐ **The substitution model is more general than header injection and it is the
one to build.** `internal/broker/broker.go:62-70` declares a placeholder and,
critically, the **surfaces** it may be written into, with the comment naming
that scoping as the security boundary. `internal/brokercore/substitution.go:16-20`
carries it through to the request.

The encoding is per surface and per content type, which is the part a
reimplementation gets wrong:

| surface | encoding | line |
| --- | --- | --- |
| path | `url.PathEscape`, applied to the wire-encoded path so the value lands exactly once | `substitution.go:47-66` |
| query | `url.QueryEscape` | `:67-70` |
| header | raw, with a CR and LF guard that **refuses** rather than strips | `:71-83` |
| body, form-encoded | `url.QueryEscape` | `:140-149` |
| body, JSON | JSON string escaping | `:151-155` |
| body, multipart | skipped | `:120-124` |

⛔ **The header guard returns an error and the caller must not forward.**
`substitution.go:36-39` says why: partial mutations may already have been
applied.

⚠ **Materializing a body costs memory, and the cap is separate from the request
cap.** `internal/brokercore/session.go:13-18`: 1 GiB for a forwarded body,
64 MiB for one that must be buffered for a body substitution.

The per-tenant model is `internal/brokercore/session.go:20-46`: one
`ProxyScope` resolved per CONNECT, carrying the actor, the vault and the role,
and carried through to injection. ⭐ **`ResolveForProxy` never silently
retargets**: a scoped session whose vault hint does not match its own vault is
an error, not a redirect (`:90-107`).

---

## The question none of the three has answered

**Short-lived, per-tenant credentials minted broker-side.**

`agent-vault` 255 is open and describes exactly the design this project needs:
store a GitHub App identity and private key, sign a short-lived app token,
exchange it for an installation access token, inject that, refresh before
expiry, and never let the agent read either the private key or the derived
token. It is a feature request, not an implementation.

`agent-sandbox` 1580, in [`../orchestration/findings.md`](../orchestration/findings.md),
is the same problem stated more generally and is also open.

⭐ **So this is not a solved problem to port. It is open work, and the entry
that does it is writing something none of the references has.** That is worth
saying plainly, because a sweep that reports "the references do this" when they
do not is how a plan acquires an imaginary dependency.

---

## Verdicts

| subject | verdict | where it lands |
| --- | --- | --- |
| SNI as the only provable endpoint identity | **adopt** | T-021 |
| refuse to inject over plaintext without an explicit opt-in | **adopt** | T-021 |
| authorize before minting a certificate | **adopt** | T-024 |
| validate policy field types at admission, fail closed at request time | **adopt** | T-023 |
| scoped matching on by default: method and path required | **adopt** | T-023 |
| the placeholder-and-surfaces substitution model, with per-surface encoding | **adopt** | T-025 |
| the per-request resolved scope that never retargets | **adopt** | T-022 |
| fail closed when the credential store is unreachable, streams included | **adopt** | T-026 |
| revalidating observed address bindings, with every clause fail-closed | **adopt** | T-027 |
| binding a name to the destination address | ⚠ **open question**, not solved upstream | T-027 records what is undecided |
| broker-minted short-lived per-tenant credentials | ⚠ **open question**, not solved upstream | T-028 |
| credentials inside post-upgrade protocol frames | **adopt** the limit, not a fix | T-029 states it as a documented limit |
| a client that bypasses the proxy and falls back | ⭐ **anti-pattern exhibit** | T-029 |
| an admin API bound to every interface without authentication | **anti-pattern exhibit** | recorded in T-060 |
| mitmproxy, OpenResty, nginx and Lua as the data plane | **refused** | this project is one Rust binary. The mechanisms transfer; the runtime does not. |
