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
  never generated agency/source membership.
- `sweden-registry` owns generated source-specific entries and may depend
  one-way on a selected agency or conformance crate behind a matching feature;
  source crates never depend back on the registry.
- `sweden-registry` also owns opaque freshness epochs and privately binds
  caller-authority state into non-serializable epoch-specific observations.
- `sweden-executor` owns generic execution only. Synthetic operation,
  encoder, decoder, validator, fixture, and output semantics live in the
  separate `sweden-conformance` crate.
- `sweden-core` owns canonical closed request-header categories and structural
  completion/generative attempt vocabulary only; constructing a structural
  attempt brand grants no execution or completion authority.
- `sweden-http` owns the opaque attempt-branded wire witness, each codec owns
  its own opaque attempt-branded syntax witness, `sweden-registry` owns branded
  semantic-witness construction through the bound validator, and
  `sweden-executor` establishes each higher-ranked attempt scope and alone
  owns same-brand finalization/provenance.
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
