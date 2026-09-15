# Session

Lifecycle, the trusted channel, teardown, and diagnostics.

[`INDEX.md`](INDEX.md) is the list.

---

## T-030: The daemon never runs a tool against agent-writable state

**Source:** `QaidVoid/errand` issue 2, **open** (`58a178b`). Read [`../docs/history/references/errand/findings.md`](../docs/history/references/errand/findings.md) before starting.
**Category:** session
**Priority:** P0
**Effort:** M
**Status:** open
**Blocked by:** T-010

---

### Problem

⛔ **Host code execution as the daemon's user, with no sandbox escape needed.**

The daemon is trusted, runs unconfined, and holds every credential. Anything it
executes with a working directory the agent can write is the agent's code.

### Premise

⭐ **MEASURED by a reporter against a running deployment, and it is the most
serious finding in this whole index.**

errand issue 2. The daemon opened pull requests by running `git` **on the host**,
outside every sandbox, with the agent's project directory as the working
directory. Git reads and executes several things out of a repository it is
pointed at:

| what git executes | reached by |
| --- | --- |
| `.git/hooks/pre-push` | the daemon's own `git push --force-with-lease` |
| `.git/config` `credential.helper` | the push's authentication |
| `core.hooksPath`, `core.fsmonitor`, `diff.*.textconv`, `filter.*`, aliases | ordinary operations |

The gate deciding a pull request was asked for was itself a file in the agent's
writable state directory, so the agent triggered the whole path unaided.

The evidence in that item is a host marker file written as the daemon's user,
and a daemon log line proving the hook ran on the host because the host shell
could not find a path that exists only inside the sandbox.

⚠ **Not reproduced here.** The code named in the report was opened at the pinned
commit and is the code described; whether it still reproduces was not tested.

⭐ **The class is larger than git.** Any tool that reads configuration out of its
working directory: package managers with lifecycle scripts, linters with plugin
paths, formatters, test runners with conftest files.

### Approach

⛔ **The rule, and it is checkable rather than remembered:**

**No process the daemon spawns has a working directory under the project root or
the session state root, and no process the daemon spawns inherits a path under
either as configuration.**

In `box-session`:

- a single spawn helper that every daemon-side subprocess goes through
  ([`../docs/conventions/code.md`](../docs/conventions/code.md): one write
  path);
- it refuses a working directory under either root, as a typed error, at the
  call;
- it clears the environment and passes only named variables;
- ⛔ **it refuses to spawn a tool known to read configuration from its working
  directory at all**, whatever the directory. That list starts with `git` and
  the entry that adds one records why the tool is safe.

⭐ **What the daemon does instead: reads bytes out and writes bytes in.** Where
the product needs a repository operation, it happens in a directory the agent
has never touched, from content the daemon copied out and inspected.

⛔ **What it must not do:** rely on sanitising flags. `git -c core.hooksPath=/dev/null
-c credential.helper=` plus `GIT_CONFIG_NOSYSTEM=1` is a denylist over a surface
that grows with every release. The reporter suggested it as a floor and named
the stronger answer in the same paragraph; this entry takes the stronger one.

### Prove

```bash
cargo test -p box-session spawn:: && cargo xtask check spawn-cwd
```

Passing: `cargo test` exits 0, with named tests for a spawn under the project
root refused, a spawn under the state root refused, and an environment not
explicitly named absent from the child.

⛔ **And `check spawn-cwd` is a mutation-proved guard**
([`../docs/methodology/reviews.md`](../docs/methodology/reviews.md) lens 2): a
plant that constructs a spawn with a project-root working directory makes it
exit 1, and the closing records the planted diff and the exit code, read
unpiped.

⚠ `cargo xtask check spawn-cwd` is built by T-052.

### Closing

Not closed.

---

## T-031: A trusted channel the session cannot forge into

**Source:** `QaidVoid/errand` issue 2, secondary section (`58a178b`).
**Category:** session
**Priority:** P0
**Effort:** M
**Status:** open
**Blocked by:** T-010

---

### Problem

The daemon reads the agent's output on one stream and treats it as genuine. If
the session can write to that stream by another route, everything downstream of
it is attacker-controlled: the transcript, the tool-call record, anything an
operator reads to decide what happened.

### Premise

⭐ **MEASURED by the same reporter, in the same item.** Against a running
deployment:

- `/proc/<pid>/environ` was readable from inside the session, exposing other
  processes' environments including secret values;
- ⛔ `/proc/1/fd/1` was **writable**, which is the agent process's standard
  output and therefore the daemon's trusted event stream. **Forged events were
  accepted as genuine agent output** and a fabricated message and a fabricated
  tool call appeared in the transcript on disk;
- `/proc/1/fd/0` was writable, which is injection toward the agent.

⚠ Not reproduced here.

⛔ **The third is the one with no obvious fix in the policy.** A trusted channel
the untrusted party can write to is not a channel, and moving it out of `/proc`
is a design change rather than a grant change.

### Approach

In `box-session`:

- a fresh procfs in the session's own PID namespace, so `/proc` shows only the
  session's own processes. ⭐ **Necessary and not sufficient**: the agent's own
  descriptors are still in its own `/proc`;
- ⛔ **the daemon-facing channel is not the agent's standard output.** It is a
  descriptor the daemon creates, passes once, and which nothing inside the
  session can reopen by path. A unix socket pair, or a pipe whose write end is
  passed and whose path does not exist in the session's filesystem view;
