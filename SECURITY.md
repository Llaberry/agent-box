# Security

The threat model, who holds what, and the blast radius of each leak.

⚠ **There is no code yet.** This describes the design
[`docs/architecture.md`](docs/architecture.md) commits to, and nothing in it has
been measured. Entry T-064 is what changes that.

⛔ **Read [`docs/limits.md`](docs/limits.md) with this.** A threat model without
its limits is a claim.

---

## Reporting

This project has no users and no releases. Open an issue on the repository.

⚠ If you have found something in one of the projects this one reads
([`docs/history/references/pins.md`](docs/history/references/pins.md)), report
it to that project. This repository carries findings about them as citations
into their own trackers, and none of them is this project's to disclose.

---

## The threat

Code chosen at run time by a model, executing with the same user authority as
every legitimate tool in the session. ⛔ **It does not have to be malicious to be
a problem.** It has to be wrong, prompt-injected through a file or a web page it
read, fed adversarial tool output, or running a dependency somebody else
compromised.

### The attacker

An unprivileged program running as the operator, which may be actively hostile
and which **knows it might be sandboxed**. It is not already root and it does
not have a kernel exploit. ⛔ **If either of those is false, no unprivileged
sandbox helps.**

Tenants are untrusted and are not assumed to be mutually hostile nation-state
attackers. ⚠ That assumption is load-bearing for the choice of a shared kernel,
and entry T-091 is where it gets revisited.

---

## Who holds what

| | holds | blast radius if it leaks |
| --- | --- | --- |
| **the daemon** | every credential, the certificate authority's root key, the configuration | ⛔ everything. It is the trusted process and there is no inner boundary. |
| **the broker**, inside the daemon | the credential store in memory, and encrypted at rest | the same |
| **a session** | ⭐ **one nonce per credential**, worth only what the broker will do with it, and only while that session runs | the session's own bindings, for the session's own lifetime. ⛔ Not the credential, and nothing after teardown. |
| **the project directory** | whatever the agent wrote | ⛔ **untrusted, every byte.** The daemon never runs a tool against it. |
| **the session state directory** | the policy, the agent's home, the transcript | ⛔ untrusted, every byte |

⛔ **A nonce is a secret while its session runs.** It is compared in constant
time, never logged, and removed at teardown. A nonce that outlives its session
is a credential with no owner, which is why teardown removes three things and
not one (entry T-032).

---

## What is enforced, and by what

⚠ **Status is what the design commits to.** Nothing below is measured.
[`docs/sandbox-model.md`](docs/sandbox-model.md) has the per-backend detail.

| property | mechanism |
| --- | --- |
| cannot read or write ungranted paths | Landlock, or the container's filesystem |
| cannot see ungranted paths | mount namespace, or the image |
| cannot see or signal host processes | PID namespace |
| ⭐ cannot reach anything but the broker | the network namespace, plus a backend flag that refuses rather than degrading |
| cannot exceed memory, process and cpu limits | cgroup v2. ⚠ Best-effort: skipped without a delegated cgroup, and the run says so. |
| cannot read the operator's environment | the environment is built, never inherited |
| cannot supply its own policy | the policy lives outside the project, and a discovered configuration is not policy until accepted |
| a credential reaches only a peer that proved its identity | the eight-condition injection gate, [`docs/credential-brokering.md`](docs/credential-brokering.md) |

---

## ⛔ The two design defects this project exists to not repeat

Both were measured against a running deployment of the project this one
replaces, by a reporter, and both are in its own tracker.

**1. The orchestrator running a tool against agent-writable state.**

A daemon opened pull requests by running `git` on the host with the agent's
project directory as the working directory. Git executes `.git/hooks/pre-push`,
`credential.helper`, `core.hooksPath`, `core.fsmonitor`, textconv filters and
aliases out of the repository it is pointed at. ⭐ **Host code execution as the
daemon's user, with no sandbox escape involved**, triggerable by anyone who
could drive a session.

⛔ **The class is larger than git**: any tool that reads configuration out of its
working directory. Entry T-030 makes it a check rather than a rule.

**2. A trusted channel the untrusted party can write to.**

In the same deployment, `/proc/1/fd/1` was writable from inside the session,
which was the agent's standard output and therefore the daemon's trusted event
stream. **Forged events were accepted as genuine agent output** and reached the
transcript on disk. `/proc/<pid>/environ` was readable, exposing other
processes' environments.

Entry T-031 makes the channel unforgeable by construction and adds a sequence so
a forgery is detectable even where it is not preventable.

---

## ⛔ What this does not protect against

The full set is [`docs/limits.md`](docs/limits.md). The three that matter most:

1. ⭐ **Exfiltration through an authorized channel.** Brokering closes credential
   theft. An agent with legitimate repository write access can push a private
   tree somewhere public without ever seeing a token. ⛔ **Every reference read
   for this project has this open**, and content scanning and model-based intent
   checking are both theatre against it.
2. **A kernel defect.** Every guarantee on Linux is the kernel's.
3. **A client that bypasses the broker.** Its transport refuses the proxy and it
   falls back to one that works. ⚠ In this design the fallback fails too, which
   is safer and produces an opaque error rather than a diagnosable one. Entry
   T-029 owes the detection.

---

## Secrets, operationally

[`docs/security/secrets.md`](docs/security/secrets.md) is the rule.
⛔ **This repository is public**, so it applies to the tree as well as to a
deployment:

- a credential never enters the tree, a log, a commit message or a document. Not
  expired, not redacted-looking, not in an example, ⛔ **not in a test**;
- nothing that fingerprints a private system either: real hostnames, account
  identifiers, internal paths, the names of private projects;
- ⛔ **an agent never asks for a credential value.** The operator names what they
  hold; the agent says where it goes;
- one redaction helper, and it is the only way a secret-shaped value reaches
  output. ⚠ A redactor used in most places is a redactor that fails.

⛔ **If something got in: rotate first, tell the operator second, remove it
third.** A history rewrite is the operator's action and it is not the fix; it is
tidying after the fix, and it does not un-publish a value.
