# egi

Run coding agents from chat, sandboxed, with secrets brokered at the boundary.

A message in one configured channel opens a thread and starts a session. The
agent works in one project directory and nowhere else. Replies in the thread
are prompts; what the agent says, runs, and changes comes back to the thread.

egi is a Rust rewrite of [errand](https://github.com/QaidVoid/errand). The
shape carries over: one channel, one thread per session, several sessions at
once, a queue bounding model-provider work, and the chat token never entering
a sandbox. What changes is the agent runtime and the credential story:

- The agent is not a spawned CLI holding a key. Sessions reach models through
  `models.dev` endpoints, and every credential is brokered at the boundary.
- The agent never holds a real credential at any point in its lifetime. Dummy
  values are substituted at the egress broker, per binding, per session.
- Backends are chosen per session: bailey on Linux, containers on Windows
  through WSL2, hardware isolation (gVisor, Kata) where the operator configures it.
- Everything is a crate with one seam: chat adapters, sandbox backends,
  provider bindings, and egress policy compose without forking the daemon.

## Status

Skeleton. The workspace builds, the checks run, and no daemon exists yet.
`TODO/PROGRESS.md` is the record of where the work stands.

## Quick start

```bash
gh repo clone talaria0101/egi
```

```bash
cd egi
```

```bash
cargo test
```

```bash
sh scripts/common/check-gate.sh
```

What you should see: 8 unit tests pass, and the gate reports green except
the changelog check, which exits 2 until `CHANGELOG.md` exists.

## Requirements

| tool | minimum | why |
| --- | --- | --- |
| Rust | 1.86 | the whole daemon |
| POSIX shell | any | the checks in `scripts/` |

```bash
sh scripts/doctor/doctor.sh
```

That reports what is installed here and at what version.

## What it does

- Confines every session. There is no unsandboxed mode.
- Brokers every credential at the egress boundary. Sessions hold nonces.
- Runs on Linux natively and on Windows through WSL2.
- Keeps the chat token out of every sandbox.

## What it does not do

- It does not run agents without confinement. A backend that cannot enforce
  the configured guarantees stops the daemon instead.
- It does not mask secrets in output and call that a boundary. Masking loses
  to trivial encoding. Brokering is the boundary.
- It does not stop exfiltration through authorized channels. An agent with
  legitimate write access can still push data somewhere it may write. That
  residual risk is tracked openly, not solved by this project alone.

## Documentation

| file | what it answers |
| --- | --- |
| [`TODO/PROGRESS.md`](TODO/PROGRESS.md) | where the work is, and what is next |
| [`docs/AGENTS.md`](docs/AGENTS.md) | how an agent works on this repository |
| [`TODO/RULES.md`](TODO/RULES.md) | how this repository is worked on |
| [`HUMAN.md`](HUMAN.md) | the operator's side: setup, validation, runbooks |
| [`SECURITY.md`](SECURITY.md) | the threat model and who holds what |
| [`CHANGELOG.md`](CHANGELOG.md) | what shipped, and when |
| [`docs/architecture.md`](docs/architecture.md) | the technical reference. When any document disagrees with it, it wins. |

## Contributing

Issues and pull requests are welcome. Read `TODO/RULES.md` and
`docs/AGENTS.md` before starting. No commit credits a tool.

## Licence

0BSD. See [`LICENSE`](LICENSE).
