# Broker

The CONNECT proxy, the injection gate, the certificate authority, and the
credential store.

[`INDEX.md`](INDEX.md) is the list.
[`../docs/credential-brokering.md`](../docs/credential-brokering.md) is the
design. ⭐ **Read [`../docs/history/references/brokers/usable.md`](../docs/history/references/brokers/usable.md)
before starting any entry here.** Nearly every rule below is a defect somebody
else shipped.

---

## T-020: The CONNECT proxy

**Source:** `QaidVoid/errand` `src/sandbox/broker.ts` (`58a178b`); `Infisical/agent-vault` `internal/mitm/` (`f0cdfac`).
**Category:** broker
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-001 (and its async decision, which this entry cannot start without)

---

### Problem

A session has one route off the host. Something has to be at the other end of
it, gate what it opens, and copy bytes.

### Premise

READ, not measured. Two implementations, and the errand one is small enough to
port line by line.

errand, `src/sandbox/broker.ts`, 347 lines:

| what | lines |
| --- | --- |
| `hostAllowed` | 20-46 |
| `parseConnect` | 48-75 |
| the upstream port list | 76 |
| `ProviderRoute` | 79-96 |
| the hop-by-hop list | 98-110 |
| `sameSecret`, a constant-time comparison | 112-119 |
| the `Broker` class | 132-205 |
| ⭐ `readRequestHead` | 296-326 |
| `refuse`, `pipe` | 328-347 |

agent-vault, for the shapes errand does not have:

| what | file and lines |
| --- | --- |
| the two request shapes on one listener | `internal/mitm/proxy.go:1-27` |
| ⛔ every error written **before** the hijack | `internal/mitm/connect.go:74-88` |
| the one-shot listener that serves HTTP off the terminated TLS connection | `internal/mitm/connect.go:199-238` |
| the timeouts, with what each protects | `internal/mitm/connect.go:139-154` |
| the upstream transport bounds | `internal/mitm/proxy.go:83-91` |

### Approach

In `box-broker`:

- bind loopback, or the link-local address the backend routes to. ⛔ **Refuse a
  public bind rather than warning about it.** One reference ships seven
  management services defaulting to every interface, most with no
  authentication (`TencentCloud/CubeSandbox` `docs/guide/network-hardening.md`);
- ⭐ **read the CONNECT head one byte at a time, up to and including the blank
  line.** errand's comment at `broker.ts:296-310` is the reason and it is worth
  porting verbatim: reading past the head strips bytes the client expects to
  carry its TLS, and reading only the request line leaves header bytes in the
  socket that are then piped to the upstream **ahead of the ClientHello**. Cap
  the head at 8192 bytes; accept `CRLFCRLF` and a bare `LFLF`;
- ⛔ **refuse a target you had to guess at**: a missing port, a non-numeric
  port, a port out of range, a bracketed authority, a slash in the host;
- ⛔ **strip hop-by-hop headers in both directions**: `connection`,
  `keep-alive`, `proxy-authenticate`, `proxy-authorization`, `te`, `trailer`,
  `transfer-encoding`, `upgrade`, `host`. ⚠ Forwarding `proxy-authorization`
  upstream sends the caller's own proxy credential to the origin;
- ⭐ **longest prefix wins** when routes are nested (`broker.ts:162-166`), or a
  route under another's path is silently unreachable;
- a wildcard does not match its own apex, and comparison is case-folded
  (`broker.ts:20-46`);
- ⛔ **every error response is written before the connection is hijacked.** Once
  hijacked, no status can be sent (`connect.go:74-88`);
- the timeouts from [`../docs/credential-brokering.md`](../docs/credential-brokering.md).
  ⚠ **A model turn and a repository clone are both long streaming transfers**,
  and one reference shipped a 60-second read timeout baked into its image that
  cut off real model requests (CubeSandbox 1650, fixed by 1655 raising it to two
  hours).

⛔ **What it must not do:** admit a lone `*` as a default allowlist. errand's
default is `["*"]` (`src/config/schema.ts:423`) and a lone `*` admits any host
(`broker.ts:34-36`), so out of the box it gates the port and restricts no host.
⭐ **An empty allowlist here refuses everything, and a configuration that names
no host does not start.**

