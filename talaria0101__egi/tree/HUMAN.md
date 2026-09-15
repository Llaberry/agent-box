# HUMAN.md

Your side of egi.

⭐ **One line: if it needs a login, a token, a payment, a domain or a judgement
call, it is yours. If it is code, tests or local verification, it is the
agent's.**

You own three things the agent cannot: **credentials and remote
infrastructure**, **validation** (you are the acceptance gate), and **session
management** (starting sessions with the right context, receiving handoffs).

---

## 1. Machine check

⭐ **This is your first task, and it exists so work does not stall three tasks
in on a missing tool.**

```bash
sh scripts/doctor/doctor.sh
```

That reports what is installed and at what version. What this project needs:

| tool | minimum | needed from | check |
| --- | --- | --- | --- |
| Rust | 1.86 | now | `rustc --version` |
| POSIX shell | any | now | `sh --version` |
| podman | 4.x | sandbox backend work | `podman --version` |
| bailey | latest | sandbox backend work | `bailey doctor` |

---

## 2. Secrets you hold

⛔ **Never paste a value into an agent session.** Name what you hold; the agent
tells you where it goes.

| what | where it goes | who sets it |
| --- | --- | --- |
| chat bot token | the config file, operator-readable only | you |
| provider credential | the daemon's secret store | you |
| GitHub token, if sessions need it | the daemon's secret store | you |

⚠ The agent will not ask for a value, and will not set a platform secret it
cannot read back. If one of those needs doing, it appears as an item for you.

---

## 3. Prompts to paste

### Start the next unit of work

The agent prints this at the end of every session. ⭐ **Paste it into a fresh
session.** It carries the reading order and a pointer to the record; it does
not carry the work order, because that lives in the record and would go stale
here.

### Resume an interrupted session

The agent prints this instead whenever anything was left unfinished. Paste it
the same way. ⚠ The agent will reconstruct the state from the tree and the
record rather than trusting anything it is told, including by you.

### Add new work

```text
Read docs/AGENTS.md and docs/methodology/authoring.md. Author a plan from
this intake. ⛔ Do not implement.

Title:
Type:            bug | feature | refactor | hardening | polish | chore
What and why:
Evidence:        <file and line, an error, a report, a URL>
In scope:
Out of scope:
Constraints:
Already decided:
```

### Reject a unit of work

```text
The unit is NOT accepted. What I found during validation:

1. What I did: ...
   What happened: ...
   What I expected: ...
2. ...

Reproduce each one first. Fix the root cause, not the symptom, and no
workarounds. Add a regression test per issue. Re-run the full gate. Update the
record. Do not proceed to anything else.
```

### Answer a design question

```text
Decision: <your decision in one sentence>.
Proceed with that. Record the question, my decision and the implications in the
record. Do not revisit unless I raise it.
```

---

## 4. Receiving a handoff

Fifteen minutes, and it is worth all fifteen:

1. Read the items at the top. Do them, or schedule them.
2. ⭐ **Run the verify commands yourself.** All of them.
3. Spot-check two or three acceptance items by hand.
4. Read the self-review answers. ⚠ **Red flags: "mostly", "should", an
   unanswered question, and a checklist item with no pasted output.**
5. Read the driven-pass log. ⚠ If it describes intent rather than what actually
   happened, that is a failed gate.
6. Accept, or reject with the prompt above.

---

## 5. Validating each unit of work

| unit | what to validate | what correct looks like |
| --- | --- | --- |
| skeleton | `cargo test` and the gate pass on a fresh clone | 8 tests green, docs check clean |

---

## 6. Ground rules to hold

- **One unit of work per session chain.** The agent hands you a prompt at each
  boundary; paste it into a fresh session.
- ⭐ **A deviation from a locked decision is your call, never the agent's.** It
  will ask. Answer in one sentence and it records the ruling.
- ⛔ **"Should work" without pasted output is not done.**
- **If it says it is blocked on you, check the record.** Either do the item,
  or tell it to mock and continue.
- ⚠ **If the agent accepts everything you say without a single question or
  counter-proposal, it is not doing the job.** The method is explicitly not a
  yes-machine.
