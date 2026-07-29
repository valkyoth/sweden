# Sweden Threat Model

Status: foundation model extended for the planned registry/executor boundary

## Assets

- Caller and upstream credentials.
- Correct source, origin, operation, schema, terms, and rate policy.
- Integrity of the closed registry and the indivisible binding between a
  canonical request, encoder, response profile, decoder, semantic validator,
  output/provenance type, limits, evidence, and finalization.
- One-use authorization, quota/concurrency lease, fencing, and completion
  state.
- Producer-owned wire, codec, semantic, and final completion witnesses and
  their invariant per-attempt, codec, registry/version, operation,
  environment, origin, response-profile, and validator bindings.
- Integrity of the closed request-header categories, protected slots,
  canonical/cache identity dimensions, and transport-owned framing.
- Integrity of status-specific body/`304`/empty/redirect/source-error outcome
  dispatch and the exact prior finalized cache identity required by `304`.
- Dossier-generated quota scope, provider-owned opaque credential-pool
  identity, and continuity across shared-pool rotation/aliases.
- Dossier-generated data-access scope, authoritative entitlement partition,
  cache identity/store isolation, and bounded collision-candidate work.
- Cache-entry provenance trust, non-deserializable `Finalized<R>`, exact-match
  uniqueness, `CacheEpoch`/trusted-expiry freshness, and bounded persistent
  capacity/partition cardinality.
- Provider-session binding epoch/generation integrity, one-use non-secret
  binding tokens, and current same-partition access assertion at cached return.
- Full cache-fill shareability identity, monotonic publication fencing,
  entry-revision compare-and-swap, and idempotent publication/release.
- Non-secret credential binding identity/generation/expiry and matching
  short-lived one-use secret materialization.
- Registry-bound executable data-handling profile preserved in finalized
  provenance.
- XML computational-work ledger and parser progress.
- Honest meaning of adapter-delivered body-wire bytes and normalized response
  metadata.
- Freshness requirements and the ordering of policy revalidation, non-secret
  binding, cache resolution, access revalidation, quota reservation, late
  secret/injection, commit, and transport handoff.
- Non-serializable current-authority observations and their originating
  monotonic clock/session epoch.
- Integrity and provenance of official data.
- Personal, sensitive, confidential, and security-relevant response data.
- Official fixture bodies and their classification, retention,
  redistribution, evidence-expiry, and replay metadata.
- Tenant isolation in future hosted services.
- Release artifacts, source snapshots, fixtures, and dependency integrity.

## Adversaries

- A caller supplying malicious paths, filters, payloads, budgets, or cursors.
- A downstream crate implementing structurally valid operation/decoder traits,
  forging registry-shaped values, substituting a decoder or output type, or
  reusing authorization after consumption.
- A caller retaining valid authorization until after expiry/revocation,
  downgrading its freshness requirement, or skipping/reordering time-of-use
  policy, binding, cache, quota, secret, injection, and attempt states.
- A downstream crate forging a completion witness, substituting JSON/XML or
  cross-execution witnesses—including valid witnesses from concurrent
  same-operation attempts—or bypassing the registry-owned validator.
- A caller injecting raw, duplicate, hop-by-hop, framing, control-bearing, or
  representation-changing request headers outside reviewed identity rules.
- A caller or transport reclassifying `304`, empty, redirect, or source-error
  responses as body success, or mixing a cached value/validator/partition/
  version/`Vary` identity.
- A caller forging or multiplying quota partitions through new clients,
  operations, cache partitions, credential rotation/aliases, or secret-derived
  keys.
- A caller/store forging, merging, or substituting data-access partitions,
  sharing private entries across credentials, or amplifying cache collision
  work through poor hashes/excess candidates.
- A store deserializing or fabricating authority-bearing finalized entries,
  returning duplicate exact candidates, replaying a prior cache epoch, lying
  about authenticated persistence/expiry, exploiting clock rollback/skew, or
  exhausting entries, bytes, validators, partitions, eviction, or purge work.
- A provider reusing a binding epoch/generation or consumed token after
  restart, reset, wrap, or ABA; revoking/changing entitlement while cache
  lookup/fill/`304` waits; or becoming unavailable before protected return.
- Concurrent callers stampeding one lawful cache miss, retaining quota/secrets
  while waiting, or exploiting cancelled/expired cache-fill leadership to
  bypass fencing.
- An expired fill leader publishing after takeover, a store accepting a stale
  fence or revision, concurrent `304` updates overwriting each other, or a
  partial fill identity coalescing unrelated namespaces/policies/results.
