# Sweden Modularity Policy

Sweden must not become a monolithic facade or agency implementation.

Rules:

- Main crate `sweden` is feature wiring and public re-exports.
- Shared contracts live in focused shared crates.
- Each agency or upstream platform owns a separate crate.
- Agency crates depend inward on `sweden-core`; they do not depend on the
  facade or on one another.
- Transport crates do not own agency semantics.
- `lib.rs` and binary entry points remain orchestration.
- Parsing, validation, policy, generated models, hand-written facades, tests,
  fixtures, and transport adapters stay separate.
- Non-generated Rust source files must not exceed 500 lines.
- Review splitting once a Rust file approaches 300 lines.
- Generated Rust must be deterministic, declared by a source manifest, clearly
  marked, and split by upstream object family.
- Feature flags must not silently enable networking, credentials, filesystem,
  telemetry, or hosted relaying.

The local gate is:

```bash
scripts/validate-modularity-policy.sh check
```
