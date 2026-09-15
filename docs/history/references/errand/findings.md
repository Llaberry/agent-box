# errand: findings

What `QaidVoid/errand` at `58a178b2a03dee289bb535e1fdc58676344050eb` does, what
transfers to this project, and what must not.

[`usable.md`](usable.md) is the same sweep written for the session doing the
work. [`../pins.md`](../pins.md) is the commit and the depth.

---

## ⛔ What this sweep did NOT establish

- **Nothing here was run.** No daemon was started, no session was launched, no
  sandbox was created. Every statement below is read from source at the pinned
  commit, and the tracker items are read from their own text. ⚠ Where the two
  disagree, the disagreement is reported as the finding; it was not settled by
  measurement.
- **The reproduction in errand issue 2 was not re-run here.** It is reported as
  the reporter wrote it, with the code it names opened and confirmed to be the
  code it describes. Whether it still reproduces at the pinned commit is not
  established.
- **No platform was tested.** The Windows claims come from
  [`../windows/findings.md`](../windows/findings.md), which is a different
  reference and a different set of conditions.
- **Discussions were not fetched**, per [`../pins.md`](../pins.md). The tracker
  is three items, so a design argument that never became an issue is plausible
  and was not read.
- **This is revision 1.** No claim here has been corrected yet, which is not
  the same as no claim being wrong. ⛔ Assume more remain.

## ⚠ The claims here that are weakest

Read these before the recommendations.

1. **"The provider credential does not enter the sandbox" is conditional, and
   the condition is not always met.** It holds only where the model store on
   the host knows the provider's base URL. Where it does not, the code falls
   back to handing the session the real credential, with a warning
   (`src/serve.ts:224-228`). That branch was read, not exercised.
2. **The marker for "what errand holds" comes from two sources that
   disagree**, and the code is the one believed. See the contradiction below.
3. **Sizes and counts** are from `wc -l` over the corpus. They move with every
   commit and are given for shape, not as a measurement of anything.

---

## What it is

A daemon that runs a coding agent from a chat channel. One channel, one thread
per session, several sessions at once. The agent runs sandboxed under one of
two backends and there is no unsandboxed mode: the daemon refuses to start
rather than run a session unconfined (`README.md:21-27`).

35,471 lines of TypeScript on Deno, counted over the whole tree at the pinned
commit: `src/` is 26,552 and the separate web interface 5,087. ⭐ **The whole
sandbox surface is 3,034 lines across 14 files in `src/sandbox/`**, of which the
broker is 347.

⚠ **Line counts are a shape, not a measurement of anything.** They move with
every commit and some counters skip blank lines. What they establish is the
ratio, and the ratio is the point: **the part this project is about is under a
tenth of that tree.**

⭐ **The shape transfers. The runtime is what this project replaces.**

---

## ⛔ The finding that matters most: host code execution, with no escape needed

**errand issue 2, open**, titled "Critical: orchestrator's host-side git
executes repository hooks/credential helpers -> host code execution as the
daemon user".

The daemon opens pull requests by running `git` **on the host**, outside every
sandbox, with the agent's project directory as the working directory. Git reads
and executes several things out of a repository it is pointed at:

| what git executes | reached by |
| --- | --- |
| `.git/hooks/pre-push` | the daemon's own `git push --force-with-lease` |
| `.git/config` `credential.helper` | the push's authentication |
| `core.hooksPath`, `core.fsmonitor`, `diff.*.textconv`, `filter.*`, aliases | ordinary git operations |

The project directory is writable by the agent, so all of it is
attacker-controlled. The gate that decides a pull request was asked for is
itself a file in the agent's own writable state directory, so the agent can
trigger the whole path unaided.

The reporter's evidence is a host-side marker file written as the daemon's
user, and a daemon log line that proves the hook ran on the host because the
host shell could not find a path that exists only inside the sandbox.

⭐ **The verdict is `anti-pattern exhibit`, and it is the single most valuable
thing this sweep found.** Nothing in errand's own documentation says the daemon
runs git against an agent-writable tree, and no test could have caught it: the
sandbox was correct, every gate was correct, and the hole was in the
orchestrator's own use of a tool that executes configuration.