- every message carries a monotonically increasing sequence the daemon checks,
  ⛔ **so a replay or an out-of-band write is detectable even where it is not
  preventable**;
- the agent's own standard output and error are captured separately and marked
  as untrusted in everything the daemon writes.

### Decision

**Whether the channel is authenticated, or only unforgeable by construction.**

⭐ **Recommendation: unforgeable by construction, plus the sequence.** A shared
secret in the session is a secret in the session, which is what this project
exists to avoid. The sequence gives detection without one.

⚠ **The alternative** is a per-session key the daemon passes and the agent
signs with. It detects a forged message rather than preventing one, and it
reintroduces exactly the thing the design removes.

### Prove

```bash
cargo test -p box-session channel::
```

Passing: exit 0, and ⛔ **the acceptance is a forgery that is seen to fail**:

| test | asserts |
| --- | --- |
| a process in the session writing to the agent's standard output | the write does not reach the daemon's event stream |
| a process in the session opening the channel by any path visible to it | no such path resolves |
| a message with a repeated sequence | refused, and reported |
| a message with a skipped sequence | refused, and reported |
| `/proc` inside the session | lists only the session's own processes |

⚠ **A test that only checks the happy path proves nothing here.** Lens 2: plant
the forgery and watch it be refused.

### Closing

Not closed.

---

## T-032: Teardown, and orphan discovery at startup

**Source:** `QaidVoid/errand` `src/sandbox/backend.ts:133-154` (`58a178b`); [`../docs/security/remote-ops.md`](../docs/security/remote-ops.md), "Teardown".
**Category:** session
**Priority:** P1
**Effort:** S
**Status:** open
**Blocked by:** T-010, T-022

---

### Problem

A daemon that crashes leaves sandboxes running. A session that ends leaves
broker bindings and nonces behind. Both are capabilities with no owner.

### Premise

READ, not measured. errand's trait carries `listOrphans` and `removeOrphans`
(`src/sandbox/backend.ts:143-150`), and its discovery works from a label and a
name prefix (`:40-47`).

⚠ **Nothing in that reference ties the broker's bindings to the sandbox
lifecycle**, because in its design the credential is in the sandbox rather than
in the broker. ⭐ **This project's teardown therefore has one more thing to
remove than its reference's does**, and forgetting it leaves a nonce the broker
would still honour.

### Approach

In `box-session`:

- ⛔ **teardown removes three things: the sandbox, the broker bindings for that
  session, and the nonces.** A nonce that outlives its session is a credential
  with no owner;
- teardown runs on every path, including a launch that failed part way;
- ⛔ **orphan discovery runs at startup**, before anything is served. A sandbox
  this system owns that no live session claims is removed;
- ⛔ **a daemon that cannot enumerate its own sandboxes does not start.** It
  cannot know what is running as it.

⚠ **Verify by counting, not by remembering.**
[`../docs/security/remote-ops.md`](../docs/security/remote-ops.md): a count that
returns to its baseline is evidence, and "I think I removed them" is not.

### Prove

```bash
cargo test -p box-session teardown::
```

Passing: exit 0, with named tests for: a failed launch leaving no sandbox, no
binding and no nonce; a nonce refused by the broker after its session is torn
down; orphan discovery finding a labelled sandbox with no live session;
discovery failing making the daemon refuse to start; and ⭐ a count of live
sandboxes returning to its baseline after a start-and-stop cycle.

### Closing

Not closed.

---

## T-033: Diagnostics good enough to work without kernel audit

**Source:** `QaidVoid/bailey` `docs/security/limitations.md` (`d3c73f7`), "A denied access is not reported".
**Category:** session
**Priority:** P2
**Effort:** M
**Status:** open
**Blocked by:** T-011, T-013

---

### Problem

⚠ **Landlock denies silently.** A session that fails under enforcement gives its
own error and nothing more, and an operator diagnosing it has no denial record
to read.

### Premise

READ, not measured. bailey's limitations page states it and says why it cannot
be fixed at that layer: seeing a denial needs the kernel's audit subsystem
enabled at boot **plus** permission to read its records, and a normal host
provides neither. On a kernel with the log made readable but audit disabled, a
deliberately triggered denial produced **zero** records.

⭐ **That is why an `on_violation` hook was specified, built, measured to be
unfirable, and deleted** (`docs/roadmap.md`, "Decided against"). ⛔ **Removed
rather than carried as config that does nothing**, which is the same rule as
this project's own forbidden pattern about a setting no code reads.

### Approach

The daemon cannot see the denial, so it makes the rest legible:

- ⭐ **the policy in force is written where an operator can read it**, per
  session, at a path outside the project. ⚠ Not a summary: the actual generated
  policy;
- the startup report names every grant, and ⛔ **calls out a writable grant
  separately**, since that is the one that lets a session change something
  outside its own project;
- a session's diagnostics carry what the probe measured on this host, so a
  failure on a host missing a capability reads as that rather than as a mystery;
- a command that answers "what was this session given", from the recorded
  policy rather than by re-deriving it.

⛔ **What it must not do:** add a hook that cannot fire, or report a denial it
did not observe. Both are worse than the silence.

### Prove

```bash
cargo test -p box-session diagnostics:: && cargo run -p box-cli -- sessions show --last
```

Passing: `cargo test` exits 0 with named tests for the written policy matching
the generated one byte for byte, and a writable grant appearing in its own
section of the report. The command prints the policy and the probe result for
the most recent session, and its output goes in this entry's closing.

### Closing

Not closed.
