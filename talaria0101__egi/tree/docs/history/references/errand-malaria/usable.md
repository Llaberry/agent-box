# Sweep 1 usable lines: errand and malaria

Mechanisms with file and line for the session doing the work. Commit pins
in `../pins.md`. Read `findings.md` for verdicts.

## Broker (T-001, T-002)

- errand `src/sandbox/broker.ts:30`: `hostAllowed` wildcard rule (bare
  matches self, `*.` matches subdomains not apex, `*` admits all,
  case-folded). Already ported to `egi-core/src/sandbox.rs`.
- errand `src/sandbox/broker.ts:59`: `parseConnect` refuses unparseable
  targets with `None` rather than guessing. Ported.
- errand `src/sandbox/broker.ts:98-119`: `HOP_BY_HOP` strip list and
  constant-time `sameSecret` nonce check. Port both verbatim.
- errand `src/sandbox/broker.ts:292-340`: byte-at-a-time head reader so
  no TLS byte is consumed and no header byte leaks into the tunnel. Port
  the technique, keep the comment.
- errand `src/sandbox/broker.ts:139-189`: longest-prefix route match,
  nonce check, credential put on at this end, stream back. Port the
  shape; the runtime is tokio, not Deno TCP.

## Sandbox (T-003 through T-006)

- errand `src/sandbox/backend.ts:133-154`: the `Sandbox` trait (probe,
  launch, listOrphans, removeOrphans). Port as the Rust trait.
- errand `src/sandbox/bailey.ts:77-98`: built environment, never
  inherited; daemon-set names win. Keep.
- errand `src/sandbox/bailey.ts:100-131`: parse the tool's own doctor
  output, do not hardcode. Keep, and extend to the ABI number (T-006,
  T-023).
- errand `src/sandbox/podman.ts:47`: `RESTRICTED_NETWORK` with the
  comment at lines 40-47 (measured on podman 5.8.2, pasta 2025.12.15;
  legacy `--no-map-gw` does not close it). Keep the comment with the
  constant.
- errand `src/sandbox/podman.ts:50-60`: `FORBIDDEN_ARGS`. Refuse each.
- errand `src/daemon.ts:88-147`: `renderStartupReport` plus
  `probeSandbox`; gaps stated, refuse unless degraded mode accepted.
  Port the sequence.
- malaria `docs/windows.md` (whole, 209 lines): firewall gap stays
  reported, plain `rw`, keep-id to uid 1000, OOM 137, 11x drive-mount
  cost. Port the report lines.
- malaria `research/wsl-kernel.md` (whole, 62 lines): ABI ladder table
  (ABI 1 everywhere, ABI 4 needs 6.7+, ABI 5/6 newest only). Port as the
  version gate.

## Protocol and sessions (T-011, T-012, T-016)

- errand `src/agent/client.ts:46`: `AgentState` union; settlement from
  `agent_settled`, never release admission on `agent_end`. Port both.
- errand `src/session/record.ts:20-47`: record dir, transcript path,
  prepare. Port the layout.
- errand `src/admission/scheduler.ts:17-74`: `Clock`, `Ticket`,
  `SubmitOutcome`, `QueueEntry`, provider-backoff pause. Port the queue.

## Chat and daemon (T-013 through T-015, T-017)

- errand `src/chat/gateway.ts:32-51`: `ReconnectPolicy`,
  `DEFAULT_RECONNECT`, `reconnectDelayMs`. Port the policy.
- errand `src/web/address.ts:14-63`: public bind refused, not warned.
  Port the verdict.
- errand `src/config/schema.ts:411`: `SECRET_PATHS` beside the schema.
  Keep beside the Rust types.
- errand `src/config/load.ts:19-34`: search order constants. Port.
- errand `src/session/github.ts:65-140`: `ghShimContents`,
  `gitConfigContents`, `gitIdentityEnv`, attribution footer. Re-derive
  each as a broker binding (T-018), do not copy the token-in-env shape.
