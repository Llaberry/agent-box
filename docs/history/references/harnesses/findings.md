# harnesses: findings

Four projects this one **drives** rather than ports: `earendil-works/pi` (the
agent harness that runs inside a sandbox), `can1357/oh-my-pi` (a fork of it with
a credential broker already built), `herdrdev/herdr` and `pingdotgg/t3code`
(control surfaces that drive vendor agent CLIs).

⛔ **This sweep read documentation only.** These are dependencies, not designs
to reimplement or patch. At most this project writes a wrapper around one, so
what matters is the interface each publishes and what its own documentation
promises. [`usable.md`](usable.md) is the interface. [`../pins.md`](../pins.md)
is the commits.

---

## ⛔ What this sweep did NOT establish

- **No tracker was fetched and no code was read**, by decision rather than by
  obstacle. ⚠ That is a real limit on everything below: a published document is
  evidence of what a maintainer intends, never of what the code does, and none
  of these claims was checked against an implementation.
- **Nothing was run.** No harness was installed, started, or driven.
- ⛔ **No version was pinned beyond the commit.** These move fast, and a flag or
  a protocol detail below can be wrong by the time an entry implements it.
- **`herdr` and `t3code` were read from a README and a handful of pages**, not
  from their full documentation sets.
- ⚠ **`oh-my-pi` has about 80 documentation pages and four were read.** The
  conclusions below are drawn from those four.
- ⛔ **This is revision 2, and revision 1 got two of the four references
  wrong.** It pinned a mirror instead of the canonical repository, read one
  project's README instead of the documentation index it publishes for agents,
  and ⛔ **did not sweep `oh-my-pi` at all**, which turned out to carry a
  credential broker four entries were planning to build.
  [`../../README.md`](../../README.md) carries all three withdrawals.
  ⭐ **That is the honest estimate of how much of revision 2 is still wrong.**

## ⚠ The claims here that are weakest

1. **The interface details** (flag names, file paths, protocol shapes) are the
   most likely to be stale. ⭐ **Re-read the published documentation before
   writing an adapter**, and treat this page as a map rather than a
   specification.
2. ⛔ **Everything about `oh-my-pi`'s broker is read from one page**, and that
   page describes intent. Whether it behaves as described, how it performs, and
   what it does under failure are all unestablished, and T-042's first job is to
   find out.
3. **"herdr and t3code are not needed here" is this sweep's judgement**, argued
   below, not anybody's position.

---

## ⭐ pi names the gap this project fills

`packages/coding-agent/docs/security.md`, "No Built-in Sandbox", states it
without hedging:

> Pi does not include a built-in sandbox. Built-in tools can read files, write
> files, edit files, and run shell commands with the permissions of the pi
> process. Extensions are TypeScript modules that run with the same permissions.

And the reasoning, which is worth keeping because it is the argument against a
half-measure:

> A partial in-process sandbox would be easy to misunderstand as a security
> boundary while still depending on the host shell, filesystem, package
> managers, credentials, and extension code. Real isolation needs to come from
> the operating system or a virtualization/container boundary.

⭐ **Its containerization page is a table of four patterns and every row states
its own credential cost**, which is the most useful thing in this sweep:

| pattern | what is isolated | the credential |
| --- | --- | --- |
| a micro-VM extension | built-in tools and shell commands | stays on the host, because pi stays on the host |
| plain container | the whole process | ⛔ **"Provider API keys enter the container."** |
| a policy-controlled sandbox | the whole process | "can keep raw model API keys outside the sandbox" |
| a managed sandbox service | the whole process | "with provider keys kept on the host" |

⛔ **This project is the missing row: the whole process isolated, the credential
outside, self-hosted, with no third-party gateway.** That is the one
combination the table does not have.

⚠ **And the page carries a warning this project must not trip over:** "avoid
mounting host `~/.pi/agent` unless the container should access host sessions,
settings, and credentials". That directory holds `auth.json` and the session
store. ⭐ A session gets a fresh one inside its own state directory, never the
operator's.

---

## ⛔ The finding: a repository can ship configuration that pi will load

`docs/security.md`, "Project Trust". Pi loads project-local settings,
extensions, skills, prompts and system-prompt files from a `.pi/` directory in
the working directory:

