# Sandbox

The backend contract, the probe, the policy, and one module per backend.

[`INDEX.md`](INDEX.md) is the list. [`../docs/sandbox-model.md`](../docs/sandbox-model.md)
is what each backend can promise.

---

## T-010: The backend trait and the capability report

**Source:** `QaidVoid/errand` `src/sandbox/backend.ts` (`58a178b`). Read [`../docs/history/references/errand/usable.md`](../docs/history/references/errand/usable.md) first.
**Category:** sandbox
**Priority:** P1
**Effort:** S
**Status:** open
**Blocked by:** T-002

---

### Problem

Two backends confine a session in completely different ways, and the session
manager must treat them alike: probe, launch, stop, discover orphans. Without
one contract every caller branches per backend, and adding a third rewrites the
manager.

### Premise

READ, not measured. errand's contract is 189 lines total, at
`src/sandbox/backend.ts`:

| what | lines | note |
| --- | --- | --- |
| path and label constants | 14-47 | ported by T-002 |
| `SandboxLaunch` | 49-75 | ⚠ its `env` field is documented as "holding the provider credential". ⛔ **This project's equivalent holds nonces.** |
| `SandboxHandle` | 77-99 | including `toHostPath`, which returns nothing for anything outside the project |
| `CapabilityReport` | 101-111 | `gaps` and `notes`, and gaps are fatal by default |
| `SandboxUnavailableError`, `SandboxLaunchError` | 113-131 | |
| the `Sandbox` interface | 133-154 | `probe`, `launch`, `listOrphans`, `removeOrphans` |
| `agentCommand` | 156-189 | ⚠ argv shape only; what this project starts is T-040's |

Its own header states the sizing rule and it is worth keeping:
**"Sized to exactly what a backend does. It is not a plugin system."**

### Approach

In `box-sandbox`, port lines 49-154 as Rust:

- `Launch`, carrying the session id, the host project path, the host state
  directory, the environment, and whether this is a resume;
- `Handle`, carrying the sandbox name, a stop that escalates to a kill after a
  grace period and is safe to call twice, and `to_host_path`. ⭐ **That last one
  is a security boundary, not a convenience**: it returns `None` for anything
  outside the project, which is what stops a crafted path in a tool call making
  the daemon read a file the session itself could not;
- `CapabilityReport { gaps: Vec<String>, notes: Vec<String> }`;
- `Unavailable` and `LaunchError` as typed errors;
- the `Sandbox` trait as in [`../docs/architecture.md`](../docs/architecture.md).

⛔ **What it must not do:** add a hook, a middleware point, or a registry. Two
backends justify a trait. Nothing here justifies a plugin system, and errand's
own header says so.

⛔ **`probe` never falls back.** Not to another backend, not to unconfined.

### Prove

```bash
cargo test -p box-sandbox trait::
```

Passing: exit 0, with named tests for a fake backend satisfying the trait as an
object, `to_host_path` returning `None` for `..` traversal and for an absolute
path outside the project, and `stop` being safe to call twice.

### Closing

Not closed.

---

## T-011: A probe that attempts the real setup, and a gap that stops the daemon

**Source:** `QaidVoid/bailey` `docs/reference/kernel.md` (`d3c73f7`); `talaria0101/malaria` `docs/windows.md` (`499e897`). Both in [`../docs/history/references/bailey/usable.md`](../docs/history/references/bailey/usable.md) and [`../docs/history/references/windows/usable.md`](../docs/history/references/windows/usable.md).
**Category:** sandbox
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-010

---

### Problem

A host either can enforce what the configuration asks for, or it cannot. A
daemon that starts without knowing which is a daemon making a promise it has not
checked.

### Premise

⭐ **MEASURED, by two projects independently, and this is the strongest premise
in the whole index.**

Reading the configuration answers the wrong question. On a WSL2 kernel 6.18,
recorded in `talaria0101/malaria` `docs/windows.md` under "bailey on Windows,
and inside WSL":