### Prove

```bash
cargo test -p box-broker connect::
```

Passing: exit 0, with named tests for: a head split across two reads reassembled
whole; a head followed immediately by TLS bytes, where ⭐ **the assertion is
that the first byte after the blank line is still in the socket**; each of the
five malformed targets refused; `proxy-authorization` absent from the upstream
request; a nested route reaching the longer prefix; `*.example.com` not matching
`example.com`; an empty allowlist refusing; and a public bind refused.

⚠ **The head test is the one that matters and it is easy to write wrongly.**
Assert on the bytes left in the socket, not on the parsed head.

### Closing

Not closed.

---

## T-021: The injection gate

**Source:** `opensandbox-group/OpenSandbox` item 1759 (`8f9b616`); `TencentCloud/CubeSandbox` items 1410 and 1411 (`eaddce3`). Both in [`../docs/history/references/brokers/findings.md`](../docs/history/references/brokers/findings.md).
**Category:** broker
**Priority:** P1
**Effort:** L
**Status:** open
**Blocked by:** T-020, T-022, T-023

---

### Problem

⭐ **This is the entry the project exists for.** A credential is attached to an
outbound request, and every condition that is not checked is a way to send the
operator's credential somewhere they did not agree to.

### Premise

⭐ **MEASURED by two separate projects, each of which shipped the defect and
fixed it.** This is the strongest evidence in the index and it is not this
project's own.

**OpenSandbox 1759**, in its own words: the vault selected a binding from
`request.pretty_host`, the `Host` or HTTP/2 `:authority` header, **which is
unauthenticated input from the sandbox workload**. Nothing checked it against
the identity of the peer the request was delivered to, so a workload could open
a TLS session to any host its egress policy allowed, send `Host: <bound host>`,
and have that binding's credential injected into a request delivered to a peer
of its choosing.

The reasoning for the fix, which is the part to carry:

> The SNI is the right anchor because mitmproxy propagates it to the upstream
> handshake and verifies the upstream certificate against it, so unlike the
> `Host` header it is an identity the peer must prove.

**CubeSandbox 1410** is the same defect from the other side, and it shows where
the asymmetry leaks. Its HTTPS path was safe **by accident of a different
control**: upstream certificate verification with the original SNI meant an
attacker address could not present a valid certificate for another name. Its
plaintext path had no certificate, so the same code injected the credential
toward a sandbox-chosen address on a `Host` header alone. The reporter's driven
output:

```
HTTPS, Host==SNI (legit)              allow=true  injected=<the real credential>
HTTP, spoofed Host -> attacker IP     allow=true  injected=<the real credential>
```

⚠ **The reachability note is what makes it critical**: the destination is only
constrained if the network layer restricts it, and that layer's
`AllowInternetAccess` defaulted to true when unset. ⭐ **A broker's guarantee is
conditional on the egress policy underneath it.**

⛔ **And the module's own comments claimed the gate was covered.** It was
covered on one of two paths. That is the one-gated-door class,
[`../docs/methodology/reviews.md`](../docs/methodology/reviews.md) lens 1, in
the exact place this project puts a credential.

### Approach

One function, in `box-broker`, that every injection path goes through.
⛔ **One gate per action**
([`../docs/conventions/code.md`](../docs/conventions/code.md)): if there is a
second path that can attach a credential, this entry has failed.

The eight conditions are normative in
[`../docs/credential-brokering.md`](../docs/credential-brokering.md). This entry
implements 1 through 6 and 8; condition 7 is T-027.

⛔ **Conditions 5 and 6 are separate checks and both are required:**

- **5, the ClientHello SNI falls inside the binding's host scope.** Take the SNI
  from the handshake, normalize it (case, and a trailing dot), and test it
  against the binding's host patterns. ⚠ OpenSandbox's own note on the
  behaviour change: a client that coalesces an HTTP/2 connection across
  authorities no longer gets a credential for a binding scoped to the second
  name alone, and that is fail-closed by design;
- **6, the upstream certificate verifies against that same SNI.** ⛔ Not against
  the address, not against the `Host`.