- `.pi/settings.json`
- `.pi/extensions`, `.pi/skills`, `.pi/prompts`, `.pi/themes`
- `.pi/SYSTEM.md`, `.pi/APPEND_SYSTEM.md`
- `.agents/skills` in the current directory or an ancestor

⭐ **Extensions are TypeScript modules that run with the pi process's
permissions.** In this project's terms, that is code running as the agent,
chosen by whoever wrote the repository.

Interactively pi asks, and the answer is stored per directory. ⛔ **But the
modes this project uses do not ask:**

> Non-interactive modes (`-p`, `--mode json`, and `--mode rpc`) do not show a
> trust prompt. Without an applicable saved trust decision,
> `defaultProjectTrust: "ask"` and `"never"` ignore such resources, while
> `"always"` trusts them.

So the behaviour is safe by default and **one setting away from not being**. A
session cloning an arbitrary repository, with `defaultProjectTrust` set to
`always` by an operator who wanted fewer prompts, executes that repository's
extension code.

⛔ **This project pins the setting rather than inheriting it**, and T-046 is the
entry. ⚠ Note what it is and is not: it is an input-loading guard, and pi says
so itself, "It does not make untrusted code, untrusted prompts, or untrusted
model output safe."

⭐ **The class generalises, and it is the same class as the host-side-tool
finding** in [`../errand/findings.md`](../errand/findings.md): a tool reading
configuration out of a directory the agent controls. Here the tool is the agent
itself.

---

## What pi publishes that this project drives

| | |
| --- | --- |
| **headless mode** | `pi --mode rpc`, a JSON protocol over stdin and stdout. Commands in, responses and events out. |
| ⛔ **framing** | strict JSONL, LF only. See the trap below. |
| **flags this project sets** | `--provider`, `--model`, `--session-dir`, `--name`, `--no-session` |
| **credentials** | OAuth for subscription providers, or an API key from an environment variable or `auth.json`. `/login` and `/logout` manage both. |
| **provider catalogue** | built-in, refreshable, cached at `~/.pi/agent/models-store.json` for offline use |
| **custom providers** | declarable with a `baseUrl`, which is what makes brokering work without patching pi |
| **usage** | pi's own footer reports token and cache usage, cost, and context usage. ⚠ See below. |

### ⛔ The framing trap, stated by pi itself

> Node `readline` is not protocol-compliant for RPC mode because it also splits
> on `U+2028` and `U+2029`, which are valid inside JSON strings.

⭐ **A generic line reader corrupts this protocol on input a model can produce.**
Split on `\n` only, strip a trailing `\r`. In Rust that means not reaching for a
lines iterator without checking what it treats as a line break.

### ⚠ Usage: pi reports its own, and that is not the same as measuring it

Pi's footer carries token and cache usage and cost, and its usage page says the
totals "include assistant responses, usage reported by tools, and summary
generation".

⭐ **An agent reporting its own usage is the agent reporting on itself.** The
broker sees every request and every response body and cannot be lied to by the
thing it is metering. ⛔ **Take both and compare them**, and treat a divergence
as a finding rather than as noise. T-120.

---

## ⭐ The finding that changed four entries: a credential broker already exists

**`can1357/oh-my-pi`, `docs/auth-broker-gateway.md`.** A fork of the harness
above, carrying two cooperating services:

| service | does |
| --- | --- |
| `omp auth-broker serve` | holds the credential vault, performs OAuth refreshes, and exposes snapshot, credential, block, usage and health APIs |
| `omp auth-gateway serve` | a forward proxy accepting several model API shapes, resolving the broker-backed credential and dispatching. ⭐ **"Clients... never see the access token."** |

⭐ **Four mechanisms this project had written entries to build, already built:**

1. ⛔ **The refresh flow is brokered, not just the access token.** Clients load a
   redacted snapshot in which every refresh field is replaced by a sentinel;
   when an access token expires the client asks the broker and **the refresh
   happens server-side**. The client store rejects local mutation outright.
   ⭐ **That is exactly the rule this project wrote as "a broker that injects the
   access token and lets the tool keep the refresh token has brokered the cheap
   half"**, and it is somebody else's shipped code rather than this project's
   prose.
2. **Per-credential rate-limit blocks**, with a recorded cause, and endpoints to
   set and clear them. That is the usage-window state T-044 describes.
