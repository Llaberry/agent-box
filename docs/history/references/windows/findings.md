# Windows: findings

`talaria0101/malaria` at `499e897e8372a5db0f921542184e3b66ef610e2b`: a fork of
errand carrying the only measured Windows work in any reference here.

[`usable.md`](usable.md) is the same sweep for the session doing the work.

---

## ⛔ What this sweep did NOT establish

- ⛔ **Nothing here was measured by this session.** Every number and every
  reachability result below is read from that project's committed results, and
  this session has no Windows host, no WSL2 distro and no podman machine to
  check any of it against.
- **The battery was not re-run.** `experiments/windows/battery.ps1` and
  `experiments/results/` are in the corpus and were listed, not executed.
- **The tracker is empty**, so there is no maintainer ruling, no reported
  defect and no confirmation from anybody who used it. ⚠ A sweep of a
  reference with no tracker is missing the source that most often corrects the
  code.
- **The kernel-config claims were not re-derived.** They are read from that
  project's `research/wsl-kernel.md`, which itself reads Microsoft's published
  configs.
- ⛔ **This is revision 1. Assume more claims are wrong than have been found.**

## ⚠ The claims here that are weakest

1. ⛔ **The conditions behind every measurement are one run, on one GitHub
   hosted runner, on one day.** `windows-latest`, podman 6.1.1, a machine on
   kernel 6.18. A number from one host is not a property of Windows.
2. **The firewall result is the default posture of a stock image**, which is
   exactly the thing an operator changes.
3. **The eleven-times throughput ratio is one fixed write**, with no sample
   count and no variance given.

---

## ⭐ The finding that decides the Windows design

**bailey is Linux and stays Linux.** `docs/windows.md`:

> It is a Linux tool and stays one: on a Windows host the probe refuses it with
> the reason, and there is nothing to fix, because the backend that works there
> is podman.

bailey's own roadmap agrees under "Not planned": non-Linux platforms.

⭐ **So Windows support is not a port of the Linux backend. It is the container
backend, one hop further out, and the hop is where the surprises are.**

The two arrangements that reference names:

