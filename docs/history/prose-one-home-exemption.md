# prose.md: the one-fact-one-home exemption

Superseded 2026-09-15. The live rule is in
[`../conventions/prose.md`](../conventions/prose.md), under "One fact, one
home".

---

## The wording that was here, verbatim

> ⚠ **Two exemptions, both narrow.** The entry-point routers are exempt from each
> other, because each states the absolutes in full for a session that may be
> handed exactly one of them; a router sharing a sentence with anything else is
> still refused. And the history directory is exempt entirely, because a
> superseded page states things the live pages now state differently, which is
> the point of it.

## Why it changed

⛔ **It was vacuous in this repository.** It came from a template that ships
three entry-point routers, so "exempt from each other" had something to exempt.
This project has one, [`../AGENTS.md`](../AGENTS.md), and under the rule as
written the exemption applied to nothing while the second clause refused the
absolutes that file exists to state.

⚠ **The defect was found by running the check, not by reading the rule.** The
duplicate-sentence pass over this tree flagged one sentence shared between
`AGENTS.md` and `methodology/work-todo.md`: the rule about nothing closing as
"won't fix", "upstream's problem" or "out of scope". Both copies are correct and
both are wanted: the methodology owns the rule, and the router states it because
a session may be handed the router alone.

⭐ **The alternative was to weaken the router**, by replacing the absolute with
a link. That was rejected: an absolute that a session has to follow a link to
read is an absolute that gets skipped, which is the failure the router exists to
prevent.

## What the rule says now

The router is exempt for the numbered absolutes and the two project-specific
rules beside them. Any other sentence it shares with any other page is still
refused, and the check names the sentence.
