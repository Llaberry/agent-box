# Credential brokering

How egi keeps real credentials out of every sandbox. Read with
[`../SECURITY.md`](../SECURITY.md), which states the threat model, and
[`architecture.md`](architecture.md), which states the shape.

## The rule

⛔ **The agent never holds a real credential at any point in its lifetime.**

Not in the environment, not on the command line, not on the filesystem, not
in logs. Env var redaction is impossible: `execve` copies environ onto the
process stack, `getenv` is a pointer walk, and `/proc/self/environ` defeats
every shim. Output masking is not a boundary: trivial encoding defeats it.
Brokering at the boundary is the only pattern that holds.

## How it works

1. The operator stores credentials in the daemon's secret store. Sessions
   receive dummy values (nonces) worth only what the broker will do with
   them, reachable only from the session's own namespace.
2. The session speaks to the broker as an ordinary HTTP CONNECT proxy, so
   `HTTPS_PROXY` is all a well-behaved client needs.
3. The broker gates on the connection's real destination (the host in the
   CONNECT line, the TLS SNI) against the allowlist and refuses everything
   else. It never matches on the workload-supplied Host header alone:
   OpenSandbox issue 1758 and CubeSandbox issue 1697 record the
   credential-steering class this refuses. The tunnel is opaque once
   established: the broker copies bytes and does not read inside TLS.
4. For provider calls, a separate terminating path serves the request: the
   session offers its nonce, the broker checks it in constant time, puts the
   real credential on, forwards, and streams the answer back.
5. Request logging records hosts, decisions, and sizes. Never values.

## Bindings

Each binding names one credential, one match (scheme, host, method, path),
and one rendering (bearer, basic, apiKey, custom headers, passthrough with
scoped placeholder substitution). Exactly one binding must match; ambiguity
is refused. Default-deny egress is required: a default-allow policy may let
a credential ride to a host no binding names.

Substitution is exact, literal, and case-sensitive, on configured surfaces
only. Inserted values are not scanned again, so one secret cannot rewrite
another. Rewritten bodies get a fixed `Content-Length` and lose
`Transfer-Encoding`, because the forwarded request is now buffered.

## Prior art, and what transfers

| project | what it is | what transfers to egi |
| --- | --- | --- |
| Infisical agent-vault | MITM proxy plus vault in one binary, dummy-value substitution, short-lived per-sandbox tokens, strict deny mode | the dummy-value ergonomics, the mint-per-session token lifecycle, the matcher priority tuple, and the identity-keyed limits; the management UI does not transfer |
| OpenSandbox credential vault | egress-sidecar broker, host-written credentials, exact-match bindings with five auth types, fail-closed on sidecar faults | the binding shape (match plus rendering), the single-match rule, fail-closed semantics, the snapshot plus revision-ack protocol, and SNI-gated interception as default |
| kubernetes-sigs agent-sandbox | Sandbox CRD orchestrator delegating isolation to gVisor or Kata | the orchestration layer above egi sessions; egi stays a single-host daemon and does not adopt the CRD model |
| TencentCloud CubeSandbox | microVM sandboxes with a credential vault at v0.4 | the vault-at-egress placement, the match-field set, the audit-level discipline, and the bypass enumeration; the microVM substrate is a backend option, not the core |
| Cloudflare Sandboxes outbound Workers | egress policy as code at the boundary | the policy-at-boundary shape for the strict tier |

## Known limits

- Brokering stops credential theft, not exfiltration through authorized
  channels. An agent with legitimate write access can push data to any host
  it may write. Per-request authorization, allowlists narrowed per task
  phase, and human approval gating narrow that; LLM intent checking and
  egress content scanning are partial at best.
- git over SSH, cert-pinned clients, and raw TCP do not pass a TLS broker.
  They need per-tool answers (credential helper injection, pinned CA
  provisioning) or they stay refused. OpenSandbox issue 1376 records that
  SSH injection has no broker answer; egi states the refusal plainly.
- The ephemeral-CA private key, where MITM is used, is held by the broker
  outside the sandbox and never crosses the boundary.