- `/sys/kernel/security/lsm` listed `capability,landlock,yama,safesetid,selinux`;
- ⛔ a Landlock version query answered `EINVAL`;
- no landlock directory appeared under securityfs.

bailey reaches the same conclusion from the other side, in
`docs/reference/kernel.md` under "Distribution notes": it probes by attempting
the real setup in a throwaway child rather than reading the sysctl, because
AppArmor can block the `unshare` even where the sysctl permits it.

⚠ Neither measurement was taken by this project, and this session had no host to
check either against.

### Approach

`box-sandbox::probe`, and `agent-box doctor` prints it.

⛔ **Attempt, never read.** For each capability, fork a throwaway child that
performs the real operation and report what the child got.

| capability | the attempt |
| --- | --- |
| user namespaces | `unshare(CLONE_NEWUSER)` in a child |
| a network namespace | `unshare(CLONE_NEWNET)` in that child |
| Landlock, and its ABI | the version query, and ⛔ **the number it returns**, not the LSM list |
| cgroup v2 delegation | create a child cgroup under the named root and remove it |
| seccomp | install a trivial filter |
| the backend binary | run it and read the version |

⛔ **Two refusals are not negotiable:**

1. **the backend version floor.** bailey below 0.1.4 is refused. Below 0.1.3
   carries a seccomp bypass (`51a30ab`, `587fc6f`); 0.1.2 and 0.1.3 carry a
   working-directory regression (bailey issue 6, fixed by `b8a7eea`);
2. ⭐ **Landlock ABI 4, for a networked session.** Below Linux 6.7 the network
   rules are not applied **and the configuration is still valid**: the tool
   negotiates down rather than failing. `bailey show` prints the policy as
   written and does not say what the kernel will do with it. That other project
   made its daemon refuse in that case and this one does the same.

The report separates `gaps` from `notes`, and ⛔ **a gap stops the daemon**
unless `require_full_enforcement = false` is set. The reason goes in the startup
output, in the reference's own words: presenting a weaker boundary as if it were
a stronger one is worse than the weaker boundary, because it takes away the
chance to decide about it.

⭐ **Parse the backend's own report; do not model it.** errand does this at
`src/sandbox/bailey.ts:100-131` (`parseDoctor`), and the reason is that a model
of another tool's capabilities goes stale silently.

### Prove

```bash
cargo test -p box-sandbox probe:: && cargo run -p box-cli -- doctor
```

Passing: `cargo test` exits 0 with named tests for a fixture reporting no cgroup
delegation (one gap, and nothing else), a fixture reporting Landlock ABI 3 with
a networked configuration (⛔ refused, not a gap), and a version below the floor
(refused, naming the version). `doctor` exits 0 and prints every attempted
capability with its measured answer on this host.

⚠ **This entry cannot close on a claim.** Its closing records the actual
`doctor` output from the machine it ran on, with the kernel version, per
[`../docs/methodology/experiments.md`](../docs/methodology/experiments.md).

### Closing

Not closed.

---

## T-012: Per-session policy generation

**Source:** `QaidVoid/errand` `src/sandbox/policy.ts` (`58a178b`), and errand issue 1 (closed) for the certificate paths.
**Category:** sandbox
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-010

---

### Problem

A session needs the system paths that make a toolchain work, its own project,
its own state, and nothing else. Getting the list wrong in one direction breaks
every session; getting it wrong in the other grants the home directory and
confines nothing interesting.

### Premise

READ, not measured, except for one part that a closed tracker item settled.

errand's list is at `src/sandbox/policy.ts`:

| what | lines |
| --- | --- |
| profile names and filenames | 22-32 |
| `SYSTEM_READ`, with a comment per group saying what breaks without it | 34-69 |
| `SYSTEM_EXECUTE` | 71 |
| `RESOLV_CONF` and why the host's is not used | 74-93 |
| `policyPath` | 105-108 |
| `policyContents` | 110-220 |