| arrangement | what it costs |
| --- | --- |
| **the daemon on Windows**, containers in WSL2 below it | project roots on `C:\` work through the WSL drive mount, which does not deliver inotify events and does not enforce permission bits. ⚠ Tools inside the session that rely on either, such as watch modes and some test runners, misbehave in ways no container flag can fix. |
| **the daemon inside WSL** | a project root in the distro's own filesystem behaves as it does on Linux. ⭐ Simpler and faster, and the only Windows involvement is the terminal. |

---

## ⛔ The containment claim that changes meaning across the hop

This is the finding worth the whole sweep.

On a Linux host, `pasta:--map-host-loopback,none,--map-guest-addr,none` stops
pasta mapping any host address into the container, which closes the path to
services bound on the host. **In a podman machine, the "host" pasta protects is
the machine, not Windows.** The Windows host is one route further out, across
the WSL NAT, and what crosses that NAT is decided by the Hyper-V firewall for
WSL, which is a policy of the Windows host and invisible from inside.

⭐ **The reference's answer is to report it as a gap rather than guess**, and
that is the transferable decision. The measured result on a stock Windows
Server 2025 runner:

| flags | internet and DNS | the machine's mapped address | the Windows host | `host.containers.internal` |
| --- | --- | --- | --- | --- |
| the restricted flags | reachable | unreachable | unreachable | unreachable |
| podman's default pasta | reachable | answering, connection refused, so the mapping is alive | unreachable | - |

⚠ **"Unreachable on a stock host" is not "cannot be reached".** The page says
so plainly: a `Set-NetFirewallHyperVVMSetting` change reopens the path and the
daemon has no way to see that from inside. ⛔ **That is why the gap stays in the
report even though the measurement came back clean.** A measurement of a
default is not a guarantee about a configuration.

---

## The other measured results

All from the same run, and all subject to the conditions above.

| what | result |
| --- | --- |
| `--cap-drop=ALL` | the container reports zero effective capabilities |
| `--read-only` | writes to the root filesystem refused; the tmpfs at `/tmp` stays writable |
| `--memory`, `--cpus`, `--pids-limit` | arrive in the container's own cgroup (512m, 1.5 cores, 64 processes in the probe) |
| a container killed by its memory limit | relays exit code 137, so the resource diagnosis reads the same as on Linux |
| the RPC channel | a line piped through the Windows client into a container and back came out intact |
| `:Z` volume relabelling | ⚠ **not applied**: the podman machine has no SELinux to relabel with, so the suffix is plain `rw`, which the client accepted without complaint |
| files the daemon writes into the mount | arrive executable inside the container, because the drive mount maps them 0777. A `chmod 0755` is a formality there. |
| a fixed write through the drive mount against the machine's own filesystem | 69 MB/s against 767 MB/s, about eleven times slower |
| Windows interop from inside a container | WSL's interop is a binfmt_misc handler registered kernel-wide, so a container's exec of a PE binary reaches it. ⭐ **Measured: the exec failed inside the container, exit 1, no Windows process, the container's shell carried on.** The path is reachable and fails closed. |

⚠ **The `:Z` row is a silent difference, not an error.** The flag was accepted
and did nothing. A backend that assumes relabelling happened because the
command succeeded is a backend making a claim it did not check.

---

## ⭐ The Landlock result that says why a probe must attempt rather than read

`docs/windows.md`, on bailey inside WSL:

- Microsoft's kernel branches ship Landlock and list it in the LSM set, and the
  machine the battery booted ran kernel 6.18 with an LSM list of
  `capability,landlock,yama,safesetid,selinux`;
- ⛔ **the same kernel answered a Landlock version query with `EINVAL` and
  published no landlock directory under securityfs.**

So reading the config and reading the LSM list both say yes, and the feature is
not usable. ⭐ **That is the measured case for bailey's own rule** (probe by
attempting the real setup in a throwaway child), arrived at independently by a
second project, which is the strongest kind of confirmation available here.

⚠ **And the one check the tool's own report cannot self-certify**: network
rules need Landlock ABI 4, the tool negotiates down instead of failing, so a
networked session on a kernel below 6.7 runs with its egress policy silently
skipped. That reference made the daemon refuse in that case.
[`../bailey/usable.md`](../bailey/usable.md) carries the same conclusion from
the other side.

---

## The two host requirements nobody would guess

1. **Per-session limits inside WSL want `systemd=true` in `/etc/wsl.conf`**, so
   there is a cgroup subtree to delegate. Without it the daemon reports the same
   gap it reports on any Linux host without delegation.
2. ⛔ **Ubuntu 24.04's pasta predates `--map-host-loopback`**, so the restricted
   network flags refuse to start there. The podman machine's own Fedora image is
   new enough. ⚠ **A distribution choice is a security-relevant decision here**,
   which is not where anybody looks for one.

---

## Verdicts

| subject | verdict | where it lands |
| --- | --- | --- |
| podman as the Windows backend, bailey refused there with the reason | **adopt** | T-015 |
| reporting the Hyper-V firewall as a gap rather than guessing | ⭐ **adopt** | T-015 |
| refusing a networked session on a kernel below Landlock ABI 4 | ⭐ **adopt** | T-011 |
| probing by attempting, because the LSM list lies | **confirms** bailey, independently | T-011 |
| checking that `:Z` did something rather than that the command succeeded | **adopt** | T-015 |
| a minimum pasta version, checked | **adopt** | T-015 |
| the two arrangements, documented with what each costs | **adopt** | T-063 |
| the drive-mount throughput ratio | ⚠ **honest limit**, one sample | T-063 |
| inotify and permission bits not crossing the drive mount | ⚠ **honest limit** | T-063 |
| the binfmt interop path | **confirms** it fails closed; recorded so nobody re-derives it | T-063 |
| the battery itself, as an instrument | ⭐ **adopt the shape** | T-064: this project owes an equivalent, in Rust, that takes an expected result and exits non-zero on a mismatch |
| that project's Deno and chat surface | **refused** | it is errand's, and this project replaces it |

---

## ⚠ What this reference does not have, and it matters

**No tracker, no issues, no pull requests, no comments.** Every other reference
in this sweep produced at least one correction from somebody who used it, and
two of them produced the most important finding in their whole sweep that way.
⛔ **Treat these numbers as unreviewed.** They are one author's measurements,
committed with their instrument, which is far better than an assertion and is
not the same as a result anybody has checked.
