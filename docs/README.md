# docs

The map. Which document answers which question, so a session reads what its task
needs rather than everything.

⭐ **Start at [`AGENTS.md`](AGENTS.md).** It is the router, it is standalone, and
a session told only "read it and follow" is oriented by the end of it. This page
is the index behind it.

⚠ **Read the row, then read the document.** These summaries route; they do not
substitute.

---

## This project

| file | answers |
| --- | --- |
| ⭐ [`AGENTS.md`](AGENTS.md) | what to read for which task, and what to do first. The one file a session with no context needs. |
| ⭐ [`architecture.md`](architecture.md) | the technical reference. The components, the trust boundary, the lifecycle. ⛔ **When any document conflicts with it, it wins and the other is the defect.** |
| [`credential-brokering.md`](credential-brokering.md) | how a session reaches an upstream without holding a credential. The eight-condition injection gate. |
| [`sandbox-model.md`](sandbox-model.md) | what confines a session, what each backend promises, and where each promise ends |
| [`limits.md`](limits.md) | what this project does not do, and is not going to. ⛔ Including the one it would be most tempting to overstate. |

## methodology: how work is planned, gated and handed over

| file | answers |
| --- | --- |
| [`gate.md`](methodology/gate.md) | what an entry passes before it is done. Three parts, none skippable. |
| ⭐ [`reviews.md`](methodology/reviews.md) | the three review lenses, and why one sweep written up three times is not three passes |
| [`sessions.md`](methodology/sessions.md) | what a session owes at its start and its end, how to resume, how to freeze cleanly |
| [`authoring.md`](methodology/authoring.md) | how a rough idea becomes an approved entry. Authoring and implementing are different sessions. |
| [`work-todo.md`](methodology/work-todo.md) | the todo model: an index, a record, entries that close in place |
| [`references.md`](methodology/references.md) | how to study somebody else's project, including the two steps that always get skipped |
| [`experiments.md`](methodology/experiments.md) | taking your own measurements, and why a negative result is committed |
| [`vendoring.md`](methodology/vendoring.md) | third-party code in this tree. ⛔ Patch it here; upstreaming is not a topic. |
| [`history.md`](methodology/history.md) | where the story goes, so it stops being written into the pages that answer questions |

## conventions: how things are written here

| file | answers |
| --- | --- |
| [`prose.md`](conventions/prose.md) | how documents are written. The five characters, the density ceiling, and why amendments are made in place. |
| [`docs.md`](conventions/docs.md) | the document set, one fact one home, and the changelog rules |
| [`git.md`](conventions/git.md) | commit identity, what may reach a remote, what is never committed |
| [`code.md`](conventions/code.md) | one read path one write path, build to last, and the testing tiers |
| ⭐ [`forbidden-patterns.md`](conventions/forbidden-patterns.md) | the table to grep yourself against before declaring a gate green |
| [`shell.md`](conventions/shell.md) | quoting, heredocs, exit codes, streams, line endings, and the platform traps |

## tooling

| file | answers |
| --- | --- |
| ⭐ [`agent-tooling.md`](agent-tooling.md) | what command does what job, and which entry builds it, because almost none exist yet. ⛔ Read it before installing anything or deciding a job cannot be done. |
| [`containers.md`](containers.md) | measuring something this machine cannot measure, in a machine you throw away |
| [`hosted-sessions.md`](hosted-sessions.md) | a machine somebody else provisioned and will take back: what to read off it, what it lies about, and the two ways it dies quietly |

## security

| file | answers |
| --- | --- |
| [`secrets.md`](security/secrets.md) | what never enters the tree, and what to do when something did |
| [`remote-ops.md`](security/remote-ops.md) | the three tiers governing action on anything outside this machine |
| [`public/README.md`](public/README.md) | what changes because this repository is public |

## history: what was believed, and why that changed

| file | answers |
| --- | --- |
| [`history/README.md`](history/README.md) | the index, and the list of claims this project has published and later withdrawn |
| [`history/references/`](history/references/) | what reading eleven other projects found, at pinned commits, with what transfers and what must not |
| [`history/prior-attempt.md`](history/prior-attempt.md) | a previous attempt at this same work, read for what it missed |
| [`history/reviews/`](history/reviews/) | what each deep review swept, what it found, and what it did not look at |

---

## The rules these documents hold themselves to

- ⛔ **One fact, one home.** A value in two documents with no check between them
  drifts, and the copy a reader trusts is the wrong one.
- ⛔ **Amend in place.** When a rule changes, the rule is rewritten and the
  superseded wording moves to [`history/`](history/). Stacking a dated box under
  retired text has a documented failure mode.
- **Every claim verified before it is written.** Writing the documentation is
  the audit, and the most confident sentence in a file is regularly the only
  false one.
- **Never a fabricated number.** A dash where the value is unknown.
- **A page nothing links to is a finding.** Unlinked means unread, which means
  uncorrected.

⚠ **None of that is enforced yet.** T-052 builds the checks that hold the
mechanical half, and until it lands these rules are held by reading alone.
