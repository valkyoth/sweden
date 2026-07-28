# Sweden 0.1.0 Release Notes

Status: release candidate; pentest passed; awaiting GitHub

## Scope

This release initializes the dependency-free Sweden workspace, its two
immediately required public crates, dual licensing, security policies, CI,
documentation, and release-planning baseline.

## Crates

- `sweden-core`
- `sweden`

Both crates are intended for crates.io. Future crates are created and published
only when their implementation milestone begins.

The `sweden` facade follows the repository tag. Subcrates advance and publish
only when required; the release helper validates version policy and publishes
selected crates in dependency order. At `1.0.0`, all then-current crates
converge to `1.0.0`.

## Security

- No third-party project dependencies.
- Unsafe Rust is forbidden.
- No agency or transport crate is present in this release.
- No network, TLS, credential, parser, cache, or hosted relay implementation.
- Explicit response-budget validation is present in `sweden-core`.
- Publishing requires an exact tag check when invoked with `--require-tag`.
- Source descriptors prevent contradictory stable/unreviewed metadata and use
  private, forward-compatible fields.

## Verification Required

```bash
scripts/checks.sh
scripts/release_0_1_gate.sh
cargo deny check
cargo audit
```

The implementation baseline and source-descriptor remediation passed the
maintainer pentest and retest. The permanent report is `Status: PASS`. This
release now waits for green GitHub Actions and CodeQL default setup; tagging
still requires an explicit maintainer request.

## Known Limitations

No real Swedish API operation is implemented. This release is not suitable for
production API access and does not make upstream compatibility claims.
