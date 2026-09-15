# Docs

Pages this project owes, and the instrument that turns its claims into
measurements.

[`INDEX.md`](INDEX.md) is the list.

---

## T-060: Bind addresses, and refusing a public bind

**Source:** `TencentCloud/CubeSandbox` `docs/guide/network-hardening.md` (`eaddce3`).
**Category:** docs
**Priority:** P2
**Effort:** S
**Status:** open
**Blocked by:** T-003

---

### Problem

A management surface reachable from an untrusted network is the shortest route
to everything this project holds, and the default is where it gets decided.

### Premise

READ, not measured. That reference's own hardening guide tabulates its default
listening surface: **seven services defaulting to every interface**, of which the
cluster management API and the node management RPC are marked "no auth" and "no
TLS" respectively. Its own warning says the one-click deployments are for
development and evaluation.

⭐ **The interesting part is the second-order trap it documents**: binding the
management API to loopback **breaks** the compute nodes and the browser
interface, because each reaches it by address. So the safe default is not
loopback everywhere; it is loopback where nothing legitimately reaches across,
and a refusal where it would.

⚠ This project has one process and no cluster, so the trap does not apply. It is
recorded because the instinct it warns against is the same one.

### Approach

- ⛔ **every listener binds loopback by default**;
- ⛔ **a public bind is refused, not warned about.** The reference this project
  replaces already takes this position for its own interface, on the ground that
  the address it binds to is the access control;
- the broker's per-session listeners bind the link-local address the backend
  routes to, and ⛔ **nothing else on the host can reach them**, which is a
  property of the namespace rather than of the bind;
- `doctor` prints every listener and its bind address.

### Prove

```bash
cargo test -p box-cli bind::
```

Passing: exit 0, with named tests for a configured public bind refused naming
the address, a loopback bind accepted, and `doctor` output listing every
listener with its address.

### Closing

Not closed.

---

## T-061: Write the limits page from measurement rather than from reading

**Source:** `QaidVoid/bailey` `docs/security/limitations.md` (`d3c73f7`), as the model.
**Category:** docs
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-064

---

### Problem

⛔ **[`../docs/limits.md`](../docs/limits.md) exists and every row in it is read
rather than measured.** A limits page sourced entirely to other projects is a
limits page about other projects.

### Premise

READ. The model is that reference's own page, which opens with the standard this
project adopted:

> Every item here has been verified against the current implementation. A
> sandbox that overstates itself is worse than one that does not exist, because
> you make different decisions.

⛔ **This project cannot say that sentence yet**, and
[`../docs/limits.md`](../docs/limits.md) says so at the top rather than implying
otherwise.

### Approach

Once T-064's battery runs, go through [`../docs/limits.md`](../docs/limits.md)
row by row:

- a row the battery confirms gets this project's own measurement and conditions;
- a row the battery contradicts is ⭐ **a finding, and the finding is worth more
  than either source alone**. It goes in
  [`../docs/history/README.md`](../docs/history/README.md)'s withdrawn-claims
  table with what was believed and what was measured;
- a row the battery cannot reach stays sourced to its reference and **says that
  it is unmeasured here**;
- ⛔ **nothing is deleted for being inconvenient.** A limit is moved to history
  with the measurement that took it away, never dropped.

⭐ **And the closing states how many rows changed**, because that is the only
honest estimate of how many are still wrong.

### Prove

```bash
cargo xtask check docs && cargo xtask check record
```

Passing: both exit 0, with every row in [`../docs/limits.md`](../docs/limits.md)
carrying either a measurement with its conditions or an explicit note that it is
unmeasured here, and with the count of changed rows in this entry's closing.

### Closing

Not closed.

---

## T-062: Document the metadata-write limit where a user meets it

**Source:** `QaidVoid/bailey` `docs/security/limitations.md` (`d3c73f7`), "A read grant still allows a file's metadata to be changed".
**Category:** docs
**Priority:** P3
**Effort:** S
**Status:** open
**Blocked by:** T-013

---

### Problem

An operator granting a directory read expects read. `chmod`, `chown`,
`utimensat` and `setxattr` succeed on files inside it.

### Premise

MEASURED by the source, and its own statement of what it measured is precise:
on a read-granted path, creating, writing, deleting, linking and binding are all
refused, and those four calls return success.

⛔ **seccomp cannot close it**, and the reason is structural: it matches syscall
numbers and register values, not resolved paths, so it cannot tell a `chmod` on
a granted path from one outside, and a session that may not `chmod` inside its
own workspace is not a working sandbox. **The boundary belongs in Landlock,
which does not yet express it.**

⚠ Ordinary permissions still apply underneath, so this reaches only files the
invoking user already owns. It cannot change contents and cannot gain a
privilege; it can make a file unreadable to its owner or move its timestamps.

### Approach

- the row is in [`../docs/limits.md`](../docs/limits.md) already;
- add the operational consequence where an operator meets it: in the
  configuration reference for a read grant, ⭐ **"grant read narrowly: a
  directory granted for the sake of one file exposes the metadata of everything
  in it"**;
- the startup report calls out a read grant covering more than one file.

### Prove

```bash
cargo xtask check docs
```