3. **Usage APIs**, including one for usage a client observed and one summarising
   client-observed usage. T-120's accounting.
4. **Generation-based conditional snapshot polling**, where a client sends the
   generation it holds and gets either a new snapshot or nothing.
   ⭐ **Independently the same mechanism as a sandbox project's credential vault**,
   which is the strongest evidence available that it is the right shape.

Plus the small things that are easy to get wrong: a timing-safe token
comparison, a token file at mode `0600` under a `0700` directory, and a
background refresher with a configurable skew ahead of expiry.

### ⛔ What it does not do, and this is the division of labour

⚠ **It is a provider credential broker, not a network boundary.** Read carefully,
it solves one half:

| | `oh-my-pi`'s broker | this project's broker |
| --- | --- | --- |
| model provider credentials | ⭐ **yes, including refresh** | defers to it |
| a repository token, or any other upstream | no | ⭐ yes |
| which hosts a session may reach at all | no | ⭐ yes, and it is the namespace rather than a setting |
| binding a credential to a proven peer identity | not described | ⭐ yes, the SNI gate |
| what happens to traffic that is not a model call | not its concern | ⭐ refused |
| transport security between the parts | ⛔ **"delegated to the operator"** | inside one process and one host |

⭐ **So they compose rather than compete**, and the honest plan is to drive it
for provider credentials and keep the network boundary here. ⛔ **Four entries
change from "build this" to "evaluate and drive this", and saying so is cheaper
than discovering it during implementation.**

⚠ **And one caution its own page states:** transport security between operator,
broker and gateway is the operator's to provide. A broker reachable over a
network this project did not bound is a credential store reachable over a
network this project did not bound.

## ⛔ A second setting that defaults permissive

`can1357/oh-my-pi`, `docs/approval-mode.md`. Tool approval has three tiers,
`read`, `write` and `exec`, and three modes:

| mode | auto-approves | prompts for |
| --- | --- | --- |
| `always-ask` | `read` | `write`, `exec` |
| `write` | `read`, `write` | `exec` |
| ⛔ **`yolo` (default)** | `read`, `write`, `exec` | **none** |

⭐ **One default is safe and the other is not, in the same document.** A tool
with no declared tier is treated as `exec`, which the page calls "the safe
default for unknown custom tools" and is right. The mode's own default
auto-approves everything.

⚠ **For an interactive user at a terminal that is a reasonable default.** For a
session started by a message in a public channel it is not, and ⛔ **this project
pins it rather than inheriting it**, exactly as it pins project trust. T-048.

## ⚠ Secret obfuscation: a mitigation, and not a boundary

`can1357/oh-my-pi`, `docs/secrets.md`. Values matching configured entries and
credential-shaped patterns are replaced with deterministic placeholders before
text reaches a provider, and restored in model-authored tool arguments before
execution.

⭐ **The direction is worth noticing**: it stops a secret reaching the model,
which is a different problem from stopping the agent leaking one. It is useful,
and it is off by default.

⛔ **It is not a boundary and this project must not count it as one.** The
settled position in [`../../../limits.md`](../../../limits.md) holds: masking
loses to trivial encoding. ⚠ Its own design shows why the weaker claim is the
true one: placeholders are restored before a tool runs, so anything that can
call a tool can see the real value.

---

## herdr and t3code: read, and not adopted

⭐ **Both are control surfaces for vendor agent CLIs, and both solve a problem
this project does not have.**

**`herdrdev/herdr`** describes itself as "the runtime your coding agents live
on". One Rust binary. It keeps terminals running across disconnects, restores a
saved layout after a restart, marks every pane working, blocked or idle, and
exposes a CLI and socket API that agents themselves drive. ⭐ Its own positioning
is the reason it is not a fit: "**herdr doesn't wrap or replace them; it owns
their terminals**." This project's boundary is a network namespace and a
credential broker, not a terminal.

⭐ **One thing in it is worth taking anyway, and it is a design posture rather
than a mechanism.** Its state detection is authoritative where an integration
reports lifecycle hooks and falls back to reading the terminal otherwise, and
its own page states the rule for the ambiguous case:

> Blocked detection is deliberately strict... If no manifest rule matches for a
> known agent, Herdr falls back to `idle`... The misclassification affects only
> the visible status and waits. It should not make Herdr send input or take
> destructive action.

