# architecture.md

The technical reference. ⛔ **When any other document conflicts with this one,
this one wins and the other is the defect.** Fix it in the same change.

⚠ **Nothing here is on disk yet.** This project has no code. Every statement
below is a **decision** that an entry in [`../TODO/INDEX.md`](../TODO/INDEX.md)
implements, and each section names the entries that own it. A sentence here in
the present tense describes what the design commits to, not what a reader can
run.

⛔ **A claim about the running system belongs here only once it is measured.**
Until then it belongs in the entry, as a premise, marked read rather than
measured.

---

## What the thing is

One binary. It runs on an operator's own machine, holds every credential, and
starts confined sessions that hold none.

```
                      the operator's machine
   ------------------------------------------------------------
   agent-box (the daemon)                 the credential store
     - the broker           <---------->  never leaves this process
     - the session manager
     - the policy generator
          |
          | starts, and is the only route off the host
          v
   ------------------------------------------------------------
   one session                            holds no credential
     - the agent process, confined by a backend
     - a project directory it may write
     - a state directory it may write
     - one route out: the broker, at a link-local address
   ------------------------------------------------------------
```

⭐ **The one invariant everything else serves:** a session's network namespace
has no route off the host except to the broker, and the broker is the only
thing that holds a credential.

---

## The trust boundary, stated once

| | trusted | untrusted |
| --- | --- | --- |
| the daemon process | yes | |
| the broker, inside it | yes | |
| the policy, and where it is written | yes | |
| the operator's configuration | yes | |
| **the agent process** | | ⛔ yes |
| **the project directory** | | ⛔ yes, every byte |
| **the session state directory** | | ⛔ yes, every byte |
| **anything the agent says, writes, or asks for** | | ⛔ yes |
| **any upstream's response body** | | ⛔ yes |

⛔ **The agent runs as the same user as every tool it invokes.** Assume it can
read any file it can reach, make any syscall the filter permits, inspect its own
memory, and attempt arbitrary egress. It may be hostile through prompt
injection, through adversarial tool output, or through a compromised
dependency, and none of those require it to be malicious to begin with.

### ⛔ The rule that follows, and it is the one that was learned the hard way

**The daemon never runs a tool against state the agent controls.**

Not `git` in the project directory, not a package manager, not a linter, not a
formatter, not a test runner. Every one of those reads configuration out of the
directory it runs in, and several of them execute it.

[`history/references/errand/findings.md`](history/references/errand/findings.md)
carries the exhibit: a daemon that opened pull requests by running `git` on the
host, with the agent's project directory as the working directory, gave any
session host code execution as the daemon's user through `.git/hooks/pre-push`,
with no sandbox escape involved.

⭐ **The daemon reads bytes out and writes bytes in.** T-030 holds this and it
is checkable: a process spawned by the daemon with a working directory under
the project root is a failed check.

---

## The components

| crate | owns | entries |
| --- | --- | --- |
| `box-core` | identifiers, error types, cancellation, the types shared across the boundary. ⛔ No IO. | T-001, T-002 |
| `box-policy` | what a session may touch, as data: generation, validation, serialization. ⛔ No secret ever enters a policy. | T-012, T-016 |
| `box-broker` | the CONNECT proxy, the binding table, the injection gate, the certificate authority, the credential store, the provider slots, the usage accounting | T-020 to T-029, T-042, T-044, T-120, T-121 |
| `box-sandbox` | the backend trait, the capability report, one module per backend | T-010 to T-017 |
| `box-session` | lifecycle: launch, supervise, tear down, discover orphans, admission, the harness adapter | T-030 to T-033, T-043, T-046, T-047, T-102, T-122 |
| `box-chat` | the channel, threads, commands | T-100, T-101 |
| `box-web` | the interface, its login, its authorization, its views | T-110, T-111, T-112 |
| `box-plugin` | the extension surface and the capability grants | T-070, T-071 |
| `box-cli` | the binary. ⭐ The only crate that knows about all of the above. | T-003 |
| `xtask` | the gate, the probe, the miner, the battery. Not shipped. | T-050 to T-056, T-064 |

⚠ **`box-chat` and `box-web` are the top of the graph**, above `box-session` and
below nothing but the binary. ⛔ **Neither may reach `box-broker` directly**: a
surface that can read the credential store is a surface that can leak it, and
the layering check in T-052 is what holds that.

⛔ **Layering is strict: depend only downward.** A dependency that points upward
is a failed check, not a review comment. T-052 owns the check.

⚠ **`box-plugin` is not created until something needs it.** A plugin surface
with no plugin is machinery with one caller, and the reference that has one paid
for it in six separate file splits
([`history/references/orchestration/findings.md`](history/references/orchestration/findings.md)).

