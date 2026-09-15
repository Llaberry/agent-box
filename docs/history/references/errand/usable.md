# errand: usable

The lessons and the actual lines, for the session doing the work.
[`findings.md`](findings.md) is the reasoning and the verdicts.

⛔ **Every path below is relative to the corpus**, at the commit in
[`../pins.md`](../pins.md):

```bash
git worktree add ../agent-box-references references
```

The tree is then at `QaidVoid__errand/tree/`.

⚠ **Read the file, do not trust the line number.** A line number is a pointer
into one commit. If what you find there is not what this page says, the
disagreement is a finding and it goes in the entry that sent you.

---

## The exact reading list, by subject

| to build | open | lines |
| --- | --- | --- |
| the backend contract | `src/sandbox/backend.ts` | 14-58 constants and labels, 60-104 `SandboxLaunch` and `SandboxHandle`, 106-131 `CapabilityReport` and the two error types, 133-154 the `Sandbox` trait, 156-189 `agentCommand` |
| the CONNECT broker | `src/sandbox/broker.ts` | 20-46 `hostAllowed`, 48-75 `parseConnect`, 76 the upstream port list, 79-96 `ProviderRoute`, 98-110 the hop-by-hop list, 112-119 `sameSecret`, 132-205 the `Broker` class, 296-326 `readRequestHead`, 328-347 `refuse` and `pipe` |
| the nonce wiring | `src/serve.ts` | 78-92 `brokerableProviders`, 94-106 `providerNonce`, 108-115 `hostOf`, 166-230 the whole proxy-mode block |
| the nonce reaching the session | `src/sandbox/bailey.ts` | 133-145 the map address and the provider prefix, 158-190 `providerConfig`, 192-231 the options and `ProviderBrokering`, 283-295 the environment with the credential held back, 296-320 the agent configuration file |
| the per-session policy | `src/sandbox/policy.ts` | 22-32 the profile names and filenames, 34-72 `SYSTEM_READ` and `SYSTEM_EXECUTE`, 74-93 `RESOLV_CONF`, 105-108 `policyPath`, 110-220 `policyContents` |
| the bailey driver | `src/sandbox/bailey.ts` | 39-51 the inherited variables, 53-76 the runner seam, 77-99 `sessionEnvironment`, 100-131 `parseDoctor`, 233-268 `baileyArgs`, 270-617 `BaileySandbox` |
| the podman driver | `src/sandbox/podman.ts` | 35-49 the restricted network string, 50-61 `FORBIDDEN_ARGS`, 62-80 the runner seam, 81-148 `podmanArgs`, 149-257 `PodmanSandbox` |
| what the daemon reports at startup | `docs/sandboxing.md` | the whole page, and especially "What it says at startup" and "What is not confined" |

---

## ⭐ The six rules to carry, each with the line that teaches it

### 1. Read the CONNECT head one byte at a time

`src/sandbox/broker.ts:296-326`.

Reading past the blank line strips bytes the client expects to carry its TLS.
Reading only the request line leaves the remaining header bytes in the socket,
where they are then piped to the upstream **ahead of the ClientHello** and
corrupt the connection.

⚠ **Both failures look like a network problem, not like a parser bug.** Cap the
head (errand uses 8192 bytes) and accept both `CRLFCRLF` and a bare `LFLF`.

### 2. Compare a nonce in constant time

`src/sandbox/broker.ts:112-119`. An equality operator on a secret says how much
of it matched. In Rust this is `subtle::ConstantTimeEq`, not `==`.

### 3. Refuse a target you had to guess at

`src/sandbox/broker.ts:48-75`. A missing port, a non-numeric port, a port out
of range, a bracketed IPv6 authority, a slash in the host: every one returns
nothing rather than a best guess. ⭐ **A target the broker had to guess at is
one it cannot claim to have checked.**

### 4. Strip hop-by-hop headers in both directions

`src/sandbox/broker.ts:98-110` and its two uses at 174-179 and 186-190. The
list is `connection`, `keep-alive`, `proxy-authenticate`,
`proxy-authorization`, `te`, `trailer`, `transfer-encoding`, `upgrade`, `host`.
⚠ Forwarding `proxy-authorization` upstream sends the caller's own proxy
credential to the origin.

### 5. Match the longest prefix, not the first

`src/sandbox/broker.ts:162-166`. A provider whose route prefix is nested under
another's is otherwise shadowed by it and silently unreachable.

### 6. A wildcard must not match its own apex

