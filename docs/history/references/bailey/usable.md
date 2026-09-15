# bailey: usable

The contract, the failure modes and the lines, for the session building the
Linux backend. [`findings.md`](findings.md) is the reasoning.

---

## ⛔ The version floor, and the refusal

**0.1.4 or later.** Nothing earlier.

| version | why it is refused |
| --- | --- |
| below 0.1.3 | an unvalidated architecture let a denied syscall slip past (`51a30ab`), and btrfs ioctls Landlock does not mediate were not denied (`587fc6f`) |
| 0.1.2 and 0.1.3 | a relocated grant dropped the invocation working directory (issue 6, fixed by `b8a7eea` in 0.1.4) |
| below 0.1.4 | no `--egress-proxy`, so there is no way to lock a session to one broker and no fail-closed strictness |

⭐ **The probe reads the version and refuses below the floor.** A backend that
runs on whatever is installed is a backend whose guarantees change with the
host's package manager.

---

## The kernel table, and what each gap costs

From `docs/reference/kernel.md`. ⚠ Read from that page, not measured here.

| feature | needed for | since | without it |
| --- | --- | --- | --- |
| Landlock | the filesystem policy | 5.13 | no filesystem or network enforcement |
| Landlock ABI 4 | network rules | 6.7 | ⛔ **the network policy is not applied, and the config is still valid**. bailey negotiates down rather than failing. |
| Landlock ABI 5 | device ioctl restrictions | 6.10 | that subset is skipped |
| Landlock ABI 6 | scoping abstract sockets and signals | 6.12 | host abstract sockets and host processes stay reachable |
| unprivileged user namespaces | the isolation layer, and a denied egress | long-standing, often disabled | falls back to Landlock and seccomp; the network becomes TCP-port-only |
| cgroup v2 with delegation | memory, cpu and process limits | long-standing | limits are skipped |
| seccomp | the syscall denylist | long-standing | the run fails to establish |
| BTF | the audit backend | 5.x with `CONFIG_DEBUG_INFO_BTF` | audit is unavailable |

⛔ **ABI 4 is the one that fails quietly.** `bailey show` prints the policy as
written and does not say what the kernel will do with it. A networked session on
a kernel below 6.7 runs with its egress policy silently skipped.

⭐ **So this project's probe asserts the ABI, not the config.** That is the one
check bailey's own report cannot self-certify, and
[`../windows/usable.md`](../windows/usable.md) records a prior project reaching
the same conclusion independently.

Landlock must also be in the kernel's `lsm=` boot parameter list.

```sh
grep landlock /sys/kernel/security/lsm
```

⚠ **That command is necessary and not sufficient**, and a measured case proves
it: a WSL2 kernel listed `landlock` in its LSM set, answered a Landlock version
query with `EINVAL`, and published no landlock directory under securityfs.
[`../windows/findings.md`](../windows/findings.md) carries it. ⛔ **Probe by
attempting the real setup in a throwaway child.** That is bailey's own method
and the reason is stated: AppArmor can block the `unshare` even where the
sysctl permits it.

---

## ⭐ The one flag that makes this work

```
--egress-proxy ADDR:PORT
```

`crates/bailey/src/cli.rs:112`, parsed at `:415-427`, threaded into the inner
invocation at `:549-551`, and made strict at `:578-606`:

- it requires `pasta` on PATH, and **refuses the run** when it is absent;
- it requires user namespaces, and **refuses the run** when they are absent;
- it locks the session to that one address and port.

⭐ **The refusals are the feature.** Without it a policy that allows any egress
silently degrades to the TCP-port-only mode
(`crates/bailey/src/backend/network.rs:45-59`), which leaves UDP, QUIC, DNS,
ICMP and the host's loopback reachable. With it, the run either gets the
namespace or does not start.

The value errand pairs with it is a link-local address that routes nowhere else:
`src/sandbox/bailey.ts:133`, `169.254.169.1`, with `NO_PROXY` set to the same
address so a client does not try to proxy its own call to the broker
(`src/sandbox/bailey.ts:352-353`).

---

## The two modes, and how to read which one is in force

`crates/bailey/src/backend/network.rs:22-43`:

| mode | what it enforces |
| --- | --- |
| `Isolated` | own namespace, loopback only, no route off the host. Every protocol fails. |
| `LandlockOnly` | the host's namespace with Landlock TCP port rules. ⛔ Other protocols are unrestricted, and so is the host's loopback. |

`:51-58` chooses: `Isolated` only where the policy denies all egress **and**
binds no port **and** user namespaces are available.

⚠ **A nested run cannot create its own namespace**, because the outer seccomp
filter denies `unshare`. `:71-98` is how the inner run finds out what it
inherited: the outer run publishes `BAILEY_SANDBOX_NET` as `isolated` or `host`.
⛔ **Guessing is wrong in both directions** and the comment says so: claiming
UDP is unrestricted where there is no route to send it on is as wrong as staying
silent where there is one.

---

## The policy floor, and what bailey will not enforce

`crates/bailey/src/profiles/ai-agent.toml` is the shipped starting point: the
untrusted floor's system paths, outbound 443, no home directory, no devices.
The project directory is granted separately, because a profile cannot know
where you are.

⛔ **Three things a policy cannot express, so they have to be held elsewhere:**

1. **Which host a session may reach.** `egress_allow`'s `host` field is
   advisory: the port is enforced and the host is ignored, with a warning.
   ⭐ **The host allowlist lives in the broker.**
2. **Denying a path nested inside a granted one**, without the isolation layer.
   Landlock rights only add, so a narrower rule cannot take access away; the
   mount namespace covers the path over instead. ⛔ Never pass `--no-isolate`.
3. **Metadata on a read-granted path.** `chmod`, `chown`, `utimensat` and
   `setxattr` succeed. ⚠ It cannot change content and cannot gain a privilege,
   but it can make a file unreadable to its owner or move its timestamps.
   ⭐ **Grant read narrowly**: a directory granted for one file inside it
   exposes the metadata of everything else in it.

---

## What the daemon must do with the report

bailey reports what it could not enforce, per run and via `bailey doctor`.
errand parses that report rather than hardcoding what bailey does
(`src/sandbox/bailey.ts:100-131`, `parseDoctor`). ⭐ **Copy that: parse the
tool's own report, do not model it.**

⛔ **A gap stops the daemon by default.** errand's
`sandbox.requireFullEnforcement` is the escape hatch and it defaults to
refusing. The reason is in errand's own `docs/sandboxing.md`:

> Presenting a weaker boundary as if it were a stronger one is worse than the
> weaker boundary itself, because it takes away the chance to decide about it.

---

## The discipline, restated as rules

1. ⛔ **A config found in a directory is not policy until it is accepted.** A
   repository can ship the file that is meant to contain the code in it. bailey
   records the acceptance against the contents, so an edit asks again.
2. ⭐ **Output means something needs attention.** A run that enforced everything
   says nothing (`network.rs:100-108`).
3. ⛔ **Remove config that does nothing rather than carrying it.** The
   `on_violation` hook was specified, built, measured to be unfirable on a
   normal host, and deleted (`docs/roadmap.md`).
4. ⚠ **A denied access is invisible.** There is no way to see a Landlock denial
   without the kernel audit subsystem enabled at boot. A session that fails
   under enforcement gives its own error and nothing more, so the daemon's own
   diagnostics have to be good enough to work without one.
