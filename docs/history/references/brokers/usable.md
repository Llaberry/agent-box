# credential brokers: usable

The rules and the lines, for the session building the broker.
[`findings.md`](findings.md) is the reasoning.

Corpus paths are relative to the `references` branch worktree, at the commits
in [`../pins.md`](../pins.md).

---

## ⭐ The injection gate, as a list a check can hold

⛔ **A credential is attached only when every one of these holds.** Any of them
failing is a refusal, not a warning, and not a forward without the credential.

| # | condition | the exhibit that proves it is needed |
| --- | --- | --- |
| 1 | the caller resolved to exactly one scope: one actor, one credential set | agent-vault `internal/brokercore/session.go:74-149` |
| 2 | the request matched exactly one binding | OpenSandbox `docs/guides/credential-vault.md`, "if exactly one binding matches" |
| 3 | the binding names an explicit method set and an explicit path scope, neither of them host-wide | OpenSandbox 1762, 1776 |
| 4 | the connection is TLS | CubeSandbox 1410 |
| 5 | ⭐ the ClientHello SNI falls inside the binding's host scope | OpenSandbox 1759 |
| 6 | the upstream certificate verifies against that same SNI | CubeSandbox 1410, where this is what made the HTTPS path safe |
| 7 | the original destination address is one the name legitimately resolves to | CubeSandbox 1697 ⚠ and see the open question below |
| 8 | the credential store answered this request, freshly | OpenSandbox `docs/guides/credential-vault.md`, "Runtime availability dependency" |

⚠ **Condition 4 may be relaxed only by an explicit per-binding opt-in that the
operator wrote**, never by a default and never by absence. That is the exact
shape of CubeSandbox 1411.

⛔ **Conditions 5 and 6 are not the same check.** 5 says the session is
authenticated as a name the binding trusts. 6 says the peer proved it. A broker
with 6 and not 5 is CubeSandbox before 1411; a broker with 5 and not 6 trusts a
name nobody proved.

### The fail-closed shapes, and why two are needed

From OpenSandbox `docs/guides/credential-vault.md`, "Runtime availability
dependency". When the credential store is unreachable, or the snapshot is
malformed, or the tag is invalid:

| the request is | the answer |
| --- | --- |
| small, and its body was safely buffered | a local 503 |
| already streaming, or over the streaming threshold, or of unknown body length | ⛔ **terminated**, because a local response cannot be synthesized after streaming has started |

⭐ **Neither is "forward it without the credential".** A request that reaches
the upstream unauthenticated is a request whose failure the agent can read and
retry differently.

---

## ⛔ Authorize before you sign

CubeSandbox 1426. The TLS handshake callback is not the place to mint a leaf on
a cache miss unless the caller is already known to be authorized.

The ordering to implement:

1. the peer is a session this broker serves, established from the connection
   itself, before any TLS work;
2. only then, mint or fetch a leaf for the offered SNI;
3. cap the number of distinct SNIs one session can cause to be signed, and rate
   limit the signing path;
4. ⚠ **key the cache on a validated SNI**, never the raw one. CubeSandbox kept
   the raw value at `cert_signer.lua:163` and the cache was the amplifier.

⭐ **The second cost is the one to design against:** an unauthorized flood
evicts real traffic's certificates, so legitimate requests pay a re-sign. A
per-session signing budget bounds both.

Leaf serials come from a cryptographic random source: CubeSandbox 1455 is the
fix that says so, and agent-vault does it at `internal/ca/soft.go:497-504`.

---

## The substitution model, and the encoding table

agent-vault `internal/brokercore/substitution.go`. ⭐ **Take this whole shape.**

A substitution declares a placeholder, a value, and **the surfaces it may be
written into**. `internal/broker/broker.go:62-70` names the scoping as the
security boundary, and it is: a placeholder that may go in a header and not in
a body cannot be carried out in a body the agent chose.

```
surfaces: path | query | header | body | websocket
```

Per-surface encoding, at `substitution.go`:

| surface | rule | lines |
| --- | --- | --- |
| path | escape, then operate on the **wire-encoded** path, so the value lands exactly once and is not re-encoded on the way out | 47-66 |
| query | query-escape | 67-70 |
| header | raw, and ⛔ **refuse the whole request** if the value holds CR or LF | 71-83 |
| body, `application/x-www-form-urlencoded` | query-escape | 141-143 |
| body, `application/json` | JSON string escape | 144-146, 151-155 |
| body, `multipart/*` | ⚠ skipped entirely | 120-124 |
| body, anything else | raw | 147-148 |

