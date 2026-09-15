# harnesses: usable

The interface to drive, and the rules that come with it.
[`findings.md`](findings.md) is the reasoning.

⛔ **Re-read the published documentation before writing an adapter.** These
projects move, this page is a map, and a flag name here can be stale.

---

## The reading list

| to build | read |
| --- | --- |
| the harness adapter | `packages/coding-agent/docs/rpc.md`, all of it. It is 1,618 lines and the protocol is the whole contract. |
| the credential shape | `packages/coding-agent/docs/providers.md`, and `custom-provider.md` for the `baseUrl` declaration |
| ⛔ what a repository can make the harness load | `packages/coding-agent/docs/security.md`, "Project Trust" |
| where this project fits | `packages/coding-agent/docs/containerization.md`, the pattern table |
| the web surface's authorization rules | `pingdotgg/t3code` `docs/internals/environment-auth.md`, 79 lines, all of it |

---

## ⭐ The three settings a session pins, and why each

⛔ **Inheriting any of these is inheriting a decision somebody else made for a
different threat model.**

### 1. Project trust is `never`

A repository can ship `.pi/settings.json`, `.pi/extensions`, `.pi/skills`,
`.pi/prompts`, `.pi/SYSTEM.md`, `.pi/APPEND_SYSTEM.md`, or `.agents/skills` in
any ancestor directory. **Extensions are TypeScript modules that run with the
harness process's permissions**, which in this project's terms is code running
as the agent, chosen by whoever wrote the repository.

⚠ Headless modes do not prompt. The default is to ignore those resources, and
one setting makes them load. ⛔ **This project sets it explicitly rather than
relying on the default**, because a default that can be changed by an operator
looking for fewer prompts is not a boundary. T-046.

⭐ **It is an input guard, not a sandbox**, and the harness says so itself. It
stops a repository silently changing the agent's configuration. It does nothing
about untrusted model output, and nothing about prompt injection from a file the
agent reads on purpose.

### 2. The agent home is inside the session's own state

`~/.pi/agent` holds `auth.json`, the session store and the cached provider
catalogue. The harness's own containerization page warns against mounting the
host's copy. ⛔ **A session gets a fresh one under `/state/home`, never the
operator's**, which is what errand already does.

### 3. The provider is declared, with a `baseUrl` pointing at the broker

A custom provider declaration carries a `baseUrl`, so pointing the harness at
the broker needs **no patch to the harness**. ⭐ That is what keeps this project
a wrapper rather than a fork, and
[`../../../methodology/vendoring.md`](../../../methodology/vendoring.md) is why
that matters.

---

## ⛔ The framing rule, and it is not a detail

The protocol is JSONL over stdin and stdout, and the harness's own
documentation names the trap:

> Split records on `\n` only. Accept optional `\r\n` input by stripping a
> trailing `\r`. Do not use generic line readers that treat Unicode separators
> as newlines.

⭐ **`U+2028` and `U+2029` are valid inside JSON strings**, and a reader that
splits on them corrupts the stream on input a model can produce, in a way that
looks like a protocol error from the other side.

In Rust: read bytes, split on `0x0A`, strip one trailing `0x0D`. ⛔ **Do not
reach for a convenience lines iterator without reading what it treats as a line
break**, and make the test a record whose JSON string value contains `U+2028`.

⚠ **Bound the record length.** A record with no newline is a record that grows
until something dies, and the thing producing it is untrusted.

---

## The provider-sandbox shape, for a vendor CLI with subscription auth

⭐ **This is the pattern for "use a provider through its own harness rather than
its API", and it is the one with the worst credential to protect.**

A vendor CLI authenticated with a subscription holds a long-lived, refreshable
token tied to a person's account, on disk, in its own configuration directory.
It is worth more than an API key and it is harder to rotate.

```
  agent sandbox                provider sandbox              the vendor
  -------------                ----------------              ----------
  the harness                  the vendor CLI
    points at ---------------> a shim that speaks
    the shim's baseUrl         the model API                 
                                     |                        
                                     +--- the broker --------> the real endpoint
                                          injects the real
                                          subscription token
```

⛔ **Three rules, and the first is the one that is tempting to skip:**

1. **The vendor CLI is in its own sandbox**, not in the agent's. It is a
   different trust level from the agent and from the daemon, and putting it in
   the agent's sandbox hands the agent whatever it holds.
2. **Its egress is brokered to that vendor's hosts and nothing else.** Every
   condition in
   [`../../../credential-brokering.md`](../../../credential-brokering.md)
   applies, and the SNI condition matters most here because a subscription
   endpoint and a general API endpoint are often the same host.
3. ⛔ **The refresh flow is brokered too.** A subscription credential is not one
   value: it is an access token, a refresh token and a token endpoint. A broker
   that injects the access token and lets the CLI hold the refresh token has
   brokered the cheap half.

⚠ **And the honest limit**: a vendor CLI that pins its certificates, or that
authenticates inside a protocol message rather than a header, is not brokerable
this way. [`../../../limits.md`](../../../limits.md) carries both shapes
already. T-045 records which vendors were tried and which worked.

---

## The web surface's authorization rules, taken from t3code

⭐ **Seven rules, each one line, each closing a specific escalation.** Take them
as written.

1. **A derived credential narrows a grant and never widens it.** A session token
   minted from a login carries a subset of what the login had.
2. ⛔ **Nothing the client says about itself decides what it may do.** Labels and
   device metadata have no authorization role.
3. ⭐ **A list endpoint returns metadata, never a recoverable secret.** Otherwise
   read access to a list is a way to acquire somebody else's authority.
4. ⛔ **A failed strong check fails.** It never falls back to a weaker one.
5. **A long-lived token stays out of a socket URL.** Mint a short-lived ticket
   over authenticated HTTP instead, because a URL reaches logs and referrers.
6. ⭐ **A successful handshake grants no authority.** Every call declares the
   scope it needs, and the scope is checked per call.
7. **A development credential is ignored outside development**, and a rejected
   normal credential never falls back to it.

⚠ **Rule 6 is the one that gets skipped**, because authenticating the connection
feels like the hard part. It is not: the hard part is that every message on it
still has to be authorized.

---

## What not to take

| | why |
| --- | --- |
| a terminal multiplexer | `herdr` owns agent terminals across disconnects and machines, which is a real product and not this boundary. ⚠ If sessions ever need to survive a daemon restart, read it again before building one. |
| a control plane, a relay, mobile clients | `t3code`'s shape. This project has a Discord channel and a web page. |
| ⛔ a patch or a fork of any of the three | we drive them, and at most wrap them. [`../../../methodology/vendoring.md`](../../../methodology/vendoring.md) settles it, and upstreaming is not a topic. |
| the harness's own permission model | ⛔ there is not one, deliberately, and its own documentation says a partial in-process sandbox is worse than none because it reads as a boundary while depending on the host shell, filesystem and package managers |