⛔ **Condition 4, TLS, may be relaxed only by an explicit per-binding opt-in the
operator wrote.** Never a default, never an absence. That is exactly the shape
of CubeSandbox 1411.

⛔ **A flow with no SNI does not get a credential.** Plaintext carries none, and
this project does not have OpenSandbox's backward-compatibility reason to allow
a fallback.

⭐ **The refusal is a refusal, never a forward without the credential.** A
request that reaches an upstream unauthenticated is one the agent reads as an
upstream problem and retries differently.

### Prove

```bash
cargo test -p box-broker gate::
```

Passing: exit 0, and ⛔ **every one of these is a named test that must be seen
to refuse** ([`../docs/methodology/reviews.md`](../docs/methodology/reviews.md)
lens 2: plant the defect and read the exit code unpiped):

| test | asserts |
| --- | --- |
| spoofed `Host` on a foreign SNI | 403, and ⭐ **no header injected**, asserted on the forwarded request rather than on the response |
| matching SNI | injected |
| SNI normalization: case, trailing dot | injected |
| a wildcard binding covering an SNI that differs from `Host` | injected |
| ⛔ absent SNI | refused |
| plaintext without the per-binding opt-in | refused |
| plaintext with the opt-in | injected |
| an upstream certificate valid for a different name | refused |
| a streamed body that fails the gate mid-stream | ⛔ **terminated**, not 403 |
| ⭐ a request that reached the upstream without passing the gate | **the test fails**, even where the upstream answered 200 |

⚠ **That last row is the acceptance that matters.** A broker measured by "did
the agent succeed" passes a broker that is not injecting anything.

### Closing

Not closed.

---

## T-022: Session scope resolution, which never retargets

**Source:** `Infisical/agent-vault` `internal/brokercore/session.go` (`f0cdfac`).
**Category:** broker
**Priority:** P1
**Effort:** S
**Status:** open
**Blocked by:** T-020

---

### Problem

A connection arrives at the broker. Before anything else, it has to resolve to
exactly one session and therefore exactly one credential set. An ambiguous
answer that gets resolved by picking is a credential crossing a boundary.

### Premise

READ, not measured. agent-vault's is 150 lines at
`internal/brokercore/session.go`:

| what | lines |
| --- | --- |
| the two body caps, and why they differ | 10-18 |
| `ProxyScope`, the resolved identity | 20-39 |
| the `SessionResolver` interface | 41-46 |
| `ResolveForProxy` | 72-149 |

⭐ **The property to copy is at `:90-107`**: a scoped session whose hint does
not match its own vault is an **error**, not a redirect. The comment says
"never silently retarget". And at `:136-149`, an actor with more than one grant
and no hint is `ErrAgentVaultAmbiguous` rather than a choice.

### Approach

In `box-broker`:

- resolve from the connection itself, not from a header the session controls;
- one `Scope { session: SessionId, bindings: BindingSetId }` per connection,
  resolved once and carried through to injection;
- ⛔ **an ambiguous resolution is an error.** Never a default, never the first
  match, never the most recent;
- ⛔ **an expired or unknown session is refused before any TLS work**, which is
  what makes T-024's ordering possible;
- a per-address budget for authentication failures, so a wrong credential cannot
  be ground down. ⚠ agent-vault exempts loopback peers from that budget and says
  why (`internal/mitm/connect.go:30-44`): one agent legitimately opens dozens of
  connections at startup, and a local process can deny service by other means
  anyway. ⭐ **Read that reasoning before copying the exemption**: this project's
  peers are sessions in their own namespaces, which is a different case.

### Decision

**How the connection identifies itself.**

⭐ **Recommendation: the session's network namespace is the identity.** Each
session gets its own broker listener, on its own link-local address and port,
reachable only from that namespace. Nothing is presented and nothing can be
replayed, because there is no token to steal.

⚠ **The alternative** is one shared listener with a per-session token in
`Proxy-Authorization`, which is agent-vault's shape
(`internal/brokercore/proxyauth.go`). It costs one listener instead of many and
it reintroduces a secret in the session's environment. ⛔ **It loses on the
project's own premise**: the session is supposed to hold nothing worth stealing.