`src/sandbox/broker.ts:20-46`. `*.example.com` matches any subdomain and **not**
bare `example.com`, on the stated ground that a wildcard is written when the
apex is not what the session talks to. The comparison is case-folded because a
hostname is.

⛔ **And a lone `*` admits everything.** Keep the capability; do not keep it as
the default. [`findings.md`](findings.md) says what that default costs.

---

## The policy floor, worth copying nearly verbatim

`src/sandbox/policy.ts:34-72`. Read grants, and the reason each is there:

| path | without it |
| --- | --- |
| `/usr`, `/lib`, `/lib64`, `/bin`, `/sbin` | nothing runs |
| `/proc` | a fresh procfs in the session's own PID namespace. ⛔ See the `/proc` finding: a fresh one is necessary and not sufficient. |
| `/etc/ld.so.cache`, `/etc/ld.so.conf`, `/etc/ld.so.conf.d` | nothing dynamically linked starts |
| `/etc/ssl`, `/etc/ca-certificates.conf`, `/etc/ca-certificates`, `/var/lib/ca-certificates`, `/etc/pki` | TLS fails and the provider is unreachable |
| `/etc/nsswitch.conf` | name resolution fails |
| `/etc/services`, `/etc/protocols` | networking tools misbehave; these name no host |

⭐ **Name all five certificate paths, without a per-distribution branch.**
Naming a path that is absent on a given host is already how the list works,
and that is what the closed tracker item settled: Arch resolves through
`/etc/ca-certificates`, openSUSE through `/var/lib/ca-certificates`, Fedora
through `/etc/pki`, Debian and Alpine carry a real file under `/etc/ssl`.

⛔ **The resolver is not from that list.** `src/sandbox/policy.ts:74-93` places
a synthetic `resolv.conf` instead of the host's, because the host's names the
operator's ISP or private network. One file for the daemon, not one per
session: a copy inside a session's own state directory would sit under a grant
placed elsewhere and never be bound.

⛔ **Grants are absolute paths.** After the pivot there is no working directory
to resolve a relative one against.

---

## The podman flags, and the one that was measured

`src/sandbox/podman.ts:35-49`:

```
pasta:--map-host-loopback,none,--map-guest-addr,none
```

The comment records the conditions: measured on podman 5.8.2 with pasta
2025.12.15, where podman's default maps the host into the container and these
two flags stop it.

⚠ **That is somebody else's measurement on somebody else's machine, and it is
a version-pinned claim about two moving projects.** Re-measure before relying
on it. [`../windows/usable.md`](../windows/usable.md) has the same flags
measured again one hop further out, and the answer there is different in a way
that matters.

`src/sandbox/podman.ts:50-61` is `FORBIDDEN_ARGS`: the flags an operator must
not be able to add, because each would undo the boundary.

---

## ⛔ The shape to refuse, stated as a rule

**Never run a host-side tool with the agent's directory as its working
directory, and never let the agent's writable state decide that the tool runs.**

`src/session/pr.ts` is the exhibit. Git executes `.git/hooks/*`,
`credential.helper`, `core.hooksPath`, `core.fsmonitor`, `diff.*.textconv`,
`filter.*` and aliases out of the repository it is pointed at, and the
directory is the agent's.

Three mitigations, weakest to strongest, from the reporter and from this sweep:

| | what it is | why it is not enough on its own |
| --- | --- | --- |
| weak | `git -c core.hooksPath=/dev/null -c credential.helper= ...` plus `GIT_CONFIG_NOSYSTEM=1` | a denylist over a surface that grows with every git release |
| better | do the push from a fresh clone the agent never touched, or from the API | the agent still chose the content |
| ⭐ strongest | **the orchestrator never runs a tool against agent-writable state at all.** It reads bytes out and writes bytes in. | this is the design rule, and it is what T-030 holds |

⚠ **The class is larger than git.** Any tool that reads configuration out of
its working directory is the same exhibit: package managers, linters,
formatters, test runners.

---

## What to leave behind

- the chat gateway, threads, outbox, rendering and slash commands: about a
  third of `src/`;
- the Svelte web interface;
- transcripts, memory and session views;
- `agentCommand` calling `pi --mode rpc` (`src/sandbox/backend.ts:156-189`).
  ⚠ **Keep the argv shape** (provider, model, appended system prompt, resume,
  session directory) and note that the shape is all that transfers: what this
  project starts inside the sandbox is its own business and is decided in
  T-040, not here.