---

## The sandbox backend contract

One trait, sized to exactly what a backend does. ⛔ **It is not a plugin
system.**

```rust
trait Sandbox {
    fn name(&self) -> BackendName;

    /// Check that this backend can run here, and report what it can enforce.
    /// Never falls back to another backend, and never to running unconfined.
    fn probe(&self) -> Result<CapabilityReport, Unavailable>;

    fn launch(&self, launch: &Launch) -> Result<Handle, LaunchError>;

    /// Sandboxes this system owns that no live session claims.
    fn orphans(&self) -> Result<Vec<String>, Error>;
    fn remove_orphans(&self, names: &[String]) -> Result<usize, Error>;
}
```

`CapabilityReport` carries `gaps` and `notes`. ⛔ **A gap stops the daemon**
unless the operator has explicitly said otherwise, and the reason is not
negotiable:

> Presenting a weaker boundary as if it were a stronger one is worse than the
> weaker boundary itself, because it takes away the chance to decide about it.

⭐ **The probe attempts the real setup in a throwaway child.** It does not read
a sysctl and it does not read the LSM list. Two references measured the same
failure independently: a kernel that lists `landlock` in its LSM set, answers a
version query with `EINVAL`, and publishes no landlock directory under
securityfs. T-011.

### The paths a session sees

⛔ **The agent is never shown a host path.** A path carries the operator's name
and the shape of their machine, and that is not access the sandbox can take back
once the agent has read it.

| constant | value |
| --- | --- |
| the project | `/workspace` |
| the session's own state | `/state` |
| the agent's home | `/state/home` |
| the agent's own bin, first on `PATH` | `/state/home/bin` |
| the broker | `169.254.169.1`, at a port the daemon chooses |

⚠ `/state/home/bin` is inside what the agent may write. A wrapper placed there
is a habit to break, never a boundary to enforce.

### The backends

| backend | platform | what confines the session | entry |
| --- | --- | --- | --- |
| `bailey` | Linux | Landlock, seccomp, and user, PID, UTS and network namespaces, as a host process. No image. ⛔ **Version floor 0.1.4.** | T-013 |
| `podman` | Linux, and Windows through a podman machine | a container from an operator-supplied image, rootless | T-014, T-015 |

⛔ **There is no unconfined mode and no fallback between backends.** A probe
that cannot run the configured backend refuses.

---

## The broker

⭐ **The whole reason this project exists.** Full design in
[`credential-brokering.md`](credential-brokering.md); this section is the
contract other components depend on.

A session's only route off the host is one CONNECT proxy at a link-local
address. The session is given a per-run random nonce in place of every
credential. The broker gates the connection, then attaches the real credential
at its own end.

⛔ **The eight-condition injection gate is in
[`credential-brokering.md`](credential-brokering.md) and it is normative.** Any
one of them failing is a refusal, never a warning and never a forward without
the credential.

⛔ **`egress.allow` has no default that admits anything.** An empty allowlist
refuses every host. A configuration that names no host is a configuration that
does not start.

### The provider slots

⭐ **A slot is the unit that carries a credential**, and a session is assigned
one. The deployment has two or three provider subscriptions, each allowing a
small number of concurrent sessions, so the pool is finite and the queue in
front of it is part of the design rather than an error path. T-042, T-043,
T-044.

⛔ **A session never learns how many slots exist, which provider it got where
the choice was automatic, or what any other session holds.** Each of those is a
fingerprint of the deployment, and the last one lets a session time its requests
to starve somebody else's.

### The provider sandbox

⭐ **A provider reachable only through a vendor's own tool gets its own
sandbox**, separate from the agent's and from the daemon. The tool holds a
nonce; the broker holds the subscription credential and injects it, refresh flow
included. The agent sandbox reaches a shim that speaks one model API, so the
agent side does not change with the vendor behind it. T-045.

⛔ **Never in the agent's sandbox.** A vendor tool is a different trust level
from the agent, and co-locating them hands the agent whatever it holds.

---

## The policy

Generated per session, written into the session's own state directory, outside
the project.

⛔ **A policy living in the project would be a policy the agent could rewrite,
which is not a policy.**

⛔ **No secret ever enters a policy.** Everything in one is a name or a path.

⛔ **Paths are absolute.** After the pivot there is no working directory to
resolve a relative one against.

⭐ **The environment is built, never inherited.** Only named variables cross,
and the names are reported at startup while the values never are.

⚠ **Three things a Linux policy cannot express**, so they are held elsewhere:

1. **which host a session may reach**: the backend's host rules are advisory,
   so the allowlist lives in the broker;
2. **denying a path nested inside a granted one**, without the isolation layer.
   ⛔ Isolation is never disabled;
