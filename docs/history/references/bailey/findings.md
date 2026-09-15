# bailey: findings

`QaidVoid/bailey` at `d3c73f7d8fd5a9da8289151eb06a35124037fbea`: the Linux
process confinement this project drives rather than replaces.

[`usable.md`](usable.md) is the same sweep for the session doing the work.

---

## ⛔ What this sweep did NOT establish

- **Nothing was run.** No sandbox was established, no kernel feature was
  probed, no benchmark was reproduced. Every capability statement below is
  bailey's own published claim, read from its documentation and cross-checked
  against the code that implements it.
- **No kernel was tested.** Every ABI number, every "since" version and every
  fallback below is read from `docs/reference/kernel.md` and from
  `crates/bailey/src/backend/network.rs`. ⚠ This session's own host was not
  probed, so nothing here is a measurement of any machine.
- **The published overhead numbers were not reproduced.** `docs/reference/benchmarks.md`
  and `scripts/overhead.py` exist in the corpus and were not run.
- **The eBPF audit backend was not swept.** It is out of this project's scope.
- **Discussions were not fetched.** The tracker is six items.
- ⛔ **This is revision 1. Assume more claims are wrong than have been found.**

## ⚠ The claims here that are weakest

1. **The bailey version this project will actually drive is not decided**, and
   the versions differ in ways that matter. See the version trap below.
2. **"Landlock does not govern file metadata" is bailey's own finding**, stated
   with a measurement in its limitations page. It was not re-measured here.

---

## What it is, and why it stays

A Rust tool that confines a program as a **host process**: Landlock for the
filesystem, seccomp for the syscall surface, and user, PID, UTS and sometimes
network namespaces. No image to build, and the confined program uses the host's
own tools.

⭐ **The verdict on the whole project is `adopt`, as a dependency and not as
code to copy.** It is the mechanism this project's Linux backend drives. What
this sweep takes is the contract, the failure modes and the honesty discipline.

---

## ⭐ What it is honest about, which is the reason to trust it

`docs/security/limitations.md` opens with the sentence this project should
adopt as a rule:

> A sandbox that overstates itself is worse than one that does not exist,
> because you make different decisions.

Every item in that page is a property of the mechanism rather than pending
work, and the page says so at the bottom. The ones that shape this project:

| limit | what it means here |
| --- | --- |
| ⛔ **a partial egress allowance drops the network namespace** | allowing any outbound at all leaves only Landlock's TCP port rules; UDP, QUIC, DNS and ICMP are unrestricted and the host's loopback is reachable. ⭐ **An agent that must call a model API is exactly this case.** |
| **host and CIDR rules are advisory** | `egress_allow = [{ host = "...", port = 443 }]` enforces the port and **ignores the host**. bailey warns. A host allowlist therefore cannot live in the policy; it has to live in the broker. |
| **a read grant still allows metadata changes** | `chmod`, `chown`, `utimensat` and `setxattr` succeed on a read-granted path. Landlock governs content and directory shape, not mode, owner, times or extended attributes. seccomp cannot close it, because it matches syscall numbers and register values rather than resolved paths. |
| **taking access away needs the isolation layer** | a `deny` nested inside a granted path is enforced by covering the path over, not by Landlock, because Landlock rights only add. ⛔ `--no-isolate` gives that up. |
| **a denied access is not reported** | Landlock denies silently and the kernel's audit subsystem is not normally available. A program that fails under enforcement gives its own error and nothing more. |
| **the seccomp filter is a denylist** | a hardening layer, not the access control |
| **limits are skipped without a delegated cgroup** | and the run says so rather than failing |

⭐ **The `on_violation` hook was specified, investigated and dropped**
(`docs/roadmap.md`, "Decided against"), because on a kernel with the log
readable but audit disabled a deliberately triggered denial produced zero
records: the hook could never have fired. ⚠ **Removed rather than carried as
config that does nothing** is the discipline to copy, and it is the same rule
as this project's own forbidden pattern about a setting no code reads.

---

## ⛔ The trap: a partial egress allowance is the normal case here

This is the finding that decides this project's network design.

`crates/bailey/src/backend/network.rs:45-59`: the network namespace is chosen
only when the policy needs no connectivity at all, which the code states as
`egress == DenyAll` **and** no bound ports. Anything else falls back to
`LandlockOnly`, which is TCP ports and nothing more.

`crates/bailey/src/profiles/ai-agent.toml` says the same thing in the profile's
own header, without hedging:

> The cost of allowing egress at all: a partial allowance gives up the network
> namespace, so the agent shares the host's, where UDP, DNS and the host's
> loopback are reachable and only TCP ports are enforced. There is no way
> around that today for a program that must call an API.