Passing: exit 0, with the consequence present beside the read-grant
configuration and not only in the limits page, and with
`cargo xtask check one-home` still passing, which means it is **derived** there
rather than duplicated.

### Closing

Not closed.

---

## T-063: The Windows page

**Source:** `talaria0101/malaria` `docs/windows.md` (`499e897`). Read [`../docs/history/references/windows/usable.md`](../docs/history/references/windows/usable.md) first.
**Category:** docs
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-015

---

### Problem

An operator on Windows meets three behaviours that no flag fixes and one
containment claim that means something different from what it means on Linux.
Finding that out by debugging is expensive.

### Premise

MEASURED by another project, once, on one hosted runner. All of it is in
[`../docs/history/references/windows/usable.md`](../docs/history/references/windows/usable.md)
with its conditions.

### Approach

`docs/windows.md`, carrying:

- the two arrangements and what each costs, with ⭐ **the daemon inside WSL as
  the recommended one** (T-015's ruling);
- ⛔ **the containment claim's change of meaning**: the flags close the machine,
  not Windows, and the Hyper-V firewall decides the hop beyond and is invisible
  from inside. With the check an operator runs;
- the three documented limits: `:Z` accepted and doing nothing, inotify and
  permission bits not crossing the drive mount, and the throughput ratio ⚠ with
  its one-sample conditions stated;
- the two host requirements: `systemd=true` for a delegated cgroup, and a pasta
  new enough for `--map-host-loopback`;
- the configuration search order and the file permissions command;
- ⛔ **bailey refused on Windows, with the reason**, so nobody files it as a gap.

⛔ **Every number carries its conditions or it is not a number**, and every
number on that page is somebody else's until T-064 re-measures it. ⚠ **Say
which.**

### Prove

```bash
cargo xtask check docs && cargo xtask check markers
```

Passing: both exit 0, with the page linked from
[`../docs/README.md`](../docs/README.md) and from
[`../docs/sandbox-model.md`](../docs/sandbox-model.md), and with every
measurement on it carrying its date, its host and its versions.

### Closing

Not closed.

---

## T-064: The containment battery

**Source:** [`../docs/methodology/references.md`](../docs/methodology/references.md), "The instrument is the deliverable"; `talaria0101/malaria` `experiments/windows/battery.ps1` (`499e897`), as the shape.
**Category:** docs
**Priority:** P1
**Effort:** L
**Status:** open
**Blocked by:** T-013, T-014

---

### Problem

⛔ **Every containment claim this project makes is currently somebody else's
measurement on somebody else's machine.** A claim with no instrument is a claim
that decays the moment a version moves.

### Premise

READ, and the methodology states the three properties an instrument needs:

1. ⭐ **it is an oracle**: it produces ground truth independently of the thing
   being measured. A reachability claim checked by asking the sandbox is not
   checked;
2. ⭐ **it takes an expected result and exits non-zero on a mismatch**, so the
   research artefact becomes a regression check the project keeps;
3. ⚠ **it carries a fixture**: a committed input with known contents, so a
   result means something without a live third party.

And one warning: ⛔ **the instrument perturbs the measurement, and that is a
finding.** A worked example from that methodology: disabling certificate
verification so a probe could terminate a connection **also changed the client's
advertised algorithms**, so what the probe captured was not what the client
ships.

### Approach

`cargo xtask battery <suite> --expect <file>`, with suites:

| suite | asserts, from outside the sandbox |
| --- | --- |
| `reach` | ⭐ a listener on the host is unreachable from a session, on TCP, on UDP, and by name. A listener at the broker's address **is** reachable. A UDP packet to an external address does not arrive at a receiver outside. |
| `inject` | a request through the broker arrives at a recording upstream **with** the credential; a request that fails any gate condition arrives nowhere |
| `spoof` | the injection gate's refusals, each driven end to end: a foreign SNI with a matching `Host`, plaintext without the opt-in, an upstream certificate for a different name |
| `limits` | memory, process and cpu limits observed from outside; an out-of-memory kill reported as the expected exit |
| `dns-binding` | T-027's false-denial count against real upstreams over a stated period |

⛔ **The oracle is always outside the sandbox.** The listener, the receiver and
the recording upstream are the daemon's own, not the session's report.

⛔ **The fixture is committed**: a recording upstream with a known certificate
and known responses, so `inject` and `spoof` mean something with no third party.

⚠ **And the battery says what it changed.** If a suite has to weaken something
to observe it, that weakening is reported beside the result, and the result is
labelled as taken under it.

### Prove

```bash
cargo xtask battery reach --expect tests/expected/reach-linux.json
```

Passing: exit 0 where the measured result matches the expectation, non-zero with
a named difference where it does not. ⛔ **And mutation-proved**: an expectation
edited to claim the host listener is unreachable when it is reachable makes the
suite exit non-zero.

⛔ **This entry's closing carries the first real run**: the machine, the kernel,
the backend version, the date, and the full output. ⭐ **Every claim in
[`../docs/limits.md`](../docs/limits.md) and
[`../docs/sandbox-model.md`](../docs/sandbox-model.md) that this run touches is
updated in the same change**, and T-061 is what does the rest.

### Closing

Not closed.