⚠ **It generalises past git.** The class is: **the orchestrator running any
tool against state the agent controls.** A linter with a config file, a package
manager with a lifecycle script, a formatter with a plugin path, a test runner
with a conftest. Every one of them executes something out of the directory.

### The secondary finding in the same item

The reporter also records, against the same daemon:

- `/proc/<pid>/environ` readable from inside the session, which exposes other
  processes' environment including secret values;
- `/proc/1/fd/1` **writable**, which is the agent process's stdout and therefore
  the daemon's trusted event stream. Forged events were accepted as genuine
  agent output and reached the transcript on disk;
- `/proc/1/fd/0` writable, which is injection toward the agent.

⛔ **The third of those is the one with no obvious fix in the sandbox.** A
trusted channel that the untrusted party can write to is not a channel, and
moving it out of `/proc` is a design change rather than a policy change.

---

## ⛔ A contradiction between the documentation and the code

⭐ **The disagreement is the finding**, and the code wins.

| source | says |
| --- | --- |
| `docs/sandboxing.md`, "What the agent holds" | "The provider credential, because it needs it, and a GitHub token when one is configured" |
| `src/sandbox/broker.ts:79-96` and `src/serve.ts:180-213` | the provider credential is replaced by a per-run random nonce, and the real value never leaves the daemon |

Both are true of different configurations, and the page states the weaker one
as though it were the only one. Reading the code settles it:

- `sandbox.egress.mode` **defaults to `proxy`** (`src/config/schema.ts:423`),
  so brokering is on unless an operator turns it off;
- under `proxy`, one 32-byte random nonce is minted per provider per daemon run
  (`src/serve.ts:102-106`) and substituted for the credential in the session's
  environment (`src/sandbox/bailey.ts:283-295`);
- the broker puts the real credential on at its own end
  (`src/sandbox/broker.ts:181`);
- ⚠ **but the default allowlist is `["*"]`** (`src/config/schema.ts:423`), and
  a lone `*` admits any host (`src/sandbox/broker.ts:34-36`). Out of the box the
  broker gates the port and logs every connection and restricts no host.

⛔ **The GitHub token has no equivalent and is not brokered at all.** It is put
into the sandbox environment directly (`src/session/session.ts:390-397`), and
`src/session/github.ts:1-14` says so plainly: "Treat the token as known to the
agent, and scope it on that basis." That is the honest half of the
documentation sentence, and it is the leak this project exists to close.

---

## ⭐ What it already gets right, and should be kept

| mechanism | where | why it transfers |
| --- | --- | --- |
| **a gap is always stated, and by default stops the daemon** | `docs/sandboxing.md`, "What it says at startup"; `src/sandbox/backend.ts:101-111` | "Presenting a weaker boundary as if it were a stronger one is worse than the weaker boundary itself, because it takes away the chance to decide about it." |
| **no unsandboxed mode, and no fallback between backends** | `src/sandbox/backend.ts:133-141` | a probe that cannot run the configured backend raises rather than quietly choosing another |
| **the agent is never shown a host path** | `src/sandbox/backend.ts:14-25` | a path carries the operator's name and the shape of their machine, and that is not access the sandbox can take back |
| **the policy lives outside the project** | `src/sandbox/policy.ts:110-118` | "The agent can write to its project, so a policy living there would be a policy the agent could rewrite, which is not a policy." |
| **the environment is built, never inherited** | `src/sandbox/bailey.ts:50` | only named variables cross |
| **no way to supply a whole policy file** | `docs/sandboxing.md`, "Granting more than the default" | a supplied file would let the startup report claim guarantees the file does not make |
| **a synthetic resolver instead of the host's** | `src/sandbox/policy.ts:74-93` | the host's `/etc/resolv.conf` names the operator's ISP or private network |
| **a constant-time comparison for the nonce** | `src/sandbox/broker.ts:112-119` | it is a secret, and an equality operator on a secret is a timing oracle |
| **the CONNECT head is read one byte at a time** | `src/sandbox/broker.ts:296-326` | reading past the head strips bytes the client expects to carry its TLS; reading only the request line leaves header bytes that corrupt the upstream handshake |

