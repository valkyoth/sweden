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
- `XmlWork` charging for namespace/QName/attribute/end-tag/reference work and
  a consume/event/need-input/exhaust progress invariant;
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
  access partition/local narrowing, validator, policy/schema/registry,
  classification/handling profile, and `Vary` identity;
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
- distinct authorized, policy-revalidated, non-secret
  credential/access-binding-selected, cache-resolved, access-revalidated,
  quota-reserved, final-policy-revalidated, one-use-secret-materialized,
  credential-injected, attempt-committed, and attempt-in-flight states;
- dossier-generated `QuotaScope` with no caller-created production partition;
  opaque provider pool IDs remain stable across shared-pool rotation/aliases,
  never derive from secrets, and cross-client scopes require coordination;
- dossier-generated `DataAccessScope` with registry-global anonymous or
  provider-owned entitlement partitions distinct from quota; caller
  namespaces only narrow and entitlement-changing rotation changes partition;
- invariant non-serializable `CredentialBindingEpoch` with generations valid
  only inside one provider session; binding tokens are non-cloneable,
  non-serializable, and consumed by materialization or terminal cache return;
- fresh provider access revalidation before every credential-partitioned cache
  hit/fill-waiter/`304` return; only the same current `AccessPartitionId` may
  proceed, changed entitlement restarts lookup, and provider unavailability
  denies protected cached data;
- registry-bound finite `AccessRebindLimit` and non-cloneable
  `AccessRebindLedger`, pre-charged before every provider access reselection;
  changed-partition lookup consumes the same parent cache-work ledger;
- `AccessUnstable` on exhaustion with candidate discard and no fallback to an
  earlier partition, entry, or fresh ledger within that execution; A/B
  oscillation, repeated expiry, epoch churn, provider restart, fill waits,
  `304`, and `CacheOnly` all share the same limit;
- explicit `NoCache` default plus blocking/async cache-store boundary with
  bounded collision candidates/comparison work, atomic replacement/purge,
  safe errors, and current permission/access/classification revalidation on
  fresh, stale, cache-only, miss, and `304`;
- closed cache-entry trust: only opaque same-process/same-`CacheEpoch`
  `Finalized<R>` is built-in provenance-preserving storage for 1.0;
  persistent/shared provenance requires an explicit authenticated external
  authority, and untrusted bytes cannot mint finalization or satisfy `304`;
- no generic `Finalized<R>` deserialization, duplicate exact candidates fail
  closed, and candidate order never chooses an authoritative entry;
- closed cache time: monotonic age cannot cross `CacheEpoch`; authenticated
  persistence needs trusted absolute expiry plus rollback-resistant sequence;
  malformed/future timestamps deny, and upstream cache metadata narrows but
  does not broaden dossier freshness by default;
- declared store ceilings for entries, owned/encoded bytes, key/validator
  bytes, access-partition cardinality, and eviction/purge/cleanup work, with
  stable `StoreFull` and distinct insertion-versus-required-purge failure;
- optional fenced full-shareability cache-fill lease whose
  waiters hold neither quota nor credentials and whose cross-process guarantee
  requires coordination;
- registry-produced opaque `CacheFillIdentity` includes local namespace,
  environment/origin, schema/registry/policy, classification/handling,
  representation/`Vary`, and raw/transformed identity as well as canonical
  request and authoritative partition;
- each fill publication atomically checks its monotonic fence; expired leaders,
  coordination restart, reset/wrap, or takeover returns `StaleFence`, while
  `304` updates compare-and-swap the selected `CacheEntryRevision`;
- release and exact same-fence/revision/entry publication are idempotent while
  conflicting repeats fail closed; cancellation after a live response
  preserves that result without authorizing an unfenced write;
- non-secret provider binding before cache/quota and matching short-lived
  one-use `SecretLease` only after quota wait/final policy check; expiry,
  revocation, generation, or partition mismatch cancels and restarts;
- authority-local quota commit before the external transport call, with the
  intervening crash gap conservatively spending the attempt;
- explicit total-deadline mode across cache, policy, quota, credential, and
  transport waits without claiming a clock can preempt stalled blocking work
  or a never-waking future, plus phase-specific fenced cancellation cleanup;
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
- registry-bound `DataHandlingProfile` enforcement for cache, recording,
  transform/export, retention/purge, and sensitive-field diagnostics;
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