- A provider expiring, revoking, rotating, or changing identity between
  non-secret binding and secret materialization, or retaining a secret lease.
- XML engineered for quadratic namespace/QName/attribute/end-tag/reference
  work or unchanged-state spin despite finite byte limits.
- A compromised, malicious, or unexpectedly changed upstream service.
- An attacker attempting SSRF or credential exfiltration through redirects,
  origins, proxy settings, errors, fixtures, or logs.
- An upstream payload designed to exhaust memory, CPU, stack, time, or storage.
- A tenant attempting to access another tenant's cache, quota, credential, or
  audit records.
- A supply-chain attacker targeting dependencies, schemas, generators, CI, or
  release credentials.
- A user attempting to bypass source terms, access rules, quotas, attribution,
  or redistribution controls.
- A transport, clock, quota/policy authority, credential provider, cache, or
  allocator that lies, stalls, over-admits, rolls back, leaks, retains, or
  consumes more physical memory than requested.
- A cache, quota, credential, or policy future that never wakes or blocking
  operation that never returns while holding leases or secret material.
- A transport fabricating response status/framing/singleton/trailer metadata
  or under-reporting body/network bytes.
- A caller treating provisional streaming events as committed before final
  envelope validation.
- A caller copying, logging, retaining, transforming, or redistributing
  decoded data after it crosses the SDK boundary.
- A cursor-cycle attacker relying on hash collisions or exhausting bounded
  history so paging repeats indefinitely.
- A recorder or replay consumer attempting to retain protected official data,
  scrub personal data best-effort, use expired/mismatched fixture evidence, or
  keep official corpus bytes after retention permission expires.
- An event sink callback copying data, blocking forever, panicking, consuming
  unbounded CPU, or returning misleading application errors.
- A process reusing cached authority observations after restart, monotonic
  reset/wrap, or clock/session epoch replacement.

## Trust Boundaries

- Caller input to validated agency operation.
- Operation to executable source policy.
- Source-independent policy decision and source-specific generated entry to
  registry-created `AuthorizedExecution<R>`.
- Bound canonical request/encoder/decoder/validator/output package to the
  generic executor.
- Closed operation/header schema through canonical/cache identity, late
  protected-slot filling, and transport-owned framing to wire handoff.
- Selected cache entry through executor-only validator insertion and exact
  `304` revalidation.
- Opaque in-process cache entry or externally authenticated persistent record
  through entry-trust, exact-match, cache-time, and private reconstruction
  checks to a reusable finalized value.
- Earlier credential binding through cache/network waits to a fresh
  same-partition provider assertion and private `AccessRevalidated<R>` before
  protected cached return.
- Provider/registry-derived access binding through bounded cache lookup,
  current hit permission, and store replacement/purge.
- Retained authorization to immediate policy freshness revalidation.
- Policy-revalidated execution to late quota reservation, credentials,
  authority-local attempt commit, the crash gap, and caller transport
  invocation.
- Credential selection and opaque provider pool identity to registry-generated
  quota scope and coordinated authority admission.
- Non-secret provider binding through cache/quota wait and final revalidation
  to matching one-use `SecretLease` and immediate injection.
- Executor request plan to caller-supplied transport.
- Closed origin selection to late credential injection.
- Upstream bytes to bounded codec.
- Adapter-normalized response metadata and `BodyWireBytes` to status-specific
  outcome dispatch.
- Decoded syntax to source semantic validation.
- Provisional stream events through the producer-owned witness chain to
  executor-owned finalization.
- HTTP to private wire completion, codec to its private syntax completion,
  registry validator to semantic completion, and executor-established
  invariant attempt scope to final provenance.
- Borrowed event delivery to the caller-owned sink callback and its
  continue/pause/stop/abort decision.
- Raw source data to normalized or transformed data.
- Source result to cache and hosted response.
- Registry-bound data-handling profile to every Sweden-owned cache, fixture,
  transform/export, retention/purge, diagnostic, and sensitive-field path.
- Tenant context to credentials, limiter, cache, and audit storage.
- Official schema/terms input to checked-in reviewed snapshot.
- Cargo tools and GitHub Actions to release evidence.
- Local ledger to coordinated quota/concurrency authority, including lease
  acquisition, fencing, release, expiry, cancellation, and restart.
- Registry-produced full `CacheFillIdentity` to optional fenced cache-fill
  admission, then store-issued fence/revision through leader expiry/
  cancellation, bounded takeover, atomic publication, waiter release, or
  closed stale-fence/revision rejection.
