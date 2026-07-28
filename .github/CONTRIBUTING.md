# Contributing To Sweden

Sweden is security-sensitive public-API infrastructure. Contributions must keep
the workspace small, explicit, source-respecting, tested, and honest about
which operations are stable.

## License

Sweden is licensed under `MIT OR Apache-2.0`. Unless explicitly stated
otherwise, contributions are offered under the same dual license.

## Development Setup

Use the pinned Rust toolchain from `rust-toolchain.toml`.

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
```

Before opening a pull request, run:

```bash
scripts/checks.sh
```

## Security-Sensitive Changes

Treat these areas as high risk:

- source terms, access classes, hosted-use policy, and attribution;
- host selection, redirects, credentials, transport, and retry behavior;
- parsers, decompression, pagination, archives, and resource limits;
- caching, tenant isolation, provenance, logging, and redaction;
- source schemas, generated code, fixtures, CI, and release scripts;
- dependency and tool updates.

Do not post exploitable details in public issues. Follow
[SECURITY.md](../SECURITY.md).

## Dependencies

Project crates admit no third-party dependencies. A proposal to add one must
have its own bounded release-plan entry and include current-version, MSRV,
license, maintenance, feature, source, security, and alternative analysis.
