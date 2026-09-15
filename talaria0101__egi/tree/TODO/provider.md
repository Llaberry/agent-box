# Provider

## T-007: Provider registry with provider colon model addressing

**Source:** kage `crates/kage-provider` registry plus provider modules (`6ad2708`), errand `src/provider/models.ts` (`634b7b9`).
**Category:** provider
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

Sessions address models as `provider:model` strings across several
providers. Without one registry, every caller parses strings and every new
provider rewrites every caller.

### Premise

READ, not measured. kage implements this registry in Rust; the port keeps
the trait and the addressing.

### Approach

Create `crates/egi-provider` depending on `egi-core`:

- Port the `Provider` trait at
  `crates/kage-provider/src/lib.rs` line 53, `ProviderMetadata` at
  `crates/kage-provider/src/metadata.rs` line 5, `StreamRequest` at
  `crates/kage-provider/src/request.rs` line 145, `ProviderEvent` at
  `crates/kage-provider/src/event.rs` line 29, `ProviderError` at
  `crates/kage-provider/src/error.rs` line 7.
- Port the registry at `crates/kage-provider/src/registry.rs` lines 1-166:
  `ProviderRegistry`, `register` replacing same-id entries, `resolve`
  splitting `provider:model` and returning `UnknownModel` on a missing
  separator or prefix. Keep the unit tests as the ported suite.
- Read errand `src/provider/models.ts` lines 35-121 (`agentDirectories`,
  `agentDirectory`, `readModels`, `visionModel`) for the model-store
  lookup shape, but do NOT copy its behavior of reading another agent's
  store: egi owns its catalog (T-009). Kage's catalog module
  (`crates/kage-provider/src/catalog/mod.rs`, `ProviderInfo` plus hand API
  over generated data) is the closer model.
- First providers: Anthropic and OpenAI-compatible (which covers ZAI and
  most gateways), then Gemini. Port one provider module at a time from
  `crates/kage-provider/src/anthropic/` and `openai.rs` (726 lines).

⛔ What it must not do: hold a credential in the registry (credentials live
in the broker); resolve a model without a provider prefix by guessing.

### Prove

```bash
cargo test -p egi-provider
```

Passing: registry tests (empty resolves unknown, registered resolves,
missing separator unknown, replace-on-register, id listing); one provider
streaming against a local fixture server, no network.

### Closing

Not closed.

---

## T-008: Streaming client with SSE parsing and usage accounting

**Source:** kage `sse.rs` plus `request.rs` plus `tokens.rs` (`6ad2708`); errand `src/provider/zai.ts` quota gate (`634b7b9`).
**Category:** provider
**Priority:** P1
**Effort:** M
**Status:** open

---

### Problem

Model answers arrive as server-sent event streams. A naive reader that
splits on newlines corrupts multi-line payloads; uncounted usage bills
blindly.

### Premise

READ, not measured. kage owns an SSE parser and token accounting; errand
owns the quota-gate UX (usage window, spent detection).

### Approach

Build in `crates/egi-provider`:

- Read `crates/kage-provider/src/sse.rs` (138 lines: `read_sse_event` frame
  reader, `SseStreamCore` state-machine trait, shared `sse_next` loop) end
  to end before writing. Port the three pieces whole: the reader does full
  SSE framing (Anthropic grammar, a superset of data-only readers), the
  state machine stays provider-specific, the loop drains pending, honors
  cancel, fuses on EOF. Read `request.rs` (321 lines), `tokens.rs` (71
  lines), `event.rs` (113 lines), `error.rs` (146 lines) the same way.
- Port the quota UX from errand `src/provider/zai.ts` lines 21-104
  (`Quota`, `readQuota`, `isSpent`, `fetchQuota`, `QUOTA_TTL_MS`,
  `QuotaGate`): cached quota reads, spent detection, human wording. The
  endpoint differs per provider; the gate shape carries over.
- Cancellation is a flag the stream checks, per kage's `cancelable.rs`.
  A cancelled stream stops promptly and reports what it had produced.
- Every turn records usage (input, output, cache splits where the
  provider reports them) into the transcript. No turn without a usage
  line where the provider reported one.

⛔ What it must not do: buffer a whole stream into memory before yielding;
retry a rate limit without honoring its stated delay and a cap.

### Prove

```bash
cargo test -p egi-provider
```

Passing: SSE parser tests (multi-line events, split chunks, stream end);
usage accumulation test; cancellation test (flag mid-stream stops and
reports partial); quota-gate tests against fixtures.

### Closing

Not closed.

---

## T-009: models.dev catalog generation through an xtask

**Source:** kage `crates/kage-provider/src/catalog/` plus `xtask/src/main.rs` (`6ad2708`).
**Category:** provider
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

Model ids, context windows, and prices drift weekly. A hand table is wrong
within a release; no table leaves pickers and defaults with nothing to
show.

### Premise

READ, not measured. kage generates its catalog from models.dev and
curates it in tree.

### Approach

- Read kage `crates/kage-provider/src/catalog/mod.rs` (the hand API over
  generated data) and `generated.rs` (the `@generated` header convention),
  plus `xtask/src/main.rs` line 8 and line 31 (`refresh-models` fetching
  `https://models.dev/api.json`).
- Add `cargo xtask refresh-models`: fetch, curate to supported providers,
  rewrite the generated file with the header intact. Pin the fetch date in
  the file. The hand API (`ProviderInfo`, `ModelInfo`, cost, context,
  thinking levels) ports from the catalog module.
- Provider impls stay hardcoded per module; the catalog carries only the
  metadata pickers and defaults need.

⛔ What it must not do: fetch at build time (a build reaching the network
is red when somebody else is down); hand-edit the generated file.

### Prove

```bash
cargo xtask refresh-models --check
```

Passing: `--check` exits 0 when the committed catalog matches the
generator's output shape (offline, against a committed fixture), non-zero
otherwise. Plus `cargo test -p egi-provider` green.

### Closing

Not closed.

---

## T-010: Delegate (cheap model) path with budget and deadline

**Source:** errand `src/provider/ask.ts` plus `src/agent/delegate.ts` plus `delegation.ts` (`634b7b9`).
**Category:** provider
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

A capable model burning context on a long log or a large diff pays for
that context on every later turn. A cheap model answering one question
about one artefact keeps it out of the session's context.

### Premise

READ, not measured. errand implements delegation with per-turn budgets;
the shape ports directly.

### Approach

- Port `ask` from errand `src/provider/ask.ts` lines 10-106: `Endpoint`,
  `Question`, `Answer`, `AskFailed`, the instruction prefix (answer from
  the material only, quote exactly, say when it does not answer), caller
  owns the deadline via abort signal, injected sender so tests need no
  network.
- Read `src/agent/delegate.ts` and `src/agent/delegation.ts` for the
  budget shape (per-turn cap, deadline, cost reporting) and port the
  accounting: `!status` reports what delegation cost and how much context
  it kept out.
- The delegate sees one artefact and nothing else: no history, no system
  prompt describing the session, no tools. What comes back is a
  description to check, not a decision to follow.

⛔ What it must not do: give the delegate tools or session context; exceed
the per-turn budget silently; abandon a delegation without reporting it.

### Prove

```bash
cargo test -p egi-provider
```

Passing: ask-success test with injected sender; budget test (over-budget
delegation refused); deadline test (abort fires, `AskFailed` carries
readable words).

### Closing

Not closed.
