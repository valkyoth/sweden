# Sweden 0.2.0 Release Notes

Status: pentest passed; awaiting green GitHub Actions and CodeQL

## Scope

This release makes source, operation, schema, policy, and upstream identity
explicit without adding agency execution, networking, credentials, or parsing.
All new runtime primitives remain dependency-free and `no_std`.

## Crates

- `sweden-core 0.2.0`
- `sweden 0.2.0`

Both crates are planned for publication. The facade follows the repository tag
and updates its exact dependency on `sweden-core`.

## Identifier Contract

- Dynamic identifiers borrow caller storage and allocate nothing.
- Source IDs accept at most 63 bytes, schema and policy IDs 95 bytes, and
  operation and upstream IDs 127 bytes.
- Canonical spelling starts with a lowercase ASCII letter.
- Remaining bytes are lowercase ASCII letters, digits, `-`, or `.`.
- Separators cannot be adjacent or final.
- Each identifier kind has an explicit byte ceiling.
- Input is rejected rather than silently case-folded or normalized.
- Comparison and display use the canonical bytes directly.

`reviewed_sources` contains the closed spellings reserved for Trafikverket,
SMHI, SCB, JobTech, and Skatteverket. `ReviewedSourceId` has no downstream
constructor. These constants prove only project-reviewed spelling; neither
they nor dynamic IDs, source descriptors, or public traits authorize an
operation.

## Versions And Errors

- `SchemaVersion`, `PolicyVersion`, and `UpstreamVersion` reject zero and
  preserve stable numeric comparison and display.
- `ValidationError` carries only bounded categories and never retains rejected
  identifier text.
- Existing source descriptor and response-budget constructors now return
  explicit `Result` errors instead of ambiguous `Option` failures.

## Security

- No third-party project dependencies or unsafe Rust.
- Compile-fail doctests prove downstream code cannot construct a
  `ReviewedSourceId` or obtain execution authority from `sweden-core`.
- Every reviewed-source literal is compile-time checked with the same
  canonical validator used by dynamic identifiers, without enabling panic.
- Boundary tests cover empty, exact-limit, one-over, every ASCII byte,
  malformed separators, non-ASCII input, zero, and maximum versions.
- Exact RFC reference bytes remain checksum pinned and excluded from crate
  packages.

## Verification Required

```bash
scripts/checks.sh
scripts/release_0_2_gate.sh
cargo deny check
cargo audit
```

The implementation baseline, findings, remediation, and green maintainer
retest are recorded in `security/pentest/v0.2.0.md`. GitHub and tagging remain
pending until the normal workflow is completed.

## Known Limitations

No real Swedish API operation is implemented. Identifier validity and a
reviewed source spelling are descriptive only; they do not establish current
policy, dossier evidence, registry membership, or execution permission.
