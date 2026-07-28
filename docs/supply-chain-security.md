# Sweden Supply-Chain Security

Sweden uses overlapping controls:

- no third-party project crates;
- exact version plus path for first-party publishable dependencies;
- `cargo deny check` for license, source, advisory, wildcard, and duplicate
  policy;
- `cargo audit` for RustSec advisories;
- `scripts/check_latest_tools.sh` for Rust, Cargo verification tools, and
  `actions/checkout` freshness;
- `scripts/check_latest_crates.py` to reject undeclared external project
  dependencies;
- Dependabot for Cargo and GitHub Actions visibility;
- SHA-pinned remote actions;
- no build-time network access;
- checked-in, hashed official schema inputs;
- deterministic code generation;
- isolated `cargo package` checks for every crate;
- release notes and a current versioned pentest report committed with the
  release work.

## Tooling

CI pins:

| Tool | Version | Purpose |
| --- | --- | --- |
| Rust | `1.97.1` | Build, lint, test, docs, package |
| `cargo-deny` | `0.20.2` | Dependency source, license, duplicate, advisory policy |
| `cargo-audit` | `0.22.2` | RustSec advisory check |
| `actions/checkout` | `v7.0.1` full SHA | Repository checkout |

The release gate queries authoritative upstream metadata and fails when a pin
is stale. Tooling freshness does not automatically authorize an update:
changed behavior and MSRV still receive review.

## Upstream Source Inputs

Agency schemas and terms are supply-chain inputs. Maintainer fetches must
record official URL, retrieval time, upstream version, content hash, licence,
and review decision. Compilation never downloads live schemas. Automation may
open a change for review but cannot merge or activate it.

## Release Credentials

Publishing credentials must be least-privilege, short-lived where supported,
absent from pull-request jobs, and scoped to the intended crates. Publication
follows dependency order and happens only after explicit maintainer approval.
