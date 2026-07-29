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
- Freshness requirements and the ordering of policy revalidation, quota
  reservation/commit, credential injection, and transport handoff.
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
- A caller treating provisional streaming events as committed before final
  envelope validation.
- A cursor-cycle attacker relying on hash collisions or exhausting bounded
  history so paging repeats indefinitely.
- A recorder or replay consumer attempting to retain protected official data,
  scrub personal data best-effort, or use expired/mismatched fixture evidence.

## Trust Boundaries

- Caller input to validated agency operation.
- Operation to executable source policy.
- Source-independent policy decision and source-specific generated entry to
  registry-created `AuthorizedExecution<R>`.
- Bound canonical request/encoder/decoder/validator/output package to the
  generic executor.
- Retained authorization to immediate policy freshness revalidation.
- Policy-revalidated execution to late quota reservation, credentials, atomic
  attempt commit, and caller transport handoff.
- Executor request plan to caller-supplied transport.
- Closed origin selection to late credential injection.
- Upstream bytes to bounded codec.
- Decoded syntax to source semantic validation.
- Provisional stream events to non-forgeable finalized completion.
- Raw source data to normalized or transformed data.
- Source result to cache and hosted response.
- Tenant context to credentials, limiter, cache, and audit storage.
- Official schema/terms input to checked-in reviewed snapshot.
- Cargo tools and GitHub Actions to release evidence.
- Local ledger to coordinated quota/concurrency authority, including lease
  acquisition, fencing, release, expiry, cancellation, and restart.
- Source response to fixture classification/retention decision and replay
  evidence validation.

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
- Closed `CompiledUntil`/`CurrentAuthorityRequired` freshness modes rechecked
  immediately before credentials/I/O and after waits, redirects, and page
  transitions; callers can tighten but never downgrade them.
- Authorization binds a quota requirement rather than a lease. Late
  two-phase reservation, atomic pre-handoff attempt commit, and fenced
  at-most-once release distinguish credential failure from an in-flight
  ambiguous attempt.
- Official network execution prohibited through `v0.36.0`; full reviewed
  rate/window/concurrency enforcement is required beginning at `v0.37.0`.
- Separate synthetic `sweden-conformance` source rather than test semantics in
  the generic executor.
- Provisional stream values cannot create complete provenance, cache entries,
  or advanced checkpoints.
- Borrowed events cannot escape synchronous visitor callbacks; `Complete`
  provenance requires wire, codec, and source-semantic completion.
- Fixture recording is synthetic-only by default. Official public fixtures
  require explicit retention and redistribution evidence; personal/sensitive
  recording fails closed and replay revalidates bound evidence.
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
- Caller-provided quota, policy, credential, cache, and allocator
  implementations can invalidate guarantees assigned to those boundaries.
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