### Prove

```bash
cargo test -p box-broker scope::
```

Passing: exit 0, with named tests for: two sessions' listeners, where a
connection to one resolves only to that session; an ambiguous resolution
refused; an expired session refused **before** any certificate is minted
(asserted by counting signing operations, not by reading a log); and the
authentication-failure budget refusing after its limit.

### Closing

Not closed.

---

## T-023: The binding table, validated at admission and failing closed at use

**Source:** `Infisical/agent-vault` `internal/broker/broker.go` (`f0cdfac`); `opensandbox-group/OpenSandbox` items 1762 and 1776; `TencentCloud/CubeSandbox` items 1458 and 1459.
**Category:** broker
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-020

---

### Problem

An operator says "this credential may be attached to requests that look like
this". Getting the matching wrong in one direction breaks every request; getting
it wrong in the other attaches a credential to a request nobody intended.

### Premise

READ, not measured. Two defects, both shipped.

**CubeSandbox 1458, open.** `validate_policy` checked that `match` was a table
and never checked the types inside it, so a policy the admin API accepted with
200 caused an uncaught error on every subsequent request from that sandbox.
Cited at `CubeEgress/lua/policy.lua:82-84` and
`CubeEgress/lua/access_phase.lua:139,147`.

⭐ **The reporter's own severity judgement is the model**: it fails closed, no
traffic is allowed through, so it is availability and diagnosability rather than
a bypass. ⚠ The operator sees failures with **no indication that what they
installed was malformed**.

**OpenSandbox 1762 and 1776.** A binding that omits `match.methods` or
`match.paths`, or carries the host-wide `/*`, was normalized to all common
methods and every path. A guard exists and it shipped **opt-in**; the open item
asks for the default to flip:

> a control plane that projects higher-level API tool definitions into vault
> bindings can silently drop scoped endpoint metadata and create a host-wide
> credential injection binding. That is an easy projection mistake with a costly
> blast radius.

agent-vault's matching, for the shapes:

| what | file and lines |
| --- | --- |
| `Service`, `Substitution`, `Auth` | `internal/broker/broker.go:21-103` |
| `Auth::Validate` | `:134-212` |
| `MatchScore` and `Better` | `:509-530` |
| `MatchService`, most specific wins | `:547-587` |
| `matchHostPattern`, exact beats wildcard | `:588-606` |
| `matchPathGlob` | `:623-651` |

### Approach

In `box-broker`:

- a binding is `{ hosts, port, methods, paths, auth, substitutions }`;
- ⛔ **`methods` and `paths` are required, and a host-wide path is refused.**
  On by default, with no opt-out until somebody produces a case
  (`../docs/architecture.md` forbids a knob with no caller);
- ⛔ **every field's type is checked at admission**, with an error naming the
  field. Not at use;
- ⛔ **`rule_matches` fails closed** on anything unexpected, and the
  unexpected case is a typed error rather than a panic;
- most specific wins, deterministically: exact host beats wildcard **even where
  the wildcard has a longer path**, then port specificity, then path literal
  length (`broker.go:509-530`);
- ⛔ **exactly one binding must match.** Two matching bindings is a
  configuration error refused at admission, not a tie broken at request time.

### Prove

```bash
cargo test -p box-broker bindings::
```

Passing: exit 0, with named tests for: a binding omitting `methods` refused at
admission; a binding with a host-wide path refused; a wrong-typed field refused
at admission **naming the field**; two bindings that can both match refused at
admission; exact host beating a wildcard with a longer path; and an unexpected
shape at request time producing a typed refusal rather than a panic and rather
than a match.

### Closing

Not closed.

---

## T-024: The certificate authority, which authorizes before it signs

**Source:** `TencentCloud/CubeSandbox` items 1426, 1427 and 1455 (`eaddce3`); `Infisical/agent-vault` `internal/ca/soft.go` (`f0cdfac`).
**Category:** broker
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-022

---

### Problem

The broker terminates TLS inside each tunnel, so it mints a leaf per name. Doing
that before knowing who asked is a signing oracle.

### Premise