- Source response to fixture classification/retention decision and replay
  evidence validation.
- Authority observation to the originating monotonic clock/session epoch.

## Baseline Mitigations

- No arbitrary production host.
- No concrete network or credential path at foundation.
- No unsafe Rust or third-party project crate.
- `no_std` core and facade boundaries; future agency boundaries inherit this
  requirement.
- Explicit budgets and fail-closed validation.
- Source and operation-specific policy.
- Source-independent policy code separated from generated source-specific
  registry entries.
- Private, non-cloneable `AuthorizedExecution<R>` construction that embeds the
  exact reviewed encoder, response, decoder, validator, output, limit, and
  finalization profiles; the executor accepts no arbitrary replacement.
- Closed typed request-header categories with canonical case/duplicate rules,
  protected late credential and cache-validator slots, dossier-bounded caller
  metadata, transport-owned framing, CRLF/control and hop-by-hop rejection,
  and exact canonical/cache or reviewed `Vary` participation.
- Cache validators originate only in an already-selected entry, are inserted
  late, stay outside its base key, and bind exact partition, versions, and
  reviewed `Vary`; `304` revalidates current cache permission and prior
  `Finalized<R>` instead of minting semantic completion.
- `DataAccessScope` is distinct from quota: anonymous public partitions are
  registry-owned, credentialed partitions are provider entitlement IDs,
  local namespaces only narrow, and changed entitlement changes partition.
- Credential bindings carry an invariant non-serializable provider-session
  epoch, in-epoch generation, expiry, and a non-cloneable one-use token.
  Restart/reset/wrap/replay/mismatch reselects; every protected cached return
  revalidates current provider access, requires the same partition, and
  restarts lookup or denies on change/unavailability.
- Explicit `NoCache` plus blocking/async store contracts bound candidate count
  and comparison work, require atomic complete replacement/purge and safe
  errors, and leave permission/identity decisions in the executor.
- Built-in provenance-preserving cache entries retain opaque
  non-serializable `Finalized<R>` only within one `CacheEpoch`. Persistent/
  shared provenance is a separately declared authenticated external authority;
  untrusted bytes are reparsed as input and cannot satisfy `304` or claim
  source provenance.
- Full-identity comparison admits exactly one match; duplicates fail closed.
  Ephemeral age cannot cross epochs, authenticated persistence supplies trusted
  absolute expiry and rollback-resistant sequence, malformed/future time
  denies, and upstream cache metadata cannot broaden dossier freshness by
  default.
- Store capacity covers total entries/bytes/key/validator/partition
  cardinality and eviction/purge/cleanup work. `StoreFull` is stable, ordinary
  insertion failure preserves live success, and forbidden-data purge failure
  is surfaced as a policy/storage violation.
- Optional fenced cache-fill admission elects one filler without quota or
  secret leases in waiters. Registry-produced fill identity covers every
  shareability dimension; publication atomically checks a monotonic fence,
  `304` updates CAS the selected revision, cancellation/expiry allows bounded
  takeover, stale leaders cannot write, release/publication is idempotent, and
  no cross-process coalescing is claimed without coordination.
- Closed `CompiledUntil`/`CurrentAuthorityRequired` freshness modes rechecked
  immediately before credentials/I/O and after waits, redirects, and page
  transitions; callers can tighten but never downgrade them.
- The direct SDK documents the residual race between final revalidation and a
  caller transport call; atomic revocation requires a controlled broker or a
  separately admitted authority-issued one-attempt grant.
- Registry-owned `FreshnessEpoch` and privately constructed
  `AuthorityObservation<'epoch>` prevent caller authority state from becoming
  a reusable bound observation. They cannot cross sessions; reset, wrap,
  restart, or mismatch re-observes or fails closed.
- Authorization binds a quota requirement rather than a lease. Late
  two-phase reservation, authority-local attempt commit, conservative
  commit/call crash gap, and fenced at-most-once release distinguish
  credential failure from an in-flight ambiguous attempt.
- Registry-bound `QuotaScope` recipes replace free-form production
  partitions. Provider-owned opaque pool IDs remain stable across rotations
  and aliases sharing upstream capacity, secrets are never key material, and
  source/deployment/IP or other cross-client scope requires coordination.
- Credential binding contains no secret bytes. After cache miss/quota wait,
  final policy revalidation precedes a matching generation/partition/expiry
  checked one-use secret lease and immediate injection; mismatch cancels and
  restarts without sending.
