# Chat

## T-013: Chat seam plus Discord adapter with reconnect policy

**Source:** errand `src/chat/gateway.ts` plus `threads.ts` plus `inbound.ts` plus `outbox.ts` (`634b7b9`), flue discord blueprint (`a20ef15`, sweep 4).
**Category:** chat
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

Sessions are driven from chat, but the daemon must not marry one chat
platform. Without a seam, adding a second platform rewrites the session
layer.

### Premise

READ, not measured. errand's Discord layer is the reference; egi keeps
the seam and the first adapter.

### Approach

Create `crates/egi-chat` depending on `egi-core`:

- Port the gateway from `src/chat/gateway.ts` lines 32-90:
  `ReconnectPolicy` and `DEFAULT_RECONNECT` (lines 32-44),
  `reconnectDelayMs` (lines 45-51), `GatewayHandlers` (lines 53-65),
  `toRaw` (lines 66-89), the `Gateway` class from line 90 to end.
  Backoff with jitter, bounded reconnects, then a loud failure, never a
  silent stall.
- Port the thread shape from `src/chat/threads.ts` lines 41-327
  (`ChatThread`, `plain` constructor) and the factory at lines 327-end.
  One thread per session; replies in the thread are prompts; `!!!`
  asides stay in the thread and never reach the agent. Flue's Discord
  blueprint (`blueprints/channel--discord.md` in sweep 4) uses HTTP
  interactions verification plus REST via `@discordjs/rest` with no
  Gateway long-lived connection for outbound calls; take that shape.
- Port inbound/outbound from `src/chat/inbound.ts` (144 lines) and
  `src/chat/outbox.ts` (141 lines): inbound parsing and admission,
  outbound queuing with ordering preserved.
- Discord is the first adapter (serenity). The seam (`ThreadPort`,
  `ThreadFactory` shapes from errand `src/session/manager.ts` lines
  37-66) is what sessions program against; no session code names
  Discord.

⛔ What it must not do: let the chat token enter a sandbox (sessions get
handles, never the token); retry without bound; deliver out of order.

### Prove

```bash
cargo test -p egi-chat
```

Passing: reconnect tests (delays match policy, giving up is loud);
thread tests (aside filtering, ordering); seam test (a fake adapter
drives a session, no network).

### Closing

Not closed.

---

## T-014: Thread commands, diffs, and redaction before delivery

**Source:** errand `src/chat/commands.ts` plus `diff.ts` plus `render.ts` plus `src/session/redacted.ts` plus `src/config/redact.ts` (`634b7b9`).
**Category:** chat
**Priority:** P2
**Effort:** M
**Status:** open

---

### Problem

Threads need commands (`!help`, `!usage`, `!model`, `!status`), readable
diffs of agent changes, and scrubbing: every secret must be redacted
before anything reaches a channel or a transcript.

### Premise

READ, not measured. errand implements each; the entry ports the three
together because redaction wraps every delivery path.

### Approach

Build in `crates/egi-chat`:

- Port commands from `src/chat/commands.ts` lines 46-156
  (`CommandRegistrationError`, `buildCommands`, `TranslatedCommand`,
  `translate`, `applicationId`). Keep the command set: help, usage,
  model switch keeping the conversation, status with delegation costs,
  facts/forget without a session. Slash-command registration mirrors
  the `!` commands.
- Port diff rendering from `src/chat/diff.ts` (140 lines) and message
  rendering from `src/chat/render.ts` (415 lines). Read both whole
  before porting; render interleaves assistant text with tool activity
  in the order the agent produced it.
- Port redaction from `src/config/redact.ts` lines 15-77 (`REDACTION`,
  `redactConfig`, `secretValues`, `redactText`) and
  `src/session/redacted.ts`: one redaction helper, the only path by
  which a secret-shaped value reaches output, with a check that nothing
  else formats those fields. Scrub before channel delivery AND before
  transcript write. This is damage control on an exposure, not a
  boundary: a re-encoded key defeats it, which is why the boundary is
  the broker (T-001).
- Emoji discipline from errand's chars module (`src/chat/chars.ts`):
  chat output may use enumerated emoji only, each carrying a state,
  none as decoration. Logs stay ASCII.

⛔ What it must not do: deliver unscrubbed text to a channel or a
transcript; narrate code in comments; use emoji outside the enumerated
set.

### Prove

```bash
cargo test -p egi-chat
```

Passing: command tests (each command routes, unknown command helps);
render tests (interleave order, diff readability); redaction tests
(every secret value scrubbed on both paths, re-encoded value noted as
out of scope in the test name).

### Closing

Not closed.
