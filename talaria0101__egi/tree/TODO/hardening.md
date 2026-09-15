# Hardening

## T-023: Landlock ABI gating and fail-closed version checks

**Source:** bailey `backend/probe.rs` plus `cli.rs` doctor (`d3c73f7`), malaria `research/wsl-kernel.md` (`499e897`).
**Category:** hardening
**Priority:** P2
**Effort:** S
**Status:** open

---

### Problem

Landlock "present" is not one thing. Filesystem policy needs ABI 1;
network rules need ABI 4 (Linux 6.7); ioctl needs ABI 5; abstract
sockets and signals need ABI 6. A yes/no parse admits a kernel that
silently skips the network half of the policy.

### Premise

READ, not measured. bailey documents the ABI ladder; malaria measured
the WSL branches carrying each rung.

### Approach

- In `egi-sandbox` (T-004): parse the ABI number from the probe output,
  never a yes/no. Bailey's own `Capabilities` (`backend/probe.rs:41-71`:
  `landlock_abi: Option<u32>`, `landlock_network()` at ABI 4,
  `landlock_scope()` at ABI 6) plus its doctor (`cli.rs:337-353`, which
  prints `landlock: ABI {abi}` with the consequence of each gap) is the
  shape to mirror. Network policy requires ABI 4 or newer; anything
  below refuses networked sessions rather than running with the egress
  policy silently skipped. Fail closed on `EOPNOTSUPP`: a kernel that
  cannot answer the version query cannot prove it enforces, so it does
  not run.
- Record the ladder in `docs/architecture.md` limits section: ABI 1
  filesystem (any WSL2 branch), ABI 4 network (6.7+), ABI 5/6 only on
  recent branches. Note the `lsm=` boot requirement beside it.
- Nested domains for subprocess gating (broker socket access, read-only
  toolchain against persistence) land with the bailey driver; this entry
  is the version gate that makes the rest honest.

⛔ What it must not do: negotiate down silently; match `landlock:` with a
substring that passes `ABI 2`; run a networked session on an unproven
kernel.

### Prove

```bash
cargo test -p egi-sandbox
```

Passing: ABI matrix tests (1 filesystem-only, 2-3 refuse networked, 4+
admit, unparsable refuses, EOPNOTSUPP refuses); architecture doc states
the ladder.

### Closing

Not closed.

---

## T-024: Exfiltration-through-authorized-channels mitigations

**Source:** the research prompt's open question 8, errand egress design.
**Category:** hardening
**Priority:** P2
**Effort:** M
**Status:** open

---

### Problem

Brokering stops credential theft but not misuse of granted access: an
agent allowed to write to a host can push private data there without
ever seeing a token. This is the residual problem and likely the real
one.

### Premise

READ, argued, not measured. No complete technical fix exists; the entry
implements the mitigations that are honest and refuses the theater.

### Approach

- Per-task-phase allowlists: narrow the egress allowlist as the task
  moves (plan with package registries, implement with the code host
  only, publish with nothing). The session manager (T-012) owns the
  phase; the broker (T-001) enforces the narrowed set.
- Human approval gating for consequential writes: pushing to a public
  host, opening a PR, publishing a package. The chat layer (T-014)
  surfaces the request; the daemon holds the turn until answered.
  Agent-vault issue 407 (per-service MITM request filter sidecar: inspect
  after match, 403 or proceed with credentials) is the same seam at the
  broker layer; take the shape (filter after match, before injection).
- Per-request authorization at the broker: log every allowed write with
  session, binding, size, and time, so misuse is attributable.
- Say plainly in `docs/credential-brokering.md` known limits what does
  NOT work: LLM intent checking (bypassable), egress content scanning
  (encoding defeats it), output masking (theater). Keep those sentences;
  a future session must not re-adopt them as solutions.

⛔ What it must not do: claim scanning or masking as a boundary (agent-vault
issues 258 and 304: redact values from headers and parseable bodies, and
document echoing upstreams as unsupported); gate
one path into a write and leave a sibling open (the door sweep,
`docs/methodology/reviews.md` lens 1).