READ, not measured. **CubeSandbox 1426, closed.** The handshake callback called
the signer on a cache miss, generating a key and a CA signature; the
authorization gate ran in a later phase. A sandbox with **no policy at all**
still got a certificate minted for every distinct name it offered.

The reporter's driven output shows three handshakes, three distinct names, three
certificates minted, and the access phase denying every one.

Cited at `CubeEgress/nginx.conf:149-171`, `CubeEgress/lua/cert_signer.lua:97-158`,
and ⚠ `:163`, where the cache key is **the raw, unvalidated name**, with no cap
on distinct names and no rate limit on the signing path.

⭐ **Two costs, and the second is easy to miss**: processor time, and eviction of
every legitimately cached certificate, so real traffic pays a re-sign.

agent-vault's authority is at `internal/ca/soft.go`: `MintLeaf` at `:395-465`,
`validateSNI` at `:466-496`, `randomSerial` at `:497-504`, `writeAtomic` at
`:505`, and the leaf key generated per leaf at `:414`.

### Approach

In `box-broker`, and ⛔ **the ordering is the entry:**

1. the peer resolves to a session this broker serves (T-022), **before any TLS
   work**;
2. only then, validate the offered name, and mint or fetch a leaf for it;
3. a per-session budget on distinct names signed, and a rate limit on signing;
4. ⛔ **key the cache on the validated name**, never the raw one.

- serials from a cryptographic random source (CubeSandbox 1455 is the fix that
  says so);
- a fresh key per leaf;
- the root key held in the daemon process, written encrypted at rest, atomically
  ⛔ **and never granted to any session by any policy.** T-052's check asserts
  that no generated policy names the path it lives at.

### Prove

```bash
cargo test -p box-broker ca::
```

Passing: exit 0, with named tests for: an unresolved peer causing ⭐ **zero
signing operations**, counted rather than logged; a session exceeding its
distinct-name budget refused; the cache keyed such that two raw spellings of one
valid name hit one entry; two leaves for the same name carrying different
serials; and ⛔ a generated policy for any session naming no path under the root
key's directory.

### Closing

Not closed.

---

## T-025: Substitutions, with per-surface encoding

**Source:** `Infisical/agent-vault` `internal/brokercore/substitution.go` (`f0cdfac`); `agent-vault` item 209 for the limit.
**Category:** broker
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-021

---

### Problem

Not every upstream takes a credential in a header. Some want it in a path
segment, a query parameter or a body field. A broker that only injects headers
cannot serve them, and one that does raw string replacement everywhere creates
an injection surface.

### Premise

READ, not measured. agent-vault's is 155 lines at
`internal/brokercore/substitution.go`:

| what | lines |
| --- | --- |
| `ResolvedSubstitution`, with `In` marked as the security boundary | 14-20 |
| `HasBodySubstitutions` | 22-34 |
| `ApplySubstitutions`, and ⛔ **the contract that the caller must not forward on error** | 36-87 |
| the path case, operating on the wire-encoded path | 47-66 |
| the query case | 67-70 |
| the header case, with the CR and LF guard | 71-83 |
| `ApplyBodySubstitutions` | 89-138 |
| `bodyEncoder`, per media type | 140-149 |
| `jsonEscapeString` | 151-155 |

⭐ **The declared-surfaces model is the security boundary**, stated at
`internal/broker/broker.go:62-70`: a placeholder that may go in a header and not
in a body cannot be carried out in a body the agent chose.

### Approach

In `box-broker`, port the model and the encoding table from
[`../docs/credential-brokering.md`](../docs/credential-brokering.md).

⛔ **Three rules that are not negotiable:**

1. **the header surface refuses on CR or LF**, it does not strip
   (`substitution.go:75-77`);
2. **on any error the request is not forwarded**, because partial mutations may
   already be applied (`:36-39`);
3. **the path surface operates on the wire-encoded path**, so the escaped value
   lands exactly once and is not re-encoded on the way out (`:51-56`).

⚠ **Materializing a body has its own cap**, much smaller than the forwarded-body
cap, because body substitutions target API payloads rather than uploads
(`session.go:13-18`: 1 GiB against 64 MiB). CubeSandbox caps an injected secret
at 2048 bytes (item 1606).

