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
- Honest meaning of adapter-delivered body-wire bytes and normalized response
  metadata.
- Freshness requirements and the ordering of policy revalidation, quota
  reservation/commit, credential injection, and transport handoff.
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
  policy, quota, credential, and attempt states.
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
- A transport fabricating response status/framing/singleton/trailer metadata
  or under-reporting body/network bytes.
- A caller treating provisional streaming events as committed before final
  envelope validation.
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
- Retained authorization to immediate policy freshness revalidation.
- Policy-revalidated execution to late quota reservation, credentials,
  authority-local attempt commit, the crash gap, and caller transport
  invocation.
- Credential selection and opaque provider pool identity to registry-generated
  quota scope and coordinated authority admission.
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
- Tenant context to credentials, limiter, cache, and audit storage.
- Official schema/terms input to checked-in reviewed snapshot.
- Cargo tools and GitHub Actions to release evidence.
- Local ledger to coordinated quota/concurrency authority, including lease
  acquisition, fencing, release, expiry, cancellation, and restart.
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
- Fixture recording is synthetic-only by default. Official public fixtures
  require explicit retention and redistribution evidence; personal/sensitive
  recording fails closed and replay revalidates bound evidence.
- `ConformanceReplay` requires current matching evidence; `CorpusReplay` is
  untrusted bytes only and cannot authorize I/O, provenance, cache, or
  checkpoint state. Corpus use still revalidates retention and purges or denies
  official bytes after permission expiry/withdrawal.
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
- A clock cannot preempt a blocking transport or never-waking future by
  itself; deadline guarantees depend on the declared transport/runtime mode.
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
- Compilation on a platform does not prove runtime networking behavior there.
- Foundation tests do not establish production readiness.
- Future hosted tenant threats remain documented but outside the pre-1.0 SDK
  product and do not justify adding tenant/gateway surfaces early.

Residual risks are narrowed release by release and must remain visible in
release notes.
