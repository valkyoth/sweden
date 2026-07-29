# Sweden Security Controls

## Active Foundation Controls

- `#![forbid(unsafe_code)]` in every first-party crate.
- Dependency-free `no_std` core and facade crates.
- No default network, TLS, credential, filesystem, clock, or telemetry path.
- Explicit response budgets.
- No agency or transport implementation before its planned review milestone.
- Foundation status prevents production-complete claims.
- Cargo source, license, duplicate, and advisory deny policy.
- Latest-stable and tool freshness release checks.
- SHA-pinned GitHub Actions.
- GitHub CodeQL default setup policy.
- Generated and handwritten Rust source limit of 500 lines.
- Byte-identical repository and facade READMEs.
- Exact-version release notes and pentest stop.

## Required Before Untrusted Parsing

- token, depth, string, collection, and decoded-byte budgets;
- exact-consumption semantics;
- malformed, truncated, oversized, and mutation tests;
- DTD/entity rejection for XML;
- decoded-name duplicate detection for JSON and explicit XML 1.0 lexical,
  QName, namespace, comment, CDATA, processing-instruction, declaration, and
  reference-digit budgets;
- non-escaping borrowed `EventSink` callbacks with closed
  continue/pause/stop/abort decisions, safe error collapse, and no completion
  after pause/stop/abort/panic;
- structural completion vocabulary in core, with private producer-owned HTTP
  wire, codec-specific, registry-semantic, and executor-final witnesses before
  `Complete` provenance;
- invariant per-attempt branding across every completion witness, with
  concurrent same-operation substitution tests;
- closed status-specific completion: body success requires wire/codec/semantic
  proof, `304` requires an exact prior finalized cache entry, reviewed empty
  requires `NoBody`, and redirect/source-error paths cannot create success
  provenance;
- event-boundary-only pause, caller ownership of unconsumed input, bounded
  already-charged decoder carry, next-chunk denial while paused, and exact-once
  resume accounting;
- archive traversal and decompression-ratio controls where applicable;
- no-panic arbitrary input evidence.

## Required Before Network Execution

- closed origin registry;
- closed typed request headers with reviewed static, protected credential,
  cache-validator, bounded dossier-permitted caller metadata, and
  transport-owned framing categories; canonical case/duplicate rules,
  CRLF/control rejection, hop-by-hop denial, and byte/count budgets;
- representation-affecting headers included in canonical/cache identity or
  reviewed `Vary`, with credentials/framing/diagnostics excluded;
- cache validators derived only from the selected cache entry, inserted late
  by executor logic, excluded from the base key, and bound to exact cache
  partition, validator, policy/schema/registry, and `Vary` identity;
- HTTPS-only production policy;
- reviewed redirect rules;
- late credential injection;
- safe proxy policy;
- indivisible registry authorization binding the exact request encoder,
  response profile, decoder, validator, output/provenance type, limits, and
  finalization;
- non-downgradable compiled-expiry or current-authority freshness revalidated
  immediately before credentials/I/O and after waits, redirects, and pages;
- explicit non-atomic revocation boundary between the final revalidation and
  an external transport call; no atomicity claim without a controlled broker
  or separately admitted authority-issued per-attempt grant;
- registry-owned opaque freshness epochs and privately bound non-serializable
  authority observations; restart, reset/wrap, or mismatch re-observes or
  denies;
- distinct authorization, policy-revalidated, quota-reserved,
  credential-selected, quota-reserved, credential-injected,
  attempt-committed, and attempt-in-flight states;
- dossier-generated `QuotaScope` with no caller-created production partition;
  opaque provider pool IDs remain stable across shared-pool rotation/aliases,
  never derive from secrets, and cross-client scopes require coordination;
- authority-local quota commit before the external transport call, with the
  intervening crash gap conservatively spending the attempt;
- explicit deadline mode, retries, and rate budgets without claiming a clock
  can preempt a stalled transport;
- complete reviewed interval/window/concurrency enforcement and fenced lease
  recovery; official network execution remains prohibited through `v0.36.0`;
- response byte accounting before decode;
- `BodyWireBytes` defined as content-coded bytes delivered after TLS/transfer
  framing and before content decoding, with separate metadata budgets and no
  total-network-bandwidth claim;
- conforming-adapter rejection of conflicting length/framing, duplicate
  singleton response metadata, unreviewed informational responses, and
  ambiguous trailers;
- SSRF and credential-destination tests;
- adversarial canonicalization tests for encoded separators/dot segments,
  duplicate query keys, backslashes, Unicode-equivalent forms, fragments,
  scheme-relative locations, and encoded controls.

## Required Before Official Fixture Recording

- synthetic-only recording by default;
- explicit operation-level retention and redistribution permission;
- fail-closed rejection of personal or sensitive body recording;
- source, operation, schema, policy/evidence, retrieval, classification, and
  redistribution metadata;
- `ConformanceReplay` rejection after expiry or any operation/evidence
  mismatch;
- type-distinct `ConformanceReplay` and untrusted `CorpusReplay`; corpus mode
  cannot authorize I/O, current provenance, caches, or checkpoints.
- retention permission rechecked at corpus use, with expired or withdrawn
  official bytes purged or denied even though corpus mode is non-authoritative.

## Required Before Hosted Multi-Tenancy

- explicit tenant context;
- tenant-scoped credentials, cache keys, quotas, and audit events;
- operation allowlist;
- policy expiry and source kill switch;
- payload-free telemetry;
- retention and deletion controls;
- cross-tenant negative tests;
- backup, restore, and incident exercises.
