# Sweden Modularity Policy

Sweden must not become a monolithic facade or agency implementation.

Rules:

- Main crate `sweden` is feature wiring and public re-exports.
- Shared contracts live in focused shared crates.
- Each agency or upstream platform owns a separate crate.
- Agency and synthetic conformance crates may depend inward on the focused
  core, policy, and codec crates they need; they do not depend on the facade,
  registry, executor, HTTP, or one another.
- `sweden-policy` owns source-independent policy algorithms and contracts,
  including closed response-outcome, quota/data-access scope, and
  data-handling/access-rebind-limit vocabulary, never generated agency/source
  membership or production partition identity.
- `sweden-registry` owns generated source-specific entries and may depend
  one-way on a selected agency or conformance crate behind a matching feature;
  source crates never depend back on the registry.
- `sweden-registry` also owns opaque freshness epochs and privately binds
  caller-authority state into non-serializable epoch-specific observations.
- `sweden-registry` binds each operation's exact status/outcome profile and
  generated `QuotaScope`, `DataAccessScope`, global anonymous/provider
  partition recipe, `DataHandlingProfile`, finite `AccessRebindLimit`, and
  opaque full-shareability `CacheFillIdentity`; callers cannot replace them
  with generic handlers, restart ledgers, partial fill keys, or free-form
  production partitions.
- `sweden-executor` owns generic execution only. Synthetic operation,
  encoder, decoder, validator, fixture, and output semantics live in the
  separate `sweden-conformance` crate.
- `sweden-executor` alone orchestrates non-secret credential/access binding,
  explicit blocking/async/`NoCache` store contracts, bounded cache lookup,
  post-wait access revalidation with one execution-scoped
  `AccessRebindLedger` and the unchanged parent cache-work ledger, closed
  entry-trust/cache-time decisions, validator insertion, optional fenced
  revisioned fill publication, quota scope, late one-use secret
  materialization, registered response outcomes, and successful final
  provenance.
- Initial provider binding is an executor-owned establishment step and does not
  charge `AccessRebindLedger`. Every later provider assertion pre-charges it
  before provider access; a changed-partition result then pre-charges the
  unchanged parent cache-work ledger. The executor alone fixes the resulting
  `AccessUnstable`-before-access and `CacheWorkExhausted`-after-change
  precedence.
- `sweden-core` owns canonical closed request-header categories and structural
  completion/generative attempt vocabulary plus generic non-cloneable restart
  ledger mechanics only; it does not choose access-rebind policy, and
  constructing a structural attempt brand or ledger grants no execution or
  completion authority.
- `sweden-http` owns the opaque attempt-branded wire witness, each codec owns
  its own opaque attempt-branded syntax witness, `sweden-registry` owns branded
  semantic-witness construction through the bound validator, and
  `sweden-executor` establishes each higher-ranked attempt scope and alone
  owns same-brand finalization/provenance.
- `sweden-http` defines normalized response metadata and adapter-delivered
  `BodyWireBytes`; it does not claim TLS/HTTP framing or total network
  bandwidth ownership.
- Credential providers own opaque binding tokens, quota/access partition IDs,
  session epochs, generations/expiries, access revalidation, and one-use
  `SecretLease` materialization; cache stores own storage mechanics, revisions,
  fences, and declared capacity only and cannot decide access, fill
  shareability, provenance, freshness, or handling policy or deserialize
  `Finalized<R>`.
- Agency crates own generated field sensitivity/handling metadata but cannot
  broaden the registry-bound operation profile.
- Transport crates do not own agency semantics.
- `lib.rs` and binary entry points remain orchestration.
- Parsing, validation, policy, generated models, hand-written facades, tests,
  fixtures, and transport adapters stay separate.
- Every Rust source file, generated or handwritten, must not exceed 500 lines.
- Review splitting once a Rust file approaches 300 lines.
- Generated Rust must be deterministic, declared by a source manifest, clearly
  marked, and split by upstream object family.
- Facade features align the compatible agency, registry, and executor graph;
  no optional-dependency combination may create a cycle or duplicate generic
  execution authority.
- Feature flags must not silently enable networking, credentials, filesystem,
  telemetry, or hosted relaying.

The local gate is:

```bash
scripts/validate-modularity-policy.sh check
```