⛔ **On any error the caller must not forward.** `substitution.go:36-39`:
partial mutations may already have been applied to the request.

⚠ **Buffering a body to substitute into it is a memory decision with its own
cap.** agent-vault: 1 GiB for a forwarded body, 64 MiB for one materialized for
a body substitution (`internal/brokercore/session.go:13-18`). CubeSandbox caps
an injected secret at 2048 bytes (item 1606).

---

## The hard limits to write down rather than work around

⭐ **Each of these is a documented limit in
[`../../../limits.md`](../../../limits.md), not a defect to fix.**

| limit | the exhibit |
| --- | --- |
| a credential carried inside a protocol message after an HTTP upgrade is not reached by metadata substitution | agent-vault 209: Discord Gateway sends the token in the Identify frame and the proxy raw-piped it |
| a client whose transport refuses the proxy URL never reaches the broker, and may fall back to a path that works | agent-vault 194: Codex's WebSocket transport rejected `wss://` through an HTTPS proxy, logged an error, and succeeded over HTTP |
| a cert-pinned client fails the interception rather than being brokered | stated by all three references' architecture; not separately measured here |
| the broker's guarantee is conditional on the egress policy underneath it | CubeSandbox 1410's reachability note: the destination is unconstrained where the network layer defaults open |
| a transparent service mesh in the same network namespace cannot coexist with a transparent broker | OpenSandbox `docs/guides/credential-vault.md`, "Service Mesh Compatibility" |
| an in-memory credential store does not survive a pause and resume | the same page, "Persistence Across Pause and Resume" |

⛔ **The second row is the operationally dangerous one.** A deployment that
measures "did the agent succeed" would have passed it. ⭐ **So the acceptance
for the broker asserts that the credential was injected, not that the request
worked**, and a request that reached an upstream without passing the gate is a
failure even when the upstream answered 200.

---

## The timeout that cut off real work

CubeSandbox 1650, closed: a 60-second read timeout was baked into the image and
cut off model requests that legitimately exceeded it; 1655 raised it to two
hours.

agent-vault's own numbers, `internal/mitm/connect.go:139-154` and
`internal/mitm/proxy.go:86-90`:

| bound | value | what it protects |
| --- | --- | --- |
| read header | 10s | slow-loris |
| read | 60s | the request side |
| write | 30 min | ⭐ long streaming transfers: a clone, a server-sent event stream |
| idle | 2 min | keep-alives between requests |
| upstream response header | 5 min | a stalled upstream |
| TLS handshake | 10s | |

⚠ **A model turn and a git clone are both long streaming transfers.** A read
timeout sized for an API call breaks them both, and the failure looks like the
model provider being flaky.

---

## ⚠ The open question this project inherits

**How a destination address is bound to an allowed name.** CubeSandbox 1697 is
open and its own design section lists what is unsettled:

- proxy-side DNS membership gives false denials under CDNs, split DNS and
  rotation, because the guest and the proxy can hold different valid address
  sets;
- using addresses the network layer already observed needs provenance,
  expiration and a defined way for the broker to read them;
- ⛔ **if the proxy resolves a new address itself, that new destination has to
  pass the address-level authorization again**, or the substitution is the
  bypass;
- whether an SNI and `Host` disagreement should be refused for every
  host-constrained rule, or only for those that inject.

⭐ **Write the constraint and the evidence, never the conclusion alone.** The
entry that owns this says what is undecided and picks a defensible default
with the alternatives recorded. It does not record the question as closed,
and it does not record it as impossible.

OpenSandbox 1804 is the nearest thing to an answer that shipped, and its
invariants are the ones to copy: background lookups never add an unobserved
address, a policy replacement clears the records and fences results already in
flight, a DNS failure does not renew, and a negative or rotated answer stops
renewal. Every clause fails closed.

---

## What not to take

| | why |
| --- | --- |
| mitmproxy, OpenResty, nginx, Lua | this project is one binary. The mechanisms transfer; the runtime does not. |
| an admin or management API bound to every interface | CubeSandbox `docs/guide/network-hardening.md` lists seven services defaulting to `0.0.0.0`, most with no authentication and no TLS. ⭐ Bind loopback by default and refuse a public bind rather than warning about it. |
| a control plane that stores the operator's tokens off their machine | this project is self-hosted; that is a requirement, not a preference |
| the multi-tenant vault, grant and role model in full | agent-vault's is larger than this project needs. ⚠ Take `ProxyScope` and `ResolveForProxy`'s refusal to retarget (`internal/brokercore/session.go:20-46`, `:90-107`); leave the rest until there is a second tenant. |