⭐ **The certificate paths are the measured part.** errand issue 1, closed, is a
TLS failure on Arch because `/etc/ca-certificates` was not granted. The
maintainer's comment on 2026-09-14 records a correction to the reporter's own
diagnosis: it is not Arch-family only, openSUSE fails the same way through
`/var/lib/ca-certificates`, and nobody had hit it. Probed across images in that
comment:

```
archlinux     /etc/ssl/certs/ca-certificates.crt -> /etc/ca-certificates/extracted/tls-ca-bundle.pem
tumbleweed    /etc/ssl/ca-bundle.pem             -> /var/lib/ca-certificates/ca-bundle.pem
fedora        /etc/ssl/certs/ca-certificates.crt -> /etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem
alpine        /etc/ssl/certs/ca-certificates.crt  REAL FILE
```

⭐ **Both paths are present in `SYSTEM_READ` at the pinned commit**
(`src/sandbox/policy.ts:56-60`), checked by this project's sweep against the
code rather than believed from the item.

### Approach

In `box-policy`, generate a policy per session:

- **read**: `/usr`, `/lib`, `/lib64`, `/bin`, `/sbin`, `/proc`,
  `/etc/ld.so.cache`, `/etc/ld.so.conf`, `/etc/ld.so.conf.d`, `/etc/ssl`,
  `/etc/ca-certificates.conf`, `/etc/ca-certificates`, `/var/lib/ca-certificates`,
  `/etc/pki`, `/etc/nsswitch.conf`, `/etc/services`, `/etc/protocols`;
- **execute**: `/usr`, `/lib`, `/lib64`, `/bin`, `/sbin`;
- **write**: the project and the session's state directory, and nothing else.

⭐ **Name all five certificate paths with no per-distribution branch.** Naming a
path absent on a given host is already how the list works, and the maintainer's
comment says so explicitly: `/etc/pki` does not exist on the Gentoo box they
tested on, has been in every generated policy there, and those sessions run.

⛔ **Four rules, each with a stated reason:**

1. **the policy is written into the session's state directory, never the
   project.** The agent can write the project, so a policy there is a policy the
   agent rewrites, which is not a policy (`policy.ts:110-118`);
2. **no secret ever enters a policy.** Everything in one is a name or a path;
3. **paths are absolute.** After the pivot there is no working directory to
   resolve a relative one against;
4. **a synthetic `resolv.conf` replaces the host's.** The host's names the
   operator's ISP or private network. ⚠ One file for the daemon, not one per
   session: a copy inside a session's state directory would sit under a grant
   placed elsewhere and never be bound (`policy.ts:74-93`).

⛔ **There is no way to supply a whole policy file.** Additions are additive and
named; the floor is changed by changing this code. errand's reasoning, in
`docs/sandboxing.md`: a supplied file would let the report claim guarantees the
file does not make.

⭐ **The selection rule for every default here**, from `withastro/flue`'s
sandboxes guide: choose the narrowest environment that supports the task,
because expanding it expands what model-directed work can read, change, execute
and reach.

### Prove

```bash
cargo test -p box-policy
```

