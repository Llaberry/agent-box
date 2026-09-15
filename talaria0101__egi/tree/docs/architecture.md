# Architecture

The technical reference for egi. When any document conflicts with this one,
this one wins and the other is the defect.

## What egi is

A daemon that runs coding agents from chat. One channel, one thread per
session, several sessions at once. Each session works in one project
directory under a sandbox backend, reaches models through brokered
credentials, and reports back to its thread.

```
chat channel ── thread per session ── session ── sandbox ── egress broker ── provider
                                              │                    │
                                         bailey/podman       allowlist + nonce
                                         (per session)       credential injection
```

## Locked decisions

| decision | ruling | why the alternative lost |
| --- | --- | --- |
| Language: Rust | 2026-09-15 | errand (Deno) works but the sandbox and broker need the control Rust gives: seccomp, Landlock, namespaces, and single-binary deploys without a runtime. |
| Agent runtime: models.dev endpoints, not a spawned CLI | 2026-09-15 | errand shells to `pi` with the credential in the session environment. Any process in the sandbox can read its own environ. Brokering at the boundary removes the credential from the sandbox entirely. |
| Credential pattern: brokered nonces, never masking | 2026-09-15 | Output masking loses to trivial encoding. The agent holds dummy values; the broker substitutes real ones per binding. |
| Work model: todo | 2026-09-15 | The skeleton exists and the work is a backlog of independent items. |
| Windows: daemon inside WSL2, containers below it | 2026-09-15 | Measured in malaria: the drive mount is slow and inotify-blind, but the enforcement story holds and one POSIX shell serves the whole gate. |
| No PowerShell twins for checks | 2026-09-15 | Same reason: WSL2 carries a POSIX shell, so one implementation per check is enough. |

## Crates

| crate | owns |
| --- | --- |
| `egi-core` | the contracts: config, sandbox backend trait, provider types. No IO. |
| `egi-broker` (planned) | the egress broker: CONNECT gating, allowlist, nonce substitution. |
| `egi-sandbox` (planned) | backends: bailey driver, podman driver, capability probing. |
| `egi-provider` (planned) | models.dev bindings: registry, streaming, usage accounting. |
| `egi-chat` (planned) | chat adapters behind one seam: Discord first, others later. |
| `egi-daemon` (planned) | sessions, admission queue, transcript journal, web observer. |

Layer rule: everything depends on `egi-core`. No crate imports a sibling's
internals. Anything touching the outside world is injected, so a test needs
no network, no container, and no clock.

## Sessions

A session is one thread, one project directory, one state directory, one
sandbox. The daemon journals every turn to a transcript on disk. The agent
drives tools inside the sandbox; the daemon relays prompts and results.

State transitions: `starting --ready--> ready <--settled--> working`,
any state `--exit--> ended`. Turn completion is taken from settlement, not
from process end, so an automatic retry never releases an admission slot
early.

## Egress

A session's network namespace reaches exactly one thing: the broker. The
broker gates on the host in the CONNECT line against the allowlist, then
copies bytes without reading inside TLS. A separate terminating path serves
provider calls: the session offers a nonce, the broker puts the real
credential on, forwards, and streams the answer back.

Ports: 443 only, unless the operator names more. Hosts: the provider plus
the operator's allowlist. A lone `*` keeps the broker as an audit
pass-through that still gates the port and logs every connection.

## Config

Searched in order: `$ERRAND_CONFIG` unset means `~/.config/egi/config.json`,
then `/etc/egi/config.json`, then `config.json` in the working directory.
On Windows the same idea with Windows spellings; [`windows.md`](windows.md)
names them. A committed `.example` twin documents every key with fake
values. Secrets live in the platform store or an ignored file, never in the
tree.

## Limits

- No backend caps aggregate disk use without a sized filesystem under it.
  Use is measured, and a session passing its budget is stopped.
- Per-session memory, cpu, and process limits need a cgroup delegation.
  Without one the daemon reports the gap and refuses to start unless the
  operator accepted it.
- The daemon holds the chat token and starts sandboxes. It is not confined.
