# sandbox-model.md

What confines a session, what each backend can promise, and where each promise
ends.

[`architecture.md`](architecture.md) wins any conflict with this page.
⚠ **Nothing here is on disk yet.** Each section names the entry that builds it.

⛔ **A sandbox that overstates itself is worse than one that does not exist,
because you make different decisions.** That sentence is taken from a
reference's own limitations page and it is the standard this page is held to.

---

## The threat

⛔ **[`../SECURITY.md`](../SECURITY.md) owns the threat and the attacker**, and
this page does not restate them. Read it first: what confines a session only
makes sense against what it is confining.

The short form, so this page is not unreadable on its own: an untrusted program
running as the operator, which may be hostile and which knows it might be
sandboxed.

### ⛔ Tenancy: users are mutually untrusted

⭐ **RULED by the operator, 2026-09-15: full isolation between users.**

The deployment is a public community channel, so the people driving sessions are
members of a server rather than colleagues. A session must not be able to read
or influence another user's session, its workspace or its output.

What follows:

| | |
| --- | --- |
| **its own workspace and its own state** | never shared, never reused across users |
| ⛔ **no shared writable cache** | a writable cache shared between mutually untrusted users is a way to reach them |
| ⚠ **a shared read-only cache is possible and is not free** | it is faster, and a poisoned entry reaches every session. If one is ever added, the daemon owns what goes into it and no session writes to it. |
| **its own provider slot** | T-042, so revoking one session's access does not touch another's |
| ⛔ **its own broker bindings and its own nonce** | removed at teardown, T-032 |

⚠ **This costs disk and warm-cache time**, and that is the trade the ruling
made. [`../SECURITY.md`](../SECURITY.md) flags the assumption as load-bearing,
and T-091 is where it gets revisited if the answer ever changes.

### ⛔ What else is assumed, and each assumption is a way this fails

| assumption | what breaks it |
| --- | --- |
| the kernel is sound | a Landlock, namespace or seccomp defect undoes everything |
| the host was not already compromised | this confines what it starts; it does not clean up what was there |
| ⭐ the policy is what the operator meant | a policy granting the home directory confines nothing interesting |
| ⛔ a configuration found in a directory is not policy until accepted | otherwise a repository ships the policy meant to contain it. T-014. |

---

## Backend: bailey, on Linux

A session runs as a confined host process. Landlock for the filesystem, seccomp
for the syscall surface, and user, PID, UTS and sometimes network namespaces. No
image to build, and the session uses the host's own tools. T-013.

⛔ **Version floor 0.1.4.** Earlier versions carry either a seccomp bypass
(below 0.1.3) or a working-directory regression (0.1.2 and 0.1.3), and none
before 0.1.4 can lock a session to one broker.
[`history/references/bailey/usable.md`](history/references/bailey/usable.md) has
the evidence.

### ⛔ The trap that decides the network design

**A partial egress allowance drops the network namespace.**

The namespace is chosen only where the policy needs no connectivity at all. A
policy that allows any outbound, which is every policy for an agent that calls a
model API, falls back to Landlock's TCP port rules, and those cover TCP connect
and bind and nothing else.

⭐ **So a session confined that way can send UDP, QUIC, DNS and ICMP anywhere,
and can reach every service on the host's loopback.** That is not a defect in
the tool; its own agent profile documents it. It is a defect in any design that
reads "sandboxed" and stops there.

### What closes it

`--egress-proxy ADDR:PORT`, which locks the session to one broker and is
**strict**: without `pasta` on PATH the run is refused, and without user
namespaces the run is refused.

⭐ **The refusals are the feature.** Without them the run silently degrades to
the port-only mode. With them it either gets the namespace or does not start.

⚠ **This is four days old at the commit this project pinned.** Treat it as
load-tested by nobody, and let the acceptance for T-013 be a measurement rather
than a reading.

### What the policy cannot express

| | why it matters |
| --- | --- |
| **which host a session may reach** | the host field of an egress rule is advisory: the port is enforced and the host is ignored. ⭐ The host allowlist therefore lives in the broker, and nowhere else. |
| **denying a path nested inside a granted one** | Landlock rights only add, so a narrower rule cannot take access away. The mount namespace covers the path instead. ⛔ **Isolation is never disabled.** |
| **metadata on a read-granted path** | `chmod`, `chown`, `utimensat` and `setxattr` succeed on a file granted only read. It cannot change content and cannot gain a privilege; it can make a file unreadable to its owner or move its timestamps. ⭐ Grant read narrowly: a directory granted for one file exposes the metadata of everything in it. |

### ⚠ A denied access is invisible

