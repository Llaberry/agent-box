# On Windows

The daemon runs inside WSL2, and the sandbox runs in containers below it.
This page says what that combination means and how to set it up.

## What runs where

- the daemon runs in a WSL2 distro, started by the distro's init or by hand;
- the `podman` it finds on PATH drives containers on the same kernel;
- project roots live in the distro's own filesystem, never on a drive mount.

A project root on `C:\` is served through the WSL drive mount, which is
slower than the distro filesystem and delivers no inotify events and no
permission enforcement. Tools that rely on either misbehave in ways no
container flag can fix. Keep project roots inside the distro.

## What the sandbox promises on this stack

The daemon probes the backend at startup and reports what it can enforce on
this host. On this stack the report says more, because two things differ.

**The isolated network closes the machine, not Windows.** The restricted
flags stop the container runtime from mapping any host address into the
container. In the machine context the host that is protected is the distro
itself. The Windows host is one route beyond it, across the WSL NAT, and
what crosses that NAT is decided by the Hyper-V firewall for WSL, which is
a policy of the Windows host and invisible from here. The daemon reports
this as a gap rather than guessing at the firewall. With
`sandbox.network` set to `none` the question disappears.

**Volumes are not relabelled.** The SELinux `:Z` suffix has nothing to
relabel against here, so mounts are plain `rw`. `--read-only`, the tmpfs
at `/tmp`, and `--userns=keep-id` hold as on Linux.

**bailey is refused here.** bailey needs Landlock ABI 4 for network rules
(Linux 6.7). The WSL kernel may report Landlock present while answering
ABI queries in ways the daemon cannot rely on, so the probe refuses bailey
inside WSL and podman is the backend. There is nothing to fix.

## Configuration

The search order on Windows is the same idea with Windows spellings:

1. `%APPDATA%\egi\config.json`
2. `%USERPROFILE%\.config\egi\config.json`
3. `config.json` in the working directory

`EGI_CONFIG` names one outright and skips the search. A token is in the
file, so keep it readable by you alone:

```powershell
icacls "$env:APPDATA\egi" /inheritance:r /grant:r "$env:USERNAME:(F)"
```

## The evidence

Measured by the malaria port of errand on a stock Windows Server 2025
runner, podman 6.1.1, kernel 6.18, and committed under that repository's
`experiments/results/`: with the restricted flags the machine hop closes
while internet and DNS keep working; `--cap-drop=ALL` reports zero
effective capabilities; `--read-only` refuses root writes while `/tmp`
stays writable; memory, cpu, and process limits arrive in the container's
own cgroup; an OOM relays exit code 137.