- Total-deadline budget reaches cache, policy, quota, credential, and transport
  waits. Cooperative mode cannot force external progress; cancellation drops
  uncommitted fenced leases at most once, never retains a secret lease, and
  preserves committed/in-flight ambiguity.
- Official network execution prohibited through `v0.36.0`; full reviewed
  rate/window/concurrency enforcement is required beginning at `v0.37.0`.
- Separate synthetic `sweden-conformance` source rather than test semantics in
  the generic executor.
- Provisional stream values cannot create complete provenance, cache entries,
  or advanced checkpoints.
- Closed response outcomes prevent body, `304`, reviewed empty, redirect, and
  source-error branches from substituting for one another; only admitted
  success branches produce or return finalized success provenance.
- Core owns structural completion vocabulary only. HTTP, each codec, registry,
  and executor privately construct their exact wire, codec, semantic, and
  final witnesses. An executor-established higher-ranked invariant brand binds
  them to one attempt; cross-codec and concurrent same-operation substitution
  fails.
- Borrowed events cannot escape synchronous visitor callbacks. Closed
  continue/pause/stop/abort results preserve provisional state correctly, and
  only exact producer witnesses can create `Complete` provenance.
- XML work is pre-charged for namespace traversal, QName/common-prefix/end-tag
  comparison, duplicate expanded attributes, and references; each parser step
  consumes, emits, needs input, or exhausts a ledger.
- Fixture recording is synthetic-only by default. Official public fixtures
  require explicit retention and redistribution evidence; personal/sensitive
  recording fails closed and replay revalidates bound evidence.
- `ConformanceReplay` requires current matching evidence; `CorpusReplay` is
  untrusted bytes only and cannot authorize I/O, provenance, cache, or
  checkpoint state. Corpus use still revalidates retention and purges or denies
  official bytes after permission expiry/withdrawal.
- `DataHandlingProfile` is carried by authorization/finalized provenance and
  consumed by all Sweden-owned cache/fixture/transform/export/retention/
  diagnostic paths; unknown or contradictory handling fails closed.
- Paging stores exact bounded cursor identities and stops when history
  capacity is insufficient; hashes never decide equality.
- Payload-free logging by default.
- Human review for upstream schema or terms changes.
- Pinned toolchain and GitHub Action revisions.
- Maintainer pentest and a current versioned report committed with the release
  work before every tag.

## Residual Risks

- `no_std`, dependency-free code, and safe Rust do not prevent logic errors.
- A caller-supplied transport remains trusted to enforce TLS and origin
  requirements.
- A clock cannot preempt a blocking transport, store/provider/authority call,
  or never-waking future by itself; deadline guarantees depend on the declared
  runtime mode and external component cooperation.
- Caller event sinks can block, panic, copy data, or consume arbitrary CPU.
  Portable `no_std` code cannot catch panics or guarantee callback cleanup.
- Policy may be revoked after the final authority check but before a
  caller-owned transport sends; direct SDK revalidation is deliberately not
  described as atomic revocation.
- Quota commit is atomic only inside the authority, not with external network
  transmission; a crash after commit but before the transport call
  conservatively spends an unsent attempt.
- Caller-provided quota, policy, credential, cache, and allocator
  implementations can invalidate guarantees assigned to those boundaries.
- An arbitrary cache store may retain/cross/fabricate entries or ignore purge,
  lie about trust/time/capacity/fences/revisions, ignore purge, or stall
  forever, and an arbitrary credential provider may lie about session epochs,
  generations, binding/entitlement, retain secret material/tokens, or stall
  forever; traits do not sandbox either.
- `BodyWireBytes` excludes TLS, HTTP transfer/framing, headers,
  retransmissions, and other network overhead; actual bandwidth enforcement
  remains a transport/deployment guarantee, and arbitrary transports may lie
  about metadata or counts.
- Owned allocation limits cover logical/requested/observable container
  metrics, not allocator metadata, fragmentation, or physical heap usage.
- Source policy metadata can be wrong or become stale.
- An offline old binary cannot learn a newly published revocation. Without a
  required trusted monotonic authority, only compiled expiry limits it.
- Official data can contain malicious, misleading, or personal content.
- Once decoded data is returned, Sweden cannot prevent caller code from
  copying, logging, retaining, transforming, or redistributing it.
- Compilation on a platform does not prove runtime networking behavior there.
- Foundation tests do not establish production readiness.
- Future hosted tenant threats remain documented but outside the pre-1.0 SDK
  product and do not justify adding tenant/gateway surfaces early.

Residual risks are narrowed release by release and must remain visible in
release notes.
