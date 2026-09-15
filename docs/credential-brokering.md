# credential-brokering.md

How a session reaches an upstream that needs a credential, without ever holding
one.

[`architecture.md`](architecture.md) is the technical reference and wins any
conflict with this page. ⚠ **Nothing here is on disk yet**; every rule is a
decision an entry implements, and the entries are named per section.

⭐ **The evidence for nearly every rule below is a defect somebody else shipped
and then fixed.** [`history/references/brokers/`](history/references/brokers/)
carries each one with its reporter, its reproduction and its fix.

---

## What the agent holds

| | the agent | the broker |
| --- | --- | --- |
| a model provider key | a per-run random nonce | the real value |
| a repository token | a per-run random nonce | the real value |
| any other credential | a per-run random nonce | the real value |

The nonce is 32 bytes from the system generator, minted per credential per
daemon run, and is worth **only what the broker will do with it**. The broker
is reachable only from the session's own network namespace.

⛔ **A nonce is a secret.** It is compared in constant time, it is never logged,
and it is removed when its session ends. T-020.

⚠ **This does not make a session's environment safe to print.** A nonce read out
of `/proc/self/environ` is still a capability for as long as that session runs.
The point is that it is worth nothing anywhere else and nothing after teardown.

---

## ⛔ The injection gate

**A credential is attached only when every one of these holds.** Any of them
failing is a refusal. ⛔ **Not a warning. Not a forward without the
credential.**

| # | condition | entry | the exhibit |
| --- | --- | --- | --- |
| 1 | the caller resolved to exactly one scope: one session, one credential set | T-022 | agent-vault's `ResolveForProxy` refuses an ambiguous grant rather than picking |
| 2 | the request matched exactly one binding | T-023 | OpenSandbox: "if exactly one binding matches" |
| 3 | the binding names an explicit method set and an explicit path scope, and neither is host-wide | T-023 | OpenSandbox 1762 and 1776: an omitted scope was normalized to everything, and the guard shipped opt-in |
| 4 | the connection is TLS | T-021 | ⭐ CubeSandbox 1410: the plaintext path injected the operator's credential toward an attacker-chosen address on the strength of a `Host` header |
| 5 | ⭐ the ClientHello SNI falls inside the binding's host scope | T-021 | OpenSandbox 1759 |
| 6 | the upstream certificate verifies against that same SNI | T-021 | CubeSandbox 1410: this is what made the HTTPS path safe while the plaintext path was not |
| 7 | the original destination address is one the name legitimately resolves to | T-027 | ⚠ CubeSandbox 1697, **open upstream**. See below. |
| 8 | the credential store answered this request, freshly | T-026 | OpenSandbox's availability note |

### ⭐ Why condition 5 exists, in one paragraph

**The `Host` header and the HTTP/2 `:authority` are input from the sandbox. The
TLS SNI is an identity the peer must prove.** A broker that selects a binding
from `Host` alone lets a session open a connection to any host its egress policy
allows, send `Host: <the bound host>`, and have that binding's credential
injected into a request delivered to a peer of its choosing. Two separate
projects shipped that and fixed it.

⛔ **Conditions 5 and 6 are not the same check.** 5 says the session
authenticated as a name the binding trusts. 6 says the peer proved it. A broker
with 6 and not 5 is the exhibit above. A broker with 5 and not 6 trusts a name
nobody proved.

⚠ **Condition 4 may be relaxed only by an explicit per-binding opt-in the
operator wrote**, never by a default and never by absence.

---

## ⛔ Fail closed, and there are two shapes

When the credential store is unreachable, or the snapshot is malformed, or the
gate cannot be evaluated:

| the request is | the answer |
| --- | --- |
| small, and its body was safely buffered | a local `503` |
| already streaming, over the streaming threshold, or of unknown body length | ⛔ **terminated**, because a local response cannot be synthesized once streaming has started |

⛔ **Neither is "forward it without the credential".** A request that reaches an
upstream unauthenticated is a failure the agent will read as an upstream
problem and retry differently, which is worse than a clear refusal.

⭐ **So the acceptance asserts the credential was injected, not that the request
worked.** A request that reached an upstream without passing the gate is a
failure even when the upstream answered 200. T-021's prove says so.

---

## The certificate authority

The broker terminates TLS inside each CONNECT tunnel, so it mints a leaf per
name.

⛔ **Authorize before you sign.** The handshake callback is not the place to
mint a leaf for a caller who has not been established as a session this broker
serves. Ordering, from T-024:

1. the peer is a session this broker serves, from the connection itself, before
   any TLS work;
2. only then, mint or fetch a leaf for the offered SNI;
3. cap the distinct SNIs one session can cause to be signed, and rate limit the
   signing path;
4. ⚠ **key the cache on a validated SNI**, never the raw value.

⭐ **The second cost is the one that is easy to miss**: an unauthorized flood
evicts legitimate traffic's certificates, so real requests pay a re-sign. A
per-session signing budget bounds both.

Serials come from a cryptographic random source. The root's private key lives in
the daemon process and is written to disk encrypted at rest; ⛔ **it is never
reachable from a session by any path**, which is a property of the session
having no filesystem grant to it and no route to anything that holds it.

