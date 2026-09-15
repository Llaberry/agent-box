# Security

## The threat model

⛔ **egi serves one operator running agents on chat, with several sessions
at once.** Tenants are the operator's own sessions, not mutually hostile
strangers. That decides everything below: per-session isolation is strict,
cross-operator tenancy is out of scope.

**In scope:** a hostile agent process (prompt injection, adversarial tool
output, compromised dependency) reading files, probing the network,
escalating privileges, persisting, or reaching another session's state.
Credential theft through any sandbox surface. Container escape to the
distro the daemon runs in.

**Explicitly out of scope:** host-kernel exploits (shared kernel by
design; gVisor or Kata backends narrow this when configured), side
channels, and exfiltration through authorized channels (an agent allowed
to write somewhere can write data there; brokering stops theft, not
misuse of granted access).

⚠ If the audience ever changes to multi-tenant hosting, this file is the
first thing to re-derive.

---

## Who holds what

| secret | held by | where it lives | what it reaches if leaked |
| --- | --- | --- | --- |
| chat token | the operator | the config file, readable by the operator alone | ⭐ post as the bot, read the channel, drive sessions |
| provider credential | the operator | the daemon's secret store; sessions hold nonces only | ⭐ model usage billed to the operator, plus whatever the provider account reaches |
| GitHub token, when configured | the operator | the daemon's secret store; injected per binding at egress | ⭐ read and write on every repository the token reaches |
| broker CA key, where MITM is used | the daemon | the broker, outside every sandbox | ⭐ decrypt brokered TLS until rotated |

⭐ **The agent holds none of these.** That is the design, not a hope:
[`credential-brokering.md`](docs/credential-brokering.md) states the
mechanism.

---

## Trust boundaries

| boundary | what enters | what validates it |
| --- | --- | --- |
| chat channel | prompts, commands, attachments | allowlisted user ids; attachments scanned before they reach a session |
| agent stdout (protocol) | tool calls, text, usage reports | the protocol parser; a violation ends the session |
| egress broker | CONNECT targets, nonces | the allowlist and constant-time nonce check |
| config file | operator intent | schema validation; the daemon refuses what it cannot enforce |

---

## The invariants

⛔ Non-negotiable. Each is a rule the code must hold, not an aspiration:

- every session runs confined; there is no unsandboxed mode;
- no real credential crosses into a sandbox, ever;
- every path into an operation passes the same gate;
- every secret comparison is timing-safe;
- nothing is ever logged that could reconstruct a credential;
- a backend gap stops the daemon unless the operator accepted degraded mode
  in advance, in the config, having read the report.

---

## Reporting a vulnerability

Open an issue with `[security]` in the title, or message the operator
directly. Expect an acknowledgement within a week. Do not promise a faster
timeline than that.

---

## Known limits

⭐ **The truths that are tempting to leave out.**

- Masking is damage control, not a boundary. Transcript scrubbing exists so
  a leaked value does not sit on disk; it does not stop the leak.
- The daemon itself is unconfined. It holds the chat token and starts
  sandboxes.
- Disk use is measured, not capped. A session passing its budget is stopped.
- An agent with legitimate write access can exfiltrate through it. Narrow
  the allowlist per task phase and gate consequential writes on approval.
