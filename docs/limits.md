# limits.md

What this project does not do, and is not going to.

⛔ **Every item here is a property of the design or of a mechanism it builds
on.** None is pending work with a fix on the way. Where something is genuinely
undecided it says so and names the entry that carries it.

⚠ **Nothing here is measured yet**, because nothing is built yet. Each limit is
sourced to the reference that measured or reported it, and
[`history/references/pins.md`](history/references/pins.md) has the commits.

---

## ⭐ The one that matters most

**Brokering closes credential theft. It does not close exfiltration through an
authorized channel.**

An agent that never sees a token, in a sandbox with no route off the host except
one broker, can still push a private repository's contents to a public place, if
the broker is configured to let it reach that repository host at all. The
credential was never stolen. It was used, for exactly what it was granted.

⛔ **Do not describe this project as preventing exfiltration.** It prevents a
class of it and the honest sentence is the narrow one.

### What actually helps, and what is theatre

| approach | honest assessment |
| --- | --- |
| **narrowing the binding scope per task phase** | ⭐ real, and it is the strongest thing available. Somebody has to say what the phases are, and a wrong answer blocks legitimate work. T-028. |
| **an out-of-band approval for widening operations** | real. The operator is in the loop, which is both the point and the cost. |
| **short-lived credentials minted per execution** | real, and not solved anywhere. The furthest-along reference has it open as a feature request. T-028 builds it or records why not. |
| an allowlist of upstream hosts | real but coarse. It stops a session reaching a paste site; it does not stop a session pushing to a repository it is allowed to push to. |
| **egress content scanning** | ⛔ **theatre, for this threat.** The agent chooses the encoding. Anything a scanner matches, an agent can avoid. It is useful as telemetry about honest mistakes and useless against an adversary. |
| **asking a model whether a request looks like exfiltration** | ⛔ **theatre.** The thing being defended against is a model that has been prompt-injected, and the defence is a model reading the same untrusted input. |
| **masking secrets in output** | ⛔ **not a boundary**, and this is settled rather than arguable. It loses to trivial encoding. Keep it as damage control on honest logging, and never count it. |

**The general form of the problem is an authorization-execution lifetime
mismatch**: a session is granted what its whole task might need, and keeps it
for every turn including the one after an injection. Every reference read for
this project has it open.
[`history/references/orchestration/findings.md`](history/references/orchestration/findings.md).

---

## What the broker cannot reach

| limit | source |
| --- | --- |
| **a credential carried inside a protocol message after an HTTP upgrade.** Metadata substitution rewrites a request's path, query, headers and body before it is forwarded; frames after the upgrade are copied through. An upstream that authenticates in its first frame is not brokered. | agent-vault 209 |
| ⛔ **a client whose transport refuses the proxy, and falls back to one that works.** The broker is never reached and never has a chance to fail. ⚠ The reported case logged an error, fell back, and produced correct output. | agent-vault 194 |
| a certificate-pinned client. It refuses the interception rather than being brokered. | architecture of every reference; not separately measured |
| **anything that is not HTTP.** Repository access over SSH, raw TCP, and any protocol the broker does not terminate. Those are refused rather than passed, which is a decision and not an accident. | design, T-021 |

⭐ **The second row is the operationally dangerous one**, because a deployment
that measures "did the agent succeed" passes it. T-029 owes a detection.

---

## What the sandbox cannot do

| limit | source |
| --- | --- |
| ⛔ **a partial egress allowance drops the network namespace**, leaving TCP port rules only: UDP, QUIC, DNS, ICMP and the host's loopback are reachable. ⭐ The `--egress-proxy` mode is what avoids it, and it is the only supported configuration here. | bailey's own agent profile and `network.rs` |
| host and address rules in a Linux policy are advisory: the port is enforced and the host is ignored | bailey `docs/security/limitations.md` |
| a read grant still allows metadata changes: `chmod`, `chown`, `utimensat`, `setxattr`. seccomp cannot close it, because it matches syscall numbers rather than resolved paths. | the same page |
| ⚠ **a denied access is invisible.** There is no way to observe a Landlock denial without the kernel audit subsystem enabled at boot. A session that fails gives its own error and nothing more. | the same page |
| resource limits are skipped without a delegated cgroup, and the run says so rather than failing | the same page |
| every guarantee is the kernel's. A shared-kernel sandbox ends where a kernel defect begins. | T-091 records when that stops being enough |
| side channels are not addressed: timing, resource observation, and that family | bailey `docs/security/model.md` |
| **anything granted is granted.** A device grant reaches a driver. Every grant is a decision, and device grants are the expensive ones. | the same page |

---

## What is different on Windows

| limit | source |
| --- | --- |
| ⛔ **the containment flags close the podman machine, not Windows.** What crosses the WSL NAT is the Hyper-V firewall's decision, which is invisible from inside. Reported as a gap rather than guessed at. | the Windows sweep, measured once on a stock image |
| `:Z` relabelling is accepted and does nothing, because there is no SELinux in the machine | the same |
| inotify events and permission bits do not cross the drive mount. Watch modes and some test runners misbehave and no flag fixes it. | the same |
| drive-mount throughput was about eleven times slower than the machine's own filesystem, on one fixed write | the same, ⚠ one sample |
| the Linux confinement tool does not run on Windows, and is refused with the reason | bailey's roadmap: non-Linux platforms are not planned |

---

## What is out of scope

⛔ **These are decisions, not gaps.** Reopening one costs a session's budget, so
each says why.

| | why |
| --- | --- |
| a chat gateway, transcripts, a web interface | a separate concern from confinement and brokering. Recorded in T-090 so it is not rediscovered as missing. |
| a hosted control plane | the operator holds their own credentials. That is the requirement this project exists to satisfy. |
| warm pools, sandbox claims, templates, cluster orchestration | machinery for a scale this project does not have. One caller. |
| transparent interception, so anything launched is automatically confined | explicit launch only |
| a display server, and anything that follows from one | not a workload this project serves |
| upstreaming a patch to a dependency | [`methodology/vendoring.md`](methodology/vendoring.md). The topic is closed. |

---

## ⚠ What is genuinely undecided

⛔ **These are open, and they are recorded as open rather than as impossible.**

| question | entry |
| --- | --- |
| how a destination address is bound to an allowed name, without false denials under content delivery networks, split DNS and rotation | T-027 |
| short-lived per-execution credential minting, and whether an upstream supports it | T-028 |
| whether this project owns an agent loop or drives one | T-040 |
| what the extension language is, if there is one | T-070 |
| when a shared kernel stops being enough, and what this design looks like under a stronger isolation tier | T-091 |

⭐ **A constraint closes a route, not a question.**
[`methodology/sessions.md`](methodology/sessions.md) is the rule, and it is why
each row above names an entry rather than a conclusion.