⛔ **What it must not do: touch frames after an HTTP upgrade.** agent-vault item
209 is the exhibit and the limit is documented rather than fixed: a chat gateway
that authenticates in its first frame after the upgrade cannot be brokered this
way, and the placeholder reaches the wire. ⭐ **Refuse the binding at admission
rather than letting it fail on the wire**, which is better than what the
reference did.

### Prove

```bash
cargo test -p box-broker substitution::
```

Passing: exit 0, with named tests for each surface's encoding; a header value
carrying CR refused **and the request not forwarded**; a path value escaped
exactly once, asserted on the bytes on the wire; a multipart body left untouched;
a body over the materialization cap refused; and a binding declaring a surface
this project does not implement refused at admission.

### Closing

Not closed.

---

## T-026: The credential store, and failing closed two ways

**Source:** `opensandbox-group/OpenSandbox` `docs/guides/credential-vault.md` (`8f9b616`), "Runtime availability dependency".
**Category:** broker
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-021

---

### Problem

The real credentials live somewhere. If that somewhere is unreachable, the
broker has to do something, and the tempting answer is the wrong one.

### Premise

READ, not measured. OpenSandbox's guide states the invariant and the two shapes
it needs:

> A timeout, refused connection, unexpected EOF, `5xx` response, malformed
> snapshot, or invalid `ETag` clears the affected plaintext cache and fails the
> request closed before any upstream forwarding. Small requests whose bodies are
> safely buffered receive a local `503`. Requests that are already streaming,
> may cross the streaming threshold, or have an unknown body length are
> terminated instead, because mitmproxy cannot safely synthesize a local
> response after streaming has started.

⭐ **Two shapes, because one does not cover both cases**, and it names the
operational consequence honestly: the store is a hard availability dependency
for **all** intercepted traffic, not only traffic that ends up matching a
binding.

Its freshness mechanism is worth reading too: an opaque snapshot tag, a
conditional read, a `304` reusing the same immutable snapshot without
re-rendering secrets, a `200` carrying a validated replacement. ⭐ **Writes are
acknowledged only after the tag has changed**, so the next flow sees the new
credentials with no cache-delay window. And the tag is separate from any public
revision, so deleting and recreating cannot reuse plaintext from a deleted
snapshot.

### Approach

In `box-broker`:

- the store is in-process, written by the operator through a command, ⛔ **never
  reachable from a session by any route**;
- at rest, encrypted with a key the operator supplies. ⛔ The key is never in
  the configuration file
  ([`../docs/security/secrets.md`](../docs/security/secrets.md));
- ⛔ **fail closed in both shapes above.** Never forward without the credential;
- a snapshot tag, and a write that is acknowledged only once the tag has moved;
- ⛔ **one redaction helper, and it is the only way a secret-shaped value
  reaches output.** A redactor used in most places is a redactor that fails,
  because the call site that forgot is the one that leaks.

⚠ **An in-memory store does not survive a restart**, and OpenSandbox documents
exactly that consequence for pause and resume. Say so in
[`../docs/limits.md`](../docs/limits.md) rather than pretending otherwise.

### Prove

```bash
cargo test -p box-broker store::
```

Passing: exit 0, with named tests for: a buffered request getting `503` when the
store is unavailable; ⛔ a streaming request **terminated** rather than answered;
neither case reaching the upstream, asserted on a recording upstream; a write
not visible until the tag moves, and visible on the next flow after it does; and
⭐ a fuzz or property test asserting that no formatting path produces a
credential value, run against every type that can hold one.

### Closing

Not closed.

---

## T-027: Binding a destination address to an allowed name

**Source:** `TencentCloud/CubeSandbox` item 1697, **open upstream** (`eaddce3`); `opensandbox-group/OpenSandbox` item 1804 (`8f9b616`).
**Category:** broker
**Priority:** P2
**Effort:** L
**Status:** open
**Blocked by:** T-021

---

### Problem

⛔ **Condition 7 of the injection gate, and the one nobody has solved.**

A matching name does not establish that the address the bytes go to belongs to
that name. [`../docs/credential-brokering.md`](../docs/credential-brokering.md)
states the shape; the measurement is below.