⭐ **So a coding agent under bailey, configured the obvious way, can send UDP
anywhere and reach every service on the host's loopback.** That is not a defect
in bailey; it is what its own profile documents. It is a defect in any design
that reads "sandboxed" and stops there.

### What closes it, and it landed the day before this sweep

bailey 0.1.4, released 2026-09-14 (`CHANGELOG.md`), carries two changes that
together answer it:

- `feat(cli): Add --egress-proxy to lock a session to a single broker`
  (`9656f22`);
- `fix(cli): Drop the namespace's forwarded loopback to the host` (`8570d3c`).

`crates/bailey/src/cli.rs:578-606` is the part that matters: passing
`--egress-proxy` makes the run **strict**. Without `pasta` on PATH the run is
refused, and without user namespaces the run is refused. ⭐ **It fails closed
rather than degrading to the TCP-port-only mode**, which is exactly the
property the ai-agent profile lacks on its own.

⛔ **This is the mechanism this project's Linux backend is built on, and it is
four days old at the pinned commit.** Treat it as load-tested by nobody.

---

## ⚠ The version trap

Three facts, each from a different source, that only matter together:

1. **0.1.2 introduced relocated grants and broke the invocation working
   directory.** bailey issue 6, closed: `covered_by_grant` was taught to match
   the placement rather than the source, and those two questions are asked from
   opposite ends. Fixed in 0.1.4 by `b8a7eea`.
2. **The reporter's own note on that issue** records that errand needed no
   change, because the deployed bailey was 0.1.1, which predates relocated
   grants: the break appears only on 0.1.2 and later.
3. **0.1.3 fixed two enforcement defects**: an unvalidated architecture let a
   denied syscall slip past (`51a30ab`), and btrfs ioctls Landlock does not
   mediate were not denied (`587fc6f`).

⭐ **So the safe version is 0.1.4 or later and nothing earlier**, because
anything before 0.1.3 carries a seccomp bypass, and 0.1.2 and 0.1.3 carry the
working-directory regression.

⛔ **The probe has to read the version and refuse below the floor**, rather
than assuming whatever is installed. [`usable.md`](usable.md) carries it.

---

## ⭐ The discipline to copy wholesale

| practice | where | why |
| --- | --- | --- |
| **probe by attempting the real thing in a throwaway child** | `docs/reference/kernel.md`, "Distribution notes" | Debian and Ubuntu restrict user namespaces at various points and AppArmor can block `unshare` even when the sysctl permits it, so reading the sysctl answers the wrong question |
| **report what was not enforced, per run** | `docs/security/model.md`, the enforcement table | every row carries a status, and "Enforced on Linux 6.7+" is a different claim from "Enforced" |
| **defence in depth, stated as a reason rather than a slogan** | the same page, last section | "absence is a stronger property than refusal, and it is enforced by a different mechanism" |
| **a config found in a directory is not policy until accepted** | `docs/roadmap.md`, "Config you decided to apply" | ⭐ **a repository cannot ship the policy that is meant to contain it**, and the acceptance is recorded against the contents so an edit asks again |
| **output means something needs attention** | `crates/bailey/src/backend/network.rs:100-108` | a run that fully denies the network says nothing |
| **an inner run reads what the outer run published** | `network.rs:71-98` | a sandbox inside a sandbox cannot create a namespace, because the outer seccomp denies `unshare`. ⛔ Guessing is wrong in both directions, so the outer run publishes which mode it built. |

---

## Verdicts

| subject | verdict | where it lands |
| --- | --- | --- |
| bailey as the Linux confinement backend | **adopt**, as a dependency | T-013 |
| `--egress-proxy`, and its fail-closed strictness | ⭐ **adopt** | T-013, T-021 |
| the version floor of 0.1.4 | **adopt** | T-013 |
| probing by attempting the real setup | **adopt** | T-011 |
| the per-run report of what was not enforced | **adopt** | T-011 |
| "a config found in a directory is not policy until accepted" | **adopt** | T-014 |
| the published limitations page, as a model for this project's own | **adopt** | T-061 |
| a partial egress allowance leaving UDP and host loopback open | ⚠ **honest limit**, not a defect | recorded in T-013 and in the limits page |
| host and CIDR rules in the policy being advisory | ⚠ **honest limit** | it is why the host allowlist lives in the broker, T-021 |
| metadata writes on a read-granted path | ⚠ **honest limit** | T-062 |
| the eBPF audit backend, `bailey audit`, `bailey reconcile` | **filed elsewhere** | out of first scope; recorded so it is not rediscovered |
| non-Linux platforms | **refused** by bailey, explicitly | ⭐ which is why the Windows answer is a different backend entirely: [`../windows/findings.md`](../windows/findings.md) |