3. **metadata on a read-granted path**: `chmod`, `chown`, `utimensat` and
   `setxattr` succeed. ⭐ Grant read narrowly.

[`history/references/bailey/usable.md`](history/references/bailey/usable.md)
carries each with its source.

---

## The session lifecycle

```
  requested
     |  admission: is there capacity, is the requester allowed
     v
  probing        the backend reports what it can enforce here
     |  ⛔ a gap stops this, unless the operator said otherwise
     v
  policy written into the session's state directory
     |
     v
  broker bindings installed for this session id
     |
     v
  running        the agent holds nonces, never a credential
     |
     v
  settled        the turn ended
     |
     v
  torn down      the sandbox, the bindings, and the nonces, all removed
```

⛔ **Teardown removes the bindings and the nonces, not only the sandbox.** A
nonce that outlives its session is a credential with no owner.

⛔ **Orphan discovery runs at startup.** A sandbox this system owns that no live
session claims is removed, and a daemon that cannot enumerate its own sandboxes
does not start. T-032.

### ⛔ The channel between the daemon and the agent

The daemon reads the agent's output on one stream and writes to another. That
stream is trusted, so it has to be a channel the agent cannot forge into.

⚠ **A prior design left the agent process's own file descriptors writable
through `/proc`, and forged events reached the transcript as genuine agent
output.** T-031 holds this. The design commits to: a fresh procfs in the
session's own PID namespace, and a channel whose write end is not reachable from
inside the session by any path other than the intended one.

⛔ **Whatever the mechanism, the acceptance is a test that forges an event and
watches it be rejected.** A trusted channel the untrusted party can write to is
not a channel.

---

## Configuration

One file, searched in order, with an environment variable naming one outright
and skipping the search.

| platform | order |
| --- | --- |
| Linux | `$XDG_CONFIG_HOME/agent-box/config.toml`, `~/.config/agent-box/config.toml`, `/etc/agent-box/config.toml`, `./config.toml` |
| Windows | `%APPDATA%\agent-box\config.toml`, `%USERPROFILE%\.config\agent-box\config.toml`, `%ProgramData%\agent-box\config.toml`, `.\config.toml` |

⛔ **No credential is in it.** Credentials live in the broker's own store, which
is written by the operator and read by nothing else. The configuration names
them; it never carries them.

⛔ **An unknown key is a refusal, not a warning.** An operator who wrote a
misspelled key believes the setting is in force.

⚠ **A setting that no code reads does not ship.** So does a value the engine
reads that nobody can set. Both are in
[`conventions/forbidden-patterns.md`](conventions/forbidden-patterns.md).

---

## The surfaces

⭐ **The target is parity with the project this replaces, and then more.** T-090
carries the ruling and the measurement behind it.

| surface | what it is | entries |
| --- | --- | --- |
| the channel | one configured chat channel, a thread per session, several at once. ⛔ The chat token never enters a sandbox. | T-100, T-101 |
| work that lands in a repository | the forge credential is brokered, and ⛔ **the daemon composes what is published**, from bytes the agent wrote and the daemon read | T-102 |
| the interface | login through the chat service's own identity. ⭐ **A user drives exactly the sessions they started**, and the operator role is configuration rather than a chat-service role. | T-110, T-111 |
| metrics | ⭐ counted at the broker, where the agent cannot forge them, and rendered in three places from one source | T-120, T-121, T-122, T-112 |

⛔ **Adding a login changes what the interface is.** Without one, the address it
binds to is the access control. With one it is a second trusted surface and it
needs per-user authorization on every call, not an operator console with a login
page in front. T-110 carries the seven rules that hold it.

---

## What is deliberately not here

| | why |
| --- | --- |
| a hosted control plane | the operator holds their own credentials. That is the requirement this project exists to satisfy. |
| warm pools, claims, templates, cluster orchestration | machinery for a scale this project does not have |
| a terminal runtime owning agent terminals across machines | a real product solving a problem this boundary does not have. ⚠ Read it again if sessions ever need to survive a daemon restart. |
| an agent loop of this project's own | ⭐ **ruled 2026-09-15**: own the protocol boundary, drive a harness that speaks it. T-040. |
| gVisor or Kata as a backend | ⚠ a real question, recorded in T-091, with the measurement to fetch named |

---

## The limits this design does not remove

⛔ **Read [`limits.md`](limits.md).** The most important one is here as well,
because a reader of this page must not finish it believing something false:

⭐ **Brokering closes credential theft. It does not close exfiltration through
an authorized channel.** [`limits.md`](limits.md) states what that means and
which mitigations are real. Nothing in this document changes it, and every
reference read for this project has the same problem open.
