# harnesses: findings

Three projects this one **drives** rather than ports: `badlogic/pi-mono` (the
agent harness that runs inside a sandbox), `herdrdev/herdr` and
`pingdotgg/t3code` (control surfaces that drive vendor agent CLIs).

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
- ⛔ **This is revision 1. Assume more claims are wrong than have been found.**

## ⚠ The claims here that are weakest

1. **The interface details** (flag names, file paths, protocol shapes) are the
   most likely to be stale. ⭐ **Re-read the published documentation before
   writing an adapter**, and treat this page as a map rather than a
   specification.
2. **"herdr and t3code are not needed here" is this sweep's judgement**, argued
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

## herdr and t3code: read, and not adopted

⭐ **Both are control surfaces for vendor agent CLIs, and both solve a problem
this project does not have.**

**`herdrdev/herdr`** describes itself as "the runtime your coding agents live
on". One Rust binary. It keeps terminals running in a background server across
disconnects, restores a saved layout after a restart, marks every pane working,
blocked or idle, and exposes a CLI and socket API that agents themselves drive.
⭐ Its own positioning is the reason it is not a fit: "**herdr doesn't wrap or
replace them; it owns their terminals**." This project's boundary is a network
namespace and a credential broker, not a terminal.

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
| pi, as the harness inside the sandbox | ⭐ **adopt**, as a dependency | T-040, now settled |
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
