# agent-box

Run AI coding agents in sandboxes, with every credential held outside them.

An agent gets a random nonce. A broker on the host attaches the real credential
to outbound requests at the boundary, after checking that the request is one the
operator allowed, to a peer that proved its identity. ⛔ **The agent never holds
a credential at any point in its lifetime.**

---

## ⚠ Status: a plan, not a program

**There is no code yet.** This repository holds the design, the research it is
built on, and 58 filed entries.

⛔ **Nothing here can be run.** Every capability described below is a decision
that an entry in [`TODO/INDEX.md`](TODO/INDEX.md) implements, and every
containment claim is currently somebody else's measurement on a machine this
project has never seen. [`docs/limits.md`](docs/limits.md) says which.

---

## What it is for

A multi-tenant service running agents that act on repositories and other APIs on
somebody's behalf. Three properties, in order of how hard they are:

1. **The agent holds no credential.** Not in its environment, not on its disk,
   not in a configuration file it can read. Environment redaction is impossible
   (`execve` copies the environment onto the process stack and `getenv` is a
   pointer walk) and output masking is not a boundary, so the credential has to
   be somewhere the agent is not.
2. **The agent has one route off the host**, and it is the broker. Not a proxy
   it is asked politely to use: a network namespace with no other route.
3. ⚠ **The agent cannot use what it has been given for something else.** This one
   is not solved, here or anywhere. See below.

---

## How it works

```
   the operator's machine
   ---------------------------------------------------------
   agent-box                          the credential store
     the broker          <---------->  never leaves this process
     the session manager
     the policy generator
          |
          | starts, and is the only route off the host
          v
   ---------------------------------------------------------
   one session                        holds no credential
     the agent, confined by a backend
     a project directory, a state directory
     one route out: the broker, at a link-local address
   ---------------------------------------------------------
```

**Linux**: a session runs as a confined host process, using Landlock, seccomp
and namespaces, with no image to build. **Windows**: a container in a podman
machine, because the Linux tool is Linux and stays Linux.

Full design: [`docs/architecture.md`](docs/architecture.md).

---

## ⛔ What it does not do

**Brokering closes credential theft. It does not close exfiltration through an
authorized channel.**

An agent that never sees a token, in a sandbox with no route off the host except
one broker, can still push a private repository's contents somewhere public, if
the broker is configured to let it reach that host at all. The credential was
never stolen. It was used, for exactly what it was granted.

⭐ **That is the honest statement, and it is the same problem at every project
working on this.** [`docs/limits.md`](docs/limits.md) has the full set,
including which mitigations are real and which are theatre.

---

## Where to start

| you are | read |
| --- | --- |
| **an agent asked to work on this** | ⭐ [`docs/AGENTS.md`](docs/AGENTS.md). It is standalone: read it end to end and it tells you what to do first. |
| **deciding whether this is for you** | [`docs/limits.md`](docs/limits.md) first, then [`docs/architecture.md`](docs/architecture.md) |
| **wondering what it is built on** | [`docs/history/references/`](docs/history/references/). Fourteen projects, at pinned commits, with what each reading changed. |
| **looking for the work** | [`TODO/INDEX.md`](TODO/INDEX.md) for the list, [`TODO/PROGRESS.md`](TODO/PROGRESS.md) for the order |
| **the operator** | [`HUMAN.md`](HUMAN.md) |
| **assessing the threat model** | [`SECURITY.md`](SECURITY.md) |
| **wondering what changed** | [`CHANGELOG.md`](CHANGELOG.md), and [`RESUME.md`](RESUME.md) for what the last session had in flight |

---

## What the research found

Six things that were believed at the start and are not believed now.
[`docs/history/references/README.md`](docs/history/references/README.md) has the
full list with sources.

- ⭐ **The most serious defect in the project this replaces was not in its
  sandbox.** It was the daemon running `git` on the host against a directory the
  agent writes, which is host code execution as the daemon's user through
  `.git/hooks/pre-push`. It came from that project's tracker, not its code.
- ⭐ **Only the TLS SNI is an endpoint identity the peer must prove.** Selecting
  a credential binding from the `Host` header lets a session have the operator's
  credential delivered to a peer of its choosing. Two projects shipped that and
  fixed it.
- **A container network flag is not a containment claim across a hypervisor
  hop.** On Windows the flags close the podman machine, and what crosses the hop
  beyond is a firewall policy invisible from inside.
- **A partial egress allowance drops the network namespace**, leaving TCP port
  rules only. An agent that calls a model API is exactly that case.
- **Nobody has solved short-lived per-execution credential minting.** It is an
  open feature request at the projects furthest along.

---

## Licence

**0BSD.** See [`LICENSE`](LICENSE). `SPDX-License-Identifier: 0BSD`.

Use it, copy it, change it, ship it. No attribution required, no notice to
carry, no conditions at all.

⭐ Deliberately the most permissive option available, for a practical reason: a
coding agent asked to read or reuse a file will sometimes decline over licence
terms, and every condition is one more thing for it to decline over. 0BSD
removes them all while staying OSI-approved and SPDX-listed.