⛔ **A state this project derives is used to surface and never to act**, which is
T-122's recommendation reached independently by somebody with the detection
problem in production.

⚠ **And its integration list names sixteen agent tools it can detect**, which is
a measure of how many harnesses exist rather than a list this project needs.

**`pingdotgg/t3code`** is an "agent harness control surface" that drives Codex,
Claude Code, Cursor, Grok Build, OpenCode and Antigravity **using the
subscriptions already authenticated on the machine**, from mobile, web and
desktop clients.

⚠ **That last property is the one worth taking seriously**, because it is the
shape the operator asked about: a vendor CLI, already logged in, driven by
something else. ⛔ **And it is the shape with the worst credential story**: the
CLI holds a long-lived subscription token on disk, refreshable, tied to a
person's account, and worth far more than an API key.

⭐ **So the mechanism transfers and the architecture does not.** Running a
vendor CLI as a provider is [`usable.md`](usable.md)'s provider-sandbox shape
and T-045; adopting either project's control plane is not.

### What t3code's auth model teaches, and it is worth more than its product

`docs/internals/environment-auth.md` is short and every rule in it is
transferable to this project's web surface:

| rule, in its own words | why it matters here |
| --- | --- |
| "Exchanging a bootstrap credential can narrow that grant but cannot widen it." | a session token derived from a login can only ever be a subset |
| "client labels and device metadata have no authorization role" | ⛔ nothing the client says about itself decides what it may do |
| "The access read model contains pairing metadata, never recoverable pairing secrets... read access to the connections list would become a way to acquire another client's authority." | ⭐ a list endpoint that returns secrets is an escalation |
| "an invalid proof must fail rather than fall back to bearer authentication" | ⛔ no downgrade on failure |
| "A rejected normal credential never falls back to the reusable credential." | the same rule, stated for a second case |
| "short-lived WebSocket tickets through authenticated HTTP so long-lived tokens stay out of socket URLs" | a token in a URL is a token in a log |
| "A successful handshake grants no extra authority: every RPC declares a required scope." | ⭐ authentication is not authorization, per call |

---

## Verdicts

| subject | verdict | where it lands |
| --- | --- | --- |
| the harness, inside the sandbox | ⭐ **adopt**, as a dependency | T-040, now settled |
| ⭐ **the fork's auth broker and gateway, for provider credentials** | **adopt**, as a dependency, and ⛔ **not rebuilt** | T-042, and it changes T-044, T-045 and T-120 |
| the refresh-token sentinel, and a client store that refuses local mutation | **confirms** this project's own rule, in shipped code | T-045 |
| generation-based conditional snapshot polling | **confirms**, independently of a second reference | T-026 |
| pinning the tool approval mode rather than inheriting `yolo` | ⭐ **adopt** | T-048 |
| secret obfuscation before text reaches a provider | ⚠ **honest limit**, useful and not a boundary | [`../../../limits.md`](../../../limits.md) |
| ⛔ transport security between broker parts left to the operator | **anti-pattern exhibit** for this deployment | T-042 bounds it rather than inheriting it |
| the fork's own container definitions and bot orchestrator | **filed elsewhere** | ⚠ close to what this project is for, and read only as a name. Worth a sweep of its own. |
| herdr's "surface a state, never act on it" | **confirms** | T-122 |
| `--mode rpc` and its JSONL framing rule | **adopt** | T-047 |
| pinning `defaultProjectTrust` rather than inheriting it | ⭐ **adopt** | T-046 |
| a fresh agent home per session, never the operator's | **confirms** what errand already does | T-012 |
| a custom provider with a `baseUrl`, so brokering needs no patch | **adopt** | T-042 |
| pi's own usage totals, as a second source to compare against | **adopt** | T-120 |
| the vendor-CLI-as-provider shape | **adopt** the mechanism | T-045 |
| t3code's scoped-session rules for the web surface | ⭐ **adopt** | T-110 |
| herdr as a terminal runtime | **refused** | it owns terminals; this project owns a namespace and a broker |
| t3code's control plane, clients and relay | **refused** | a different product. ⚠ Its auth page is kept as a source. |
| patching or forking any of the three | ⛔ **refused** | [`../../../methodology/vendoring.md`](../../../methodology/vendoring.md). We drive them and at most wrap them. |