---

## The substitution model

⭐ **More general than header injection, and it is what handles an upstream that
wants a credential somewhere other than a header.**

A binding declares a placeholder, the credential it stands for, and ⛔ **the
surfaces it may be written into**. The surface list is the security boundary: a
placeholder that may go in a header and not in a body cannot be carried out in a
body the agent chose.

```
surfaces: path | query | header | body
```

Encoding is per surface, and per content type in a body. T-025:

| surface | rule |
| --- | --- |
| path | escape for a path, and operate on the **wire-encoded** path so the value lands exactly once and is not re-encoded on the way out |
| query | escape for a query |
| header | raw, and ⛔ **refuse the whole request** if the value holds CR or LF |
| body, form-encoded | escape for a query |
| body, JSON | escape as a JSON string |
| body, multipart | ⚠ skipped |
| body, anything else | raw |

⛔ **On any substitution error the request is not forwarded.** Partial mutations
may already have been applied.

⚠ **Buffering a body to substitute into it is a memory decision.** The cap for a
materialized body is separate from, and much smaller than, the cap for a
forwarded one, because body substitutions target API payloads rather than file
uploads.

---

## Timeouts

⚠ **A model turn and a repository clone are both long streaming transfers.** A
read timeout sized for an API call breaks them both, and the failure looks like
the upstream being flaky. One reference shipped a 60-second read timeout baked
into its image and it cut off real model requests.

The starting values, from T-020, all adjustable:

| bound | value | protects against |
| --- | --- | --- |
| read header | 10s | a slow-loris |
| read | 60s | the request side |
| write | 30 min | ⭐ long streaming transfers |
| idle | 2 min | keep-alives between requests |
| upstream response header | 5 min | a stalled upstream |
| TLS handshake | 10s | |

---

## The CONNECT parser, and the three ways to get it wrong

T-020. Each of these produces a corrupted connection rather than an error, so
none is visible in review.

1. ⛔ **Read the head one byte at a time, up to and including the blank line.**
   Reading past it strips bytes the client expects to carry its TLS. Reading
   only the request line leaves the remaining header bytes in the socket, where
   they are piped to the upstream **ahead of the ClientHello**. Cap the head;
   accept `CRLFCRLF` and a bare `LFLF`.
2. ⛔ **Refuse a target you had to guess at.** A missing port, a non-numeric
   port, a port out of range, a bracketed authority, a slash in the host: every
   one is a refusal. ⭐ A target the broker had to guess at is one it cannot
   claim to have checked.
3. ⛔ **Strip hop-by-hop headers in both directions**: `connection`,
   `keep-alive`, `proxy-authenticate`, `proxy-authorization`, `te`, `trailer`,
   `transfer-encoding`, `upgrade`, `host`. ⚠ Forwarding `proxy-authorization`
   upstream sends the caller's own proxy credential to the origin.

And two matching rules:

- ⭐ **longest prefix wins**, or a route nested under another's path is silently
  unreachable;
- **a wildcard does not match its own apex**: `*.example.com` matches any
  subdomain and not bare `example.com`. Comparison is case-folded.

---

## ⚠ The open question this project inherits

**How a destination address is bound to an allowed name.** Condition 7 above.

The problem: a matching name does not establish that the address the bytes go to
belongs to that name. A hosts entry or a resolver override preserves the allowed
name and changes the destination. One reference measured all four HTTP and HTTPS
cases returning 200 against that shape.

⛔ **There is no settled answer in any reference read for this project.** What
is known:

| route | what it costs |
| --- | --- |
| proxy-side DNS membership | false denials under content delivery networks, split DNS and rotation, because the session and the broker can hold different valid address sets |
| addresses the network layer already observed | needs provenance, expiry, and a defined way for the broker to read them |
| the broker resolving the address itself | ⛔ **the new destination has to pass address-level authorization again**, or the resolution is the bypass |

⭐ **The nearest thing that shipped is a revalidation design, and its invariants
are all fail-closed**: a background lookup never adds an unobserved address, a
policy replacement clears the records and fences results already in flight, a
DNS failure does not renew, and a negative or rotated answer stops renewal.

T-027 owns this. ⛔ **It is recorded as undecided with a defensible default, not
as solved and not as impossible.**

---

## ⚠ What brokering does not do

⛔ **Read [`limits.md`](limits.md).** The three that matter most here:

1. ⭐ **It closes credential theft, not exfiltration.**
   [`limits.md`](limits.md) states it in full and says which mitigations are
   real. Every reference read for this project has it open.
2. **A credential carried inside a protocol message after an HTTP upgrade is
   not reached by metadata substitution.** One reference's users hit this with a
   chat gateway that authenticates in its first frame.
3. ⛔ **A client whose transport refuses the proxy never reaches the broker, and
   may quietly fall back to one that works.** One reference's report shows a
   tool logging a proxy error, falling back, succeeding, and producing correct
   output. ⭐ **A deployment that measures "did the agent succeed" passes that.**
   T-029 owns the detection.