Passing: exit 0, with named tests asserting all five certificate paths reach the
generated policy (⭐ the regression errand's issue 1 produced), that the policy
path is under the state directory and not under the project, that a relative
path in an addition is refused, and that no field of the generated policy can
carry a value from the credential store (a type-level assertion, not a string
scan).

### Closing

Not closed.

---

## T-013: The bailey backend, locked to one broker

**Source:** `QaidVoid/bailey` `crates/bailey/src/cli.rs` and `backend/network.rs` (`d3c73f7`); `QaidVoid/errand` `src/sandbox/bailey.ts` (`58a178b`). Read [`../docs/history/references/bailey/usable.md`](../docs/history/references/bailey/usable.md) end to end first.
**Category:** sandbox
**Priority:** P1
**Effort:** M
**Status:** open
**Blocked by:** T-011, T-012, T-020

---

### Problem

On Linux a session should run as a confined host process using the host's own
tools, with exactly one route off the host: the broker.

### Premise

⭐ **READ, and the most important premise in this index is a trap rather than a
mechanism.**

`crates/bailey/src/backend/network.rs:45-59`: the network namespace is chosen
only where the policy needs **no** connectivity at all, stated in code as
`egress == DenyAll` and no bound ports. Anything else falls back to
`LandlockOnly`, which is TCP connect and bind and nothing else.

`crates/bailey/src/profiles/ai-agent.toml`, in its own header, without hedging:

> The cost of allowing egress at all: a partial allowance gives up the network
> namespace, so the agent shares the host's, where UDP, DNS and the host's
> loopback are reachable and only TCP ports are enforced. There is no way around
> that today for a program that must call an API.

⛔ **An agent that calls a model API is exactly that case.** So the obvious
configuration produces a session that can send UDP anywhere and reach every
service on the host's loopback.

**What closes it**, from `CHANGELOG.md` for 0.1.4, released 2026-09-14:

- `feat(cli): Add --egress-proxy to lock a session to a single broker` (`9656f22`);
- `fix(cli): Drop the namespace's forwarded loopback to the host` (`8570d3c`).

`crates/bailey/src/cli.rs:578-606` is the part that matters: passing
`--egress-proxy` makes the run **strict**, refusing without `pasta` on PATH and
refusing without user namespaces. ⭐ **It fails closed rather than degrading.**

⚠ **That release is four days old at the pinned commit. Treat it as load-tested
by nobody**, which is why this entry's acceptance is a measurement.

### Approach

In `box-sandbox::bailey`:

- build the argv. errand's is at `src/sandbox/bailey.ts:233-268` (`baileyArgs`);
- ⛔ **always pass `--egress-proxy ADDR:PORT`**, pointing at the broker. There is
  no mode that does not;
- ⛔ **never pass `--no-isolate`.** Without the isolation layer a `deny` nested
  inside a granted path is not enforced, because Landlock rights only add;
- the address is link-local and routes nowhere else. errand uses `169.254.169.1`
  (`src/sandbox/bailey.ts:133`) and sets `NO_PROXY` to the same address
  (`:352-353`) so a client does not try to proxy its own call to the broker;
- read the version and refuse below 0.1.4 (T-011 owns the check; this entry
  consumes it);
- parse the tool's own report rather than modelling it. errand's parser is
  `parseDoctor` at `src/sandbox/bailey.ts:100-131`;
- the environment is built, never inherited. errand's inherited list is exactly
  four names plus the cgroup root, at `src/sandbox/bailey.ts:50`.

⛔ **What it must not do:** allow an operator to supply arbitrary bailey
arguments, disable isolation, or run without `--egress-proxy`.

### Decision

**What happens on a host where `--egress-proxy` cannot be used** (no `pasta`, or
no user namespaces).

⭐ **Recommendation: refuse, with the reason and the two remedies.** The
alternative is the port-only mode, which the profile above documents as leaving
UDP and the host's loopback open, and this project's whole claim is that a
session has one route off the host. A boundary that quietly becomes a different
boundary is the failure
[`../docs/sandbox-model.md`](../docs/sandbox-model.md) opens with.

### Prove

```bash
cargo test -p box-sandbox bailey:: && cargo xtask battery --expect docs/history/references/windows/expected-linux.json
```

Passing: `cargo test` exits 0 with named tests for the argv (⭐ asserting
`--egress-proxy` present and `--no-isolate` absent), a version below the floor
refused, and a probe fixture without `pasta` refused rather than degraded.

⛔ **And the battery is the real acceptance**, from outside the sandbox: a
listener on the host, and a session that cannot reach it on TCP, UDP, or by
name; a session that can reach the broker; and a UDP packet to an external
address that does not arrive. ⚠ `cargo xtask battery` does not exist until
T-064, so this entry **stays partial** until it does.

### Closing

Not closed.

---

## T-014: The podman backend on Linux

**Source:** `QaidVoid/errand` `src/sandbox/podman.ts` (`58a178b`).
**Category:** sandbox
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-011, T-012, T-020

---

### Problem

An operator who would rather a session saw a filesystem they assembled than the
host's needs a container backend, and on Windows it is the only option.

### Premise

READ, not measured. errand's is 257 lines at `src/sandbox/podman.ts`:

| what | lines |
| --- | --- |
| `RESTRICTED_NETWORK` | 35-49, ⚠ with the measurement conditions in the comment: podman 5.8.2, pasta 2025.12.15 |
| `FORBIDDEN_ARGS` | 50-61 |
| the runner seam | 62-80 |
| `podmanArgs` | 81-148 |
| `PodmanSandbox` | 149-257 |

The network string is
`pasta:--map-host-loopback,none,--map-guest-addr,none`, and the comment records
that podman's default maps the host into the container and these two flags stop
it.

⚠ **A version-pinned claim about two moving projects, measured once, by somebody
else.** T-064 re-measures it here.

### Approach

In `box-sandbox::podman`:

- one container per session, named from the session id, labelled so orphan
  discovery can find it (T-032);
- `--cap-drop=ALL`, `--security-opt=no-new-privileges`, `--read-only` with a
  tmpfs at `/tmp`, `--userns=keep-id`, and the memory, cpu and process limits;
- ⛔ **the restricted network string, and an explicit route to the broker**;
- ⛔ **a refusal list of flags an operator cannot add**, and ⚠ **an unknown flag
  is refused rather than passed through**. A denylist over a surface that grows
  with every podman release fails open by construction; an allowlist does not.
  errand's list at `:50-61` is the starting content, not the shape.

⛔ **What it must not do:** accept an image from anywhere but the operator's
configuration, mount anything the policy did not name, or assume a flag took
effect because the command exited 0. See T-015 for the measured case of exactly
that.

### Prove

```bash
cargo test -p box-sandbox podman::
```

Passing: exit 0, with named tests for the argv carrying every flag above, an
operator-supplied forbidden flag refused, an operator-supplied unknown flag
refused, and the label present so T-032 can find the container.

### Closing

Not closed.

---

## T-015: Podman in a machine, and the four Windows host checks

**Source:** `talaria0101/malaria` `docs/windows.md` (`499e897`). Read [`../docs/history/references/windows/usable.md`](../docs/history/references/windows/usable.md) first.
**Category:** sandbox
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-014

---

### Problem

On Windows the daemon talks to a podman client that relays to a machine, which
is a WSL2 distro. Every flag is interpreted inside the machine, and two things
that hold on Linux do not hold there.

### Premise

MEASURED by another project, once, on one GitHub hosted runner: `windows-latest`,
podman 6.1.1, a machine on kernel 6.18. ⛔ **Not measured here, and one run on
one image is not a property of Windows.**

⭐ **The finding that changes the design:** the restricted flags close the
machine, not Windows. In the machine context the "host" pasta protects is the
machine itself; the Windows host is one route beyond, across the WSL NAT, and
what crosses is decided by the Hyper-V firewall for WSL, which is a Windows
policy and invisible from inside.

Measured, on a stock image:

| flags | internet and DNS | the machine's mapped address | the Windows host | `host.containers.internal` |
| --- | --- | --- | --- | --- |
| the restricted flags | reachable | unreachable | unreachable | unreachable |
| podman's default pasta | reachable | answering, so the mapping is alive | unreachable | - |

⚠ A `Set-NetFirewallHyperVVMSetting` change reopens the path and nothing inside
can see it.

Three more measured facts: `:Z` is accepted and does nothing because there is no
SELinux to relabel with; inotify events and permission bits do not cross the
drive mount; one fixed write ran 69 MB/s through the drive mount against
767 MB/s on the machine's own filesystem.

And one measured non-problem: WSL's Windows interop is a kernel-wide
`binfmt_misc` handler, so a container's exec of a PE binary **reaches** it; the
exec failed inside the container, exit 1, no Windows process, the container's
shell carried on. ⭐ Reachable, and fails closed.

### Approach

In `box-sandbox::podman`, a machine-context path:

⛔ **Four host checks, and each refuses or reports:**

| check | on failure |
| --- | --- |
| pasta supports `--map-host-loopback` | ⛔ **refuse.** Ubuntu 24.04's does not, so the restricted flags will not start. |
| a delegated cgroup exists (`systemd=true` in `/etc/wsl.conf`) | report as a gap: per-session limits are not applied |
| ⛔ the Hyper-V firewall posture | **report as a gap, never as enforced.** It cannot be read from inside. Print the check an operator runs. |
| `:Z` actually relabelled | ⛔ **assert the effect, not the exit code.** A backend that assumes relabelling because the command succeeded is making a claim it did not check. |

⛔ **bailey is refused on a Windows host, with the reason**, and that is correct
rather than a gap: its own roadmap lists non-Linux platforms as not planned.

The startup report names the machine context and, on a networked session, states
the firewall gap.

### Decision

**Which arrangement is documented as the recommended one.**

⭐ **Recommendation: the daemon inside WSL.** A project root in the distro's own
filesystem behaves as it does on Linux, the drive-mount penalties disappear, and
the only Windows involvement is the terminal. The daemon on Windows is supported
and carries three documented limits, and T-063 writes them where an operator
reaches for them.

### Prove

```bash
cargo test -p box-sandbox podman::machine::
```

Passing: exit 0, with named tests for a pasta version without
`--map-host-loopback` refused, a machine context reporting the firewall gap in
its `gaps` list, and a `:Z` mount whose effect is asserted rather than assumed.

⛔ **And one acceptance a unit test cannot give**: the reachability table above,
re-measured on a Windows host by T-064's battery, with this project's own
numbers and its own conditions. ⚠ This entry **stays partial** until that runs,
because everything above is somebody else's measurement.

### Closing

Not closed.

---

## T-016: Interactive devices, so a terminal program works

**Source:** the operator, 2026-09-15.
**Category:** sandbox
**Priority:** P2
**Effort:** S
**Status:** open
**Blocked by:** T-012

---

### Problem

A session that cannot allocate a terminal cannot run an interactive program,
and a surprising number of ordinary tools behave differently or refuse without
one: pagers, progress output, some test runners, anything that asks a question.

### Premise

READ, not measured. ⛔ **Nothing here was tested on any host**, and the device
list below is what such programs conventionally need rather than what this
project has observed them needing.

| device | needed for |
| --- | --- |
| `/dev/ptmx` plus a `/dev/pts` mount | allocating a pseudo-terminal |
| `/dev/fd`, a symlink to `/proc/self/fd` | process substitution, and tools that pass descriptors by path |
| `/dev/null`, `/dev/zero`, `/dev/full`, `/dev/random`, `/dev/urandom`, `/dev/tty` | ordinary operation |

⭐ **These are cheap to grant and the reason is specific**: `devpts` mounted
with a new instance gives the session its own terminal namespace, so a terminal
it allocates is not the host's and not another session's.

⚠ **They are not free.** [`../docs/sandbox-model.md`](../docs/sandbox-model.md)
already says every grant is a decision and device grants are the expensive ones.
These are the cheap end of that scale, and a device that reaches a driver is not
on this list.

### Approach

- mount `devpts` with a new instance and its own pseudo-terminal multiplexer,
  so the session's terminals are its own;
- place the character devices above, and ⛔ **nothing else**;
- `/dev/fd` as a symlink to `/proc/self/fd`, which works because T-031 already
  gives the session a fresh procfs in its own process namespace;
- ⛔ **no device that reaches hardware.** No graphics device, no sound, no raw
  block device, no kernel-module interface. An entry that wants one argues for
  it on its own.

⛔ **What it must not do:** grant the host's `/dev`, or a device wildcard.

### Prove

```bash
cargo test -p box-policy devices:: && cargo xtask battery devices --expect tests/expected/devices.json
```

Passing: `cargo test` exits 0 asserting the generated policy carries exactly the
listed devices and no wildcard. ⛔ **And the battery is the acceptance**: inside
a session, allocate a terminal and run an interactive program to completion;
from outside, assert the session's terminal is not visible in the host's
terminal namespace, and that no device outside the list is openable.

⚠ `cargo xtask battery` is T-064.

### Closing

Not closed.

---

## T-017: Measure whether a session can run containers, then decide

**Source:** the operator, 2026-09-15 ("measure it first and then decide").
**Category:** sandbox
**Priority:** P2
**Effort:** L
**Status:** open
**Blocked by:** T-013, T-014, T-064

---

### Problem

Many projects build and test in containers. A session that cannot run one
cannot do that work. ⛔ **And the thing that makes a container work is the thing
the sandbox removes**, so this is not a grant to add; it is a question to
answer.

### Premise

⭐ **RULED by the operator, 2026-09-15: measure it first, then decide.** So this
entry produces a measurement and a recommendation, ⛔ **not an implementation.**

READ, and the conflict is concrete. Rootless containers need unprivileged user
namespaces **inside** the session. The Linux backend's syscall filter denies the
namespace-creation call: its own documentation lists "cannot load kernel
modules, trace processes, or manipulate namespaces" as enforced by that filter.
⚠ Its network module says the same from the other side: a run inside a sandbox
cannot create a namespace of its own, because the outer filter denies it, so
what it has is whatever it inherited.

⚠ **The container backend may differ** and that is exactly what is unmeasured.
Nested rootless containers inside a rootless container is a configuration with
known requirements and this project has tested none of them.

### Approach

⛔ **Measure, write it down, recommend. Add nothing.**

For each backend, and on each platform the project supports, establish:

1. **does it work at all**, unmodified;
2. **what would have to be relaxed** to make it work, named exactly: which
   syscalls, which namespaces, which mounts, which device;
3. ⭐ **what each relaxation costs**, stated as what a hostile session could then
   do that it currently cannot;
4. **whether the alternative is cheaper**: the daemon runs the build in a
   separate sandbox the agent cannot reach, and hands back the result. ⚠ That is
   more work in the daemon and it keeps the boundary intact.

The output is a page under `docs/` with the measurements and their conditions,
and a recommendation with the rejected alternatives recorded.

⛔ **What it must not do:** relax anything. ⚠ **A relaxation shipped before its
cost is written down is a boundary nobody decided to move**, and
[`../docs/sandbox-model.md`](../docs/sandbox-model.md) opens with why that is
worse than not having the boundary.

### Decision

Ruled at the end of this entry, not at the start, and it carries the
measurement. Three routes are already visible and each is evaluated:

| route | cost |
| --- | --- |
| relax the filter and allow nested rootless containers | ⛔ the session can create namespaces, which is a large piece of what the filter exists to remove |
| a brokered build service: the daemon builds in a separate sandbox | more daemon, and the boundary holds |
| neither: sessions do not build containers | narrowest, and it excludes a class of project |

### Prove

```bash
cargo xtask battery nested --expect tests/expected/nested.json
```

Passing: exit 0, and ⛔ **this entry closes with the actual output**: the
machine, the kernel, the backend versions, the date, and what happened on each
attempt. ⭐ **A negative result closes it just as well as a positive one**, and
[`../docs/methodology/experiments.md`](../docs/methodology/experiments.md) says
a negative result is committed rather than discarded.

### Closing

Not closed.
