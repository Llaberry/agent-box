# Windows: usable

What to build, and what to re-measure first.
[`findings.md`](findings.md) is the reasoning and the conditions.

⛔ **Every number on this page is one run, on one GitHub hosted runner, on one
day.** Re-measure before relying on any of it.
[`../../../methodology/experiments.md`](../../../methodology/experiments.md) is
what turns a quoted number into a measured one.

---

## The shape

| | |
| --- | --- |
| the daemon | runs on Windows as a native binary, or inside WSL |
| the backend | **podman only**. ⛔ bailey refuses on a Windows host, with the reason, and that is correct rather than a gap to close. |
| where containers run | a podman machine, which is a WSL2 distro, rootless, as the machine's own user |
| what the Windows side decides | where the machine looks, and moving bytes |

⭐ **Every flag the daemon passes is interpreted inside the machine**, by the
podman and the pasta that live there, not by anything on Windows.

---

## ⛔ The four host checks, and what each refuses

The probe runs these and the daemon refuses rather than degrading.

| check | why | what it costs to skip |
| --- | --- | --- |
| **pasta supports `--map-host-loopback`** | Ubuntu 24.04's does not | the restricted network flags refuse to start, so the session either runs unrestricted or does not run. ⚠ **A distribution choice is a security decision here.** |
| **a delegated cgroup exists** (`systemd=true` in `/etc/wsl.conf`) | otherwise there is no subtree to create children in | per-session memory, cpu and process limits are not applied. The service definition's limits still hold, over the daemon and every session together. |
| **the Hyper-V firewall posture for WSL** | ⛔ **it cannot be read from inside** | see below. This one is reported, not enforced. |
| **`:Z` actually relabelled** | the podman machine has no SELinux, so the flag is accepted and does nothing | a backend that assumes relabelling because the command exited 0 is making a claim it did not check |

### The firewall gap, and why it stays in the report

The restricted flags close the machine, not Windows. The Windows host is one
route beyond, across the WSL NAT, and the Hyper-V firewall for WSL decides what
crosses. That is a policy of the Windows host and invisible from inside the
machine.

Measured once, on a stock Windows Server 2025 runner: with the restricted flags
the internet and DNS were reachable and the machine's mapped address, the
Windows host and `host.containers.internal` were all unreachable.

⛔ **Do not turn that into a guarantee.** A `Set-NetFirewallHyperVVMSetting`
change reopens the path and nothing inside can see it. ⭐ **The daemon reports
it as a gap and says how to check it**, and with the network set to none the
question disappears.

The check an operator runs is the shape to reimplement: a listener on Windows,
and an attempt to reach it from a container on the machine's network.
`experiments/windows/battery.ps1` in the corpus is that check.

---

## What the measurements say holds, and what does not

⚠ Conditions: one run, `windows-latest`, podman 6.1.1, machine kernel 6.18.

**Holds, and reads the same as on Linux:**

| | |
| --- | --- |
| `--cap-drop=ALL` | zero effective capabilities in the container |
| `--read-only` plus a tmpfs at `/tmp` | root filesystem writes refused, `/tmp` writable |
| `--memory`, `--cpus`, `--pids-limit` | arrive in the container's own cgroup |
| `--userns=keep-id` | mapped to the machine's own user, uid 1000 |
| an out-of-memory kill | exit code 137 reaches the daemon |
| the RPC channel over stdin and stdout | a line survived the round trip intact |

**Does not hold, and each one is a documented limit rather than a defect:**

| | |
| --- | --- |
| `:Z` relabelling | there is no SELinux to relabel with; the suffix is plain `rw` |
| inotify events across the drive mount | not delivered, so watch modes misbehave |
| permission bits across the drive mount | not enforced; files arrive 0777, and a `chmod 0755` is a formality |
| drive-mount throughput | 69 MB/s against 767 MB/s on the machine's own filesystem, one fixed write, about eleven times slower |

⭐ **The advice that follows from the last three: put the project root in the
machine's own filesystem, or run the daemon inside WSL.** Both are the same
answer to the same cause.

---

## ⛔ Probe by attempting, because the LSM list lies

The measured case, from inside a podman machine on kernel 6.18:

- `/sys/kernel/security/lsm` listed `capability,landlock,yama,safesetid,selinux`;
- a Landlock version query answered `EINVAL`;
- no landlock directory appeared under securityfs.

⭐ **Reading the config says yes and the feature is not usable.** So the probe
attempts the real setup in a throwaway child and believes the attempt.
[`../bailey/usable.md`](../bailey/usable.md) reaches the same conclusion from
bailey's own documentation, which is two projects arriving at it independently.

⚠ **And one check no tool's self-report can make**: network rules need Landlock
ABI 4 (Linux 6.7), and bailey negotiates down rather than failing. ⛔ **A
networked session on a kernel below 6.7 therefore runs with its egress policy
silently skipped.** The daemon refuses that case itself.

---

## The interop question, answered

WSL's Windows interop is a `binfmt_misc` handler: the kernel routes PE
executables to `/init`, which launches them on the Windows side. The handler is
registered kernel-wide, so a container's exec of a Windows binary **reaches
it**.

Measured: the exec failed inside the container, exit 1, no Windows process
started, the container's shell carried on.

⭐ **Reachable but fails closed.** Record it so nobody re-derives it, and ⚠ do
not treat "fails closed today" as "cannot succeed": the path exists, and what
closes it is the container's own environment rather than a rule anybody wrote.

---

## ⭐ The instrument this project owes

That reference committed its battery beside its results, which is why any of
this is checkable. This project owes an equivalent, and T-064 is the entry.

Three properties, from
[`../../../methodology/references.md`](../../../methodology/references.md):

1. ⭐ **It is an oracle**: it produces ground truth independently of the thing
   being measured. A reachability claim checked by asking the sandbox is not
   checked. Run the listener outside and see whether the packet arrives.
2. ⭐ **It takes an expected result and exits non-zero on a mismatch**, so the
   research artefact becomes a regression check the project keeps.
3. ⚠ **It carries a fixture**: a committed input with known contents, so a
   result means something without a live third party.

⛔ **And it says what measuring changed.** A probe that disables verification to
observe something has changed the thing it is observing, and that is a finding
rather than a footnote.

---

## The configuration spellings

Search order on Windows, which is the same idea with Windows paths:

1. `%APPDATA%\agent-box\config.json`
2. `%USERPROFILE%\.config\agent-box\config.json`
3. `%ProgramData%\agent-box\config.json`
4. `config.json` in the working directory

⚠ **The file holds nothing secret in this project**, unlike the reference's,
because credentials live in the broker's own store. ⛔ The store still gets the
same treatment: readable by the operator alone.

```powershell
icacls "$env:APPDATA\agent-box" /inheritance:r /grant:r "$env:USERNAME:(F)"
```

A scheduled task at logon is the Windows spelling of a service definition, run
as the operator rather than as an administrator, for the same reason the Linux
service definitions do: files an agent writes in a project should belong to the
person whose project it is.
