# references

What this project read before it built anything, at which commit, and what each
reading changed.

[`../../methodology/references.md`](../../methodology/references.md) is the
procedure. ⛔ [`pins.md`](pins.md) is the commit table and the list of what was
not fetched; read it before citing anything here.

---

## Route the reader by budget

| you have | read |
| --- | --- |
| **two minutes** | this page, down to "What the sweep changed" |
| **ten minutes** | this page, then [`pins.md`](pins.md)'s gap table, then the "weakest claims" section at the top of whichever sweep you care about |
| **an entry to implement** | the `usable.md` of the sweeps its Source line names, in order |
| **a reason to distrust this** | [`../reviews/`](../reviews/), then [`pins.md`](pins.md), then the corpus |

---

## The six sweeps

| sweep | subject | the one thing it settled |
| --- | --- | --- |
| [`errand/`](errand/) | `QaidVoid/errand`, the project this replaces | ⭐ an orchestrator running a host-side tool against agent-writable state is host code execution, with no sandbox escape needed |
| [`bailey/`](bailey/) | `QaidVoid/bailey`, the Linux confinement this project drives | ⭐ a partial egress allowance drops the network namespace, and an agent that calls a model API is exactly that case |
| [`brokers/`](brokers/) | `agent-vault`, `OpenSandbox`, `CubeSandbox` | ⭐ only the TLS SNI is an endpoint identity the peer must prove; `Host` is input from the sandbox |
| [`orchestration/`](orchestration/) | `agent-sandbox`, `kage`, `flue` | ⭐ brokering closes credential theft and does not close exfiltration through authorized channels, and that is open at every reference |
| [`windows/`](windows/) | `talaria0101/malaria` | ⭐ the containment flags close the podman machine, not Windows, and the hop beyond is invisible from inside |
| [`harnesses/`](harnesses/) | `pi`, `oh-my-pi`, `herdr`, `t3code` | ⭐ the harness has no sandbox on purpose and says real isolation must come from outside it, so this project is the missing row in its own table. ⛔ Documentation only: these are driven, not ported. |

⚠ **A seventh reference, `talaria0101/egi`, was read and produced no sweep of its
own.** It is a prior attempt at this same work, and what it taught is recorded
in [`../prior-attempt.md`](../prior-attempt.md) rather than as a mechanism to
adopt.

---

## What the sweep changed

Eight things that were believed at the start of this project and are not
believed now. ⭐ **This list is the honest measure of how much the reading was
worth, and it is also the best estimate of how much is still wrong.**

1. **"errand leaks secrets to agents by design" is half right.** The provider
   credential is brokered behind a per-run nonce by default, and has been for
   some time. The GitHub token is not brokered at all and the code says so in a
   comment. [`errand/findings.md`](errand/findings.md).
2. **The interesting attack was not the sandbox.** It was the orchestrator
   running `git` on the host against a directory the agent writes. Nothing in
   the code or the documentation hints at it; it came from the tracker.
3. **A container network flag is not a containment claim across a hypervisor
   hop.** [`windows/findings.md`](windows/findings.md).
4. **`Host`-based binding selection is a credential-exfiltration primitive**,
   and two separate projects shipped it and fixed it.
   [`brokers/findings.md`](brokers/findings.md).
5. **Short-lived per-tenant credential minting is not a solved problem to
   port.** The furthest-along reference has it as an open issue. Building it is
   writing something new, and the plan says so.
6. ⭐ **The harness can load code out of the repository it is working on.** A
   `.pi/extensions` directory holds TypeScript modules that run with the
   harness's permissions, and the headless modes decide by a setting rather than
   a prompt. [`harnesses/findings.md`](harnesses/findings.md). It is the same
   class as finding 2, one layer in.
7. ⛔ **A provider credential broker already exists, in a fork of that harness**,
   and the plan had four entries to build one. It performs OAuth refreshes
   server-side, hands clients a snapshot with every refresh token replaced by a
   sentinel, and its proxy resolves the credential so clients never see the
   access token. ⭐ **It brokers provider credentials and does not bound the
   network**, so it composes with this project rather than replacing it.
   [`harnesses/findings.md`](harnesses/findings.md).
8. ⚠ **The same fork's tool approval mode defaults to approving everything.**
   Reasonable for a person at a terminal, wrong for a session nobody is
   watching. T-048 pins it.

⛔ **Assume more remain.** Three claims have been withdrawn so far, all three in
the harness sweep and all three found by the operator rather than by a review
pass. ⭐ **That is the honest signal**: the sweeps found real defects in other
people's work and missed a whole reference in their own.
[`../README.md`](../README.md) carries the withdrawals.

---

## The rules these pages are held to

- ⛔ **A tracker item is evidence of intent, never of behaviour.** Every defect
  reported here was read from its reporter's own text, with the named file
  opened at the pinned commit and confirmed to be the code described. Whether
  it still reproduces was not tested, and each sweep says so.
- ⛔ **Nothing here was run.** No daemon, no sandbox, no broker, no benchmark.
  Every sweep opens by saying what it did not establish.
- ⭐ **Each sweep's weakest claims are listed before its recommendations**,
  because a reader who reaches the recommendation first has already stopped
  reading.
- ⚠ **A citation is evidence of what somebody else did.** It is never evidence
  that this project does it.