Landlock denies silently, and seeing a denial needs the kernel's audit subsystem
enabled at boot plus permission to read its records, which a normal host does
not provide. ⛔ **A session that fails under enforcement gives its own error and
nothing more**, so this project's own diagnostics have to be good enough to work
without one. T-033.

---

## Backend: podman

A session runs in a container from an operator-supplied image, rootless. T-014.
Use it where an assembled filesystem is preferable to the host's, and on
Windows, where it is the only option.

The flags that carry the boundary, each with what it was measured to do:

| flag | effect |
| --- | --- |
| `--cap-drop=ALL` | zero effective capabilities in the container |
| `--read-only` plus a tmpfs at `/tmp` | root filesystem writes refused, `/tmp` writable |
| `--memory`, `--cpus`, `--pids-limit` | arrive in the container's own cgroup |
| `--userns=keep-id` | maps to the invoking user |
| `pasta:--map-host-loopback,none,--map-guest-addr,none` | stops pasta mapping any host address into the container |

⚠ **Those are somebody else's measurements on somebody else's machines**, one
run each. T-064 owes this project its own.

⛔ **An operator cannot add an arbitrary flag.** A refusal list names the ones
that would undo the boundary, and an unknown flag is refused rather than passed
through.

---

## ⛔ Windows: the containment claim changes meaning

**The restricted flags close the podman machine, not Windows.**

In a podman machine the "host" those flags protect is the machine itself. The
Windows host is one route further out, across the WSL NAT, and what crosses that
NAT is decided by the Hyper-V firewall for WSL, which is a policy of the Windows
host and **invisible from inside the machine**. T-015.

Measured once, on a stock Windows Server 2025 image: with the restricted flags
the internet and DNS were reachable and the machine's mapped address, the
Windows host and `host.containers.internal` were all unreachable.

⛔ **That is a measurement of a default, not a guarantee about a
configuration.** A firewall change reopens the path and nothing inside can see
it. ⭐ **So the daemon reports it as a gap and says how to check it**, and with
the network denied outright the question disappears.

Three more Windows facts, each a documented limit rather than a defect:

| | |
| --- | --- |
| `:Z` relabelling | there is no SELinux in the machine to relabel with. The flag is **accepted and does nothing**. ⚠ A backend that assumes relabelling because the command exited 0 is making a claim it did not check. |
| inotify and permission bits across the drive mount | not delivered, not enforced. Watch modes and some test runners misbehave in ways no flag fixes. |
| drive-mount throughput | one fixed write measured about eleven times slower than the machine's own filesystem. ⭐ Put the project root in the machine's filesystem, or run the daemon inside WSL. |

⛔ **bailey is refused on a Windows host, with the reason.** That is correct
rather than a gap: it is a Linux tool and its own roadmap says non-Linux
platforms are not planned.

---

## What is enforced, and by what

⚠ **Status column is what the design commits to. Nothing is measured yet**, and
T-064 is the entry that changes that.

| property | mechanism | status |
| --- | --- | --- |
| cannot read or write ungranted paths | Landlock, or the container's filesystem | design |
| cannot see ungranted paths | mount namespace, or the image | design |
| cannot see or signal host processes | PID namespace | design |
| ⭐ cannot reach anything but the broker | ⛔ **the network namespace, plus `--egress-proxy`**, or the container's own network | design, and T-064 measures it first |
| cannot reach host abstract sockets | Landlock scoping on 6.12 and later, and the network namespace | design |
| cannot exceed memory, process and cpu limits | cgroup v2 | design, ⚠ best-effort: skipped without a delegated cgroup, and the run says so |
| cannot read the operator's environment | the built environment | design |
| cannot supply its own policy | the policy lives outside the project, and a discovered configuration is not policy until accepted | design |

---

## ⛔ Where the guarantees end

Each of these is a property of the mechanism, not pending work.
[`limits.md`](limits.md) carries the full set; these are the sandbox's own.

- **Anything granted.** A session granted a device can talk to that device, and
  device drivers are a large kernel attack surface. Every grant is a decision.
- **Variables you name.** The environment is deny-by-default and a named
  variable is forwarded whole. ⛔ Naming one that holds a credential puts that
  credential in the session.
- **Side channels.** Timing and resource observation are not addressed.
- **A shared kernel.** Every guarantee here is the kernel's. T-091 records the
  question of when that stops being enough and names the measurement to fetch.
- ⭐ **Confinement is not authorization.** A session confined perfectly, holding
  no credential, can still ask the broker to do something the operator would not
  have agreed to. [`credential-brokering.md`](credential-brokering.md) and
  [`limits.md`](limits.md).