### Premise

⭐ **MEASURED, and open at the reference that measured it.**

CubeSandbox 1697: with an allow rule for a name resolving only to one address,
and a second backend at another, the reporter measured **200 for all four HTTP
and HTTPS cases** using `--resolve` and hosts entries. For the HTTPS case the
unrelated backend deliberately presented a certificate valid for the allowed
name and trusted by the test proxy, which demonstrates the gap even where
upstream verification succeeds.

⚠ **Read the item's own design section before writing any code.** It lists what
is unsettled, and this project inherits all of it:

- is proxy-side DNS membership the desired contract? Content delivery networks,
  split DNS and rotation give the session and the broker different valid address
  sets and cause false denials;
- should the binding use addresses the network layer already observed? That needs
  provenance, expiration and defined access;
- ⛔ **if the proxy selects a new resolved address, that destination must pass
  address-level authorization again**; silently replacing it is the bypass;
- should an SNI and `Host` disagreement be refused for **every** host-constrained
  rule, including those that inject nothing?

**What shipped closest** is OpenSandbox 1804, and its invariants are all
fail-closed: a background lookup never adds an unobserved address, a policy
replacement clears records and fences in-flight results, a DNS failure does not
renew, a negative or rotated answer stops renewal. Bounded to 128 domains, 64
addresses per domain, four workers, five seconds per lookup, a 20-second batch
deadline.

### Approach

⛔ **Do not close this as solved and do not close it as impossible.**

Start with the narrowest thing that is defensible and say so in
[`../docs/limits.md`](../docs/limits.md):

1. the broker resolves the binding's names itself, from a resolver the operator
   named, and holds the answers with their time to live;
2. a connection whose destination is not in that set is refused;
3. ⛔ **a refusal names the address and the name**, so an operator hitting a
   content-delivery-network false denial can see why in one line rather than
   bisecting;
4. every clause from OpenSandbox 1804 above, ported verbatim as invariants.

⚠ **And measure the false-denial rate before defaulting it on.** T-064's battery
is the instrument: point it at three real upstreams behind content delivery
networks, over an hour, and count refusals that were not attacks. ⭐ **A control
that fires on legitimate traffic is a control operators disable.**

### Decision

**On by default, or off by default.**

⭐ **Recommendation: off by default until the false-denial rate is measured, and
the measurement is part of this entry.** Turning it on before that trades a
known attack for an unknown outage rate, and an operator who disables it after
one outage has a worse posture than one who enabled it deliberately.

⚠ **This recommendation is the weakest thing in this entry.** It optimises for
adoption over strictness, and a reviewer who disagrees has a real case.

### Prove

```bash
cargo test -p box-broker destination:: && cargo xtask battery dns-binding --hours 1 --expect docs/history/references/expected-dns.json
```

Passing: `cargo test` exits 0 with named tests for each invariant above,
including ⛔ a background lookup that does not add an unobserved address and a
policy replacement that fences a result already in flight. The battery reports
the false-denial count against three real upstreams, and this entry's closing
records the number, the hosts, the date and the resolver.

⚠ `cargo xtask battery` does not exist until T-064.

### Closing

Not closed.

---

## T-028: Execution-scoped credentials

**Source:** `kubernetes-sigs/agent-sandbox` item 1580, **open** (`4b63868`); `Infisical/agent-vault` item 255, **open** (`f0cdfac`).
**Category:** broker
**Priority:** P2
**Effort:** L
**Status:** open
**Blocked by:** T-021, T-026

---

### Problem

⭐ **The residual problem, and the honest name for it.**

A session holds its bindings for its whole life. A task that needed repository
read in its first turn can still reach repository write in its last one, and an
agent prompt-injected in between has whatever the session was granted at the
start. Brokering moved the credential out of the sandbox; it did not narrow what
the sandbox may ask the broker to do.

### Premise

⭐ **READ, and open at both references, which is evidence of difficulty rather
than of oversight.**

agent-sandbox 1580 states it exactly:

> This is an authorization-execution lifetime mismatch: the authorization can
> outlive the execution for which it was required.