### Prove

```bash
cargo test -p egi-daemon
```

Passing: phase-narrowing tests (allowlist shrinks per phase, widened
request refused); approval tests (consequential write holds without
approval, proceeds with it); docs state the non-solutions.

### Closing

Not closed.

---

## T-025: gVisor/Kata backend shape behind the same trait

**Source:** kubernetes-sigs agent-sandbox (`809d3ed`), CubeSandbox security-proxy plus network-policy guides (`eaddce3`), sweep 4 (`docs/history/references/orchestration/`).
**Category:** hardening
**Priority:** P3
**Effort:** M
**Status:** open

---

### Problem

The design shares the host kernel. At some threat level that stops being
enough, and the answer must already have a seam to land in rather than a
rewrite to wait for.

### Premise

READ, not measured. Both references run stronger isolation under an
orchestration layer; egi needs only the backend shape, not their models.

### Approach

- Define the shape, do not implement it: a `Gvisor`/`Kata` backend
  implementing the T-003 trait, with the same two-network plus broker
  design (agent netns without egress route, proxy sidecar holding real
  tokens, identity bound per session). Agent-vault issue 382 (split
  management and MITM listeners across interfaces so the harness cannot
  reach management) applies to the sidecar shape from day one.
- Read agent-sandbox's RuntimeClass delegation (README "Architecture") and CubeSandbox's
  per-sandbox traffic tokens plus policy-routing egress (`docs/guide/route-aware-egress.md`,
  270 lines) before writing
  the shape. Note what changes: image and runtime class selection,
  snapshot/clone via the microVM substrate, startup latency budget.
- State the trigger plainly in `docs/architecture.md`: shared-kernel
  suffices while tenants are the operator's own sessions; mutually
  hostile tenants or kernel-exploit exposure move a deployment to this
  backend.

⛔ What it must not do: implement the backend now (no consumer exists);
adopt the CRD orchestration model (egi stays a single-host daemon).

### Prove

```bash
cargo test -p egi-sandbox
```

Passing: shape test (the enum variant exists, probe reports a clear
unimplemented-with-reason, docs state the trigger).

### Closing

Not closed.

---

## T-026: Deep review pass over the skeleton and the plan

**Source:** `docs/methodology/reviews.md`, `docs/methodology/gate.md`.
**Category:** hardening
**Priority:** P3
**Effort:** S
**Status:** open

---

### Problem

A skeleton plus 26 entries written in one session carries misreadings.
Three lenses catch different classes; one sweep written up three times
catches one.

### Premise

No premise to check. This entry is the review itself.

### Approach

Run three passes per `docs/methodology/reviews.md`, each naming what it
swept that the others did not:

1. Door sweep: every new affordance (crate, check, doc, workflow),
   every caller and surface reaching it, grep for the unenumerated.
   The question: what other door reaches this code.
2. Guard mutation: plant each guard's defect (non-ASCII byte, wrong
   count, bad link, tool credit in a message, public bind, forbidden
   flag) and read the exit code unpiped. A guard never seen to refuse
   is theater.
3. Claim audit: every sentence in README, architecture, SECURITY, and
   the entries that asserts a measurement or a behavior, against the
   tree and the references at their pins. A number with the wrong
   denominator, a citation that moved, a promise the code does not keep.

File the pass under `docs/history/reviews/` with findings, fixes, and
what each pass did NOT look at. Fix what it surfaces in the same
change. A pass with no findings names what would have made it fire.

⛔ What it must not do: report one sweep under three headings; list a
finding without fixing it or filing it as an entry.

### Prove

```bash
sh scripts/common/check-gate.sh --strict && cargo test --locked && cargo clippy --all-targets -- -D warnings
```

Passing: the full gate green in strict mode, the review filed, every
finding fixed or filed.

### Closing

Not closed.