⭐ **That last one is worth its own line.** It is a defect that is invisible in
review, produces a corrupted connection rather than an error, and the comment
explaining it is the only thing standing between a reimplementation and the
same bug. [`usable.md`](usable.md) carries it as a rule.

---

## ⛔ What must not transfer

| do not carry over | why |
| --- | --- |
| **running any host-side tool with the agent's directory as its working directory** | the finding above. This is the design defect, not a bug in one function. |
| **`egress.allow` defaulting to `["*"]`** | the broker is then an audit pass-through. A default that restricts nothing is a default that reads as protection. |
| **falling back to handing the session the real credential** when the provider base URL is unknown (`src/serve.ts:224-228`) | a warning is not a boundary. Refuse to start instead. |
| **the GitHub token in the session environment** | `src/session/session.ts:390-397`. This is the whole reason for this project. |
| **a `gh` wrapper that refuses to open a pull request** | `src/session/github.ts:52-62` says it plainly: "It is not a wall, because the token can reach the API directly and the wrapper sits in a directory the session can write." A control the agent can delete is not a control. |
| **output-side secret scrubbing described as damage control** | `docs/sandboxing.md`, last paragraph, is honest that "an agent that re-encodes a key defeats it". Keep the honesty; do not keep it as though it were a boundary. |
| **the whole Deno and chat-service surface** | not a defect, just not this project. Measured at the pinned commit: `src/chat` 3,124, `src/session` 9,843, `src/web` 1,512, `src/cli` 439 and the separate `web/` interface 5,087, which is 20,005 of 35,471 lines, or **56 per cent**. |

---

## Verdicts

| subject | verdict | where it lands |
| --- | --- | --- |
| the backend trait and the capability report | **adopt** | T-010 |
| the CONNECT broker, the nonce substitution, the head reader | **adopt** | T-020, T-021 |
| the per-session generated policy, and where it lives | **adopt** | T-012 |
| the built environment and the synthetic resolver | **adopt** | T-012 |
| refusing to start on an unenforceable guarantee | **adopt** | T-011 |
| host-side git against an agent-writable tree | ⭐ **anti-pattern exhibit** | T-030 exists because of it |
| `/proc` exposure and a writable trusted event stream | ⭐ **anti-pattern exhibit** | T-031 |
| the GitHub token in the session environment | **anti-pattern exhibit** | T-022 |
| `egress.allow` defaulting to `*` | **anti-pattern exhibit** | T-021 |
| the chat gateway, threads, transcripts, web interface | **filed elsewhere** | out of this project's first scope; recorded in T-090 so it is not rediscovered as missing |
| the delegate (cheap second model) design | **confirms** | the reasoning in `docs/models.md` matches what this project would have derived; no work follows from it yet |

---

## What the tracker gave that the code did not

⭐ **Three items, and two of them changed a conclusion.**

- **Issue 2** is the host-RCE finding above. Nothing in the code or the
  documentation hints at it. ⛔ It is the clearest possible argument for the
  rule that a sweep reads the tracker.
- **Issue 1, closed**, is a TLS failure on Arch hosts because
  `/etc/ca-certificates` was not granted. The maintainer's comment
  (2026-09-14) records something the reporter had got wrong: the fault was not
  Arch-family only, openSUSE fails the same way through
  `/var/lib/ca-certificates`, and nobody had hit it yet. The fix names four
  families at once rather than branching per distribution, on the stated
  ground that naming an absent path is already how the list works.
  ⭐ **Both paths are in `SYSTEM_READ` at the pinned commit**
  (`src/sandbox/policy.ts:56-60`), which is this sweep confirming a closed item
  against the code rather than believing it.
- **Issue 3, open**, proposes starting sessions from GitHub mentions. Its own
  security section states the constraint plainly: "the GitHub token is already
  reachable by the agent". That is the maintainer-side confirmation of the leak
  this project closes, written by somebody arguing for a feature rather than
  against the design.