Its worked example is two executions in one reused sandbox, one needing object
storage and one needing a repository token, where provisioning at creation time
makes the effective grant the **union**, so code in the second can misuse the
first's credential.

agent-vault 255 is the other half: a GitHub App credential type that signs a
short-lived token, exchanges it for an installation access token, injects that,
refreshes before expiry, and never lets the agent read either the private key or
the derived token. ⛔ **It is a feature request, not an implementation.**

⚠ **So this entry is writing something neither reference has.** Say that in the
plan rather than implying a port.

### Approach

Three routes, and the entry picks one after reading all three. ⛔ **None is
free, and the plan says what each costs.**

| route | what it buys | what it costs |
| --- | --- | --- |
| **narrow the binding set per phase** | the strongest thing available without upstream support | somebody has to declare the phases, and a wrong declaration blocks legitimate work |
| **an out-of-band approval for widening operations** | the operator sees what is about to happen | the operator is in the loop, which is both the point and the cost |
| **mint short-lived credentials per execution at the broker** | ⭐ the credential expires whether or not anybody revoked it | needs the upstream to support minting. GitHub App installation tokens do; most API keys do not. |

⭐ **Recommendation: build the first, design for the third.** A binding set that
can be replaced mid-session is the mechanism both need, and T-026's snapshot tag
is already the machinery for replacing one without a cache window.

⛔ **What it must not do:** claim that any of this closes exfiltration.
[`../docs/limits.md`](../docs/limits.md) says what does and does not, and this
entry does not get to soften it.

### Prove

```bash
cargo test -p box-broker execution_scope::
```

Passing: exit 0, with named tests for: a binding set replaced mid-session taking
effect on the next flow and not the one in flight; a request matching the
previous set refused after replacement; and ⭐ **a test that demonstrates the
limit**, where a session with a legitimate write binding sends a private payload
to an allowed host and the broker permits it, asserting that this project does
not claim otherwise.

⚠ **That last test is unusual and it is deliberate.** A test that proves a
documented limit is real is what stops the limit being quietly dropped from the
documentation.

### Closing

Not closed.

---

## T-029: Detecting a session that is not using the broker

**Source:** `Infisical/agent-vault` item 194, closed (`f0cdfac`).
**Category:** broker
**Priority:** P2
**Effort:** S
**Status:** open
**Blocked by:** T-020

---

### Problem

⛔ **The operationally dangerous limit.** A client whose transport refuses the
proxy never reaches the broker, may fall back to a path that works, and produces
correct output. A deployment measuring "did the agent succeed" passes it.

### Premise

READ, not measured. agent-vault 194: the Codex command-line tool's WebSocket
transport refused the proxy URL scheme for `wss://api.openai.com/v1/responses`,
logged an error, fell back to HTTP, and **returned the expected output**. The
only evidence was a line in standard error:

```
ERROR codex_api::endpoint::responses_websocket: failed to connect to websocket:
URL error: Proxy URL scheme not supported, url: wss://api.openai.com/v1/responses
```

⭐ **In this project's design the fallback would have failed too**, because the
session's namespace has no route except the broker. ⚠ **That is the argument for
the design and not a reason to skip this entry**: the failure would then be an
opaque connection error rather than a diagnosable one, which is worse for the
operator even though it is safer.

### Approach

In `box-broker` and `box-session`:

- ⛔ **count connection attempts that reached the namespace's route and were not
  CONNECT**, and report them per session. A session generating them is a session
  whose client is not proxy-aware;
- report, in the session's own diagnostics, the number of requests that passed
  the injection gate against the number of tunnels opened. ⭐ **A session that
  opened tunnels and injected nothing is the shape of this defect**;
- a startup note naming the environment variables a well-behaved client reads,
  and ⚠ the fact that naming them is not enforcement: the namespace is.

⛔ **What it must not do:** try to fix the client. That is not this project's,
and the reference's own thread concluded the failure was specific to one
transport rejecting the proxy URL before anything else saw it.

### Prove

```bash
cargo test -p box-broker bypass::
```

Passing: exit 0, with named tests for: a non-CONNECT connection to the broker
counted and refused; a session with tunnels and zero injections reported as
such; and the counters present in the session diagnostics output.

### Closing

Not closed.
