# Changelog

All notable changes to Sweden are documented here. The project follows
Semantic Versioning for each independently published crate.

## Unreleased

## 0.2.0 - 2026-07-29

### Added

- Borrowed, allocation-free `SourceId`, `OperationId`, `SchemaId`, `PolicyId`,
  and `UpstreamId` types with canonical lowercase ASCII grammar and explicit
  per-type byte ceilings.
- Closed `ReviewedSourceId` constants for planned Swedish agency sources,
  separated from dynamic descriptive IDs and future registry authority.
- Non-zero `SchemaVersion`, `PolicyVersion`, and `UpstreamVersion` wrappers.
- Payload-free `ValidationError` categories and `Result`-based validation for
  identifiers, source descriptors, versions, and response budgets.
- Compile-fail documentation proving downstream code cannot construct reviewed
  source identity or execution authority through `sweden-core`.
- Locked, checksum-verified RFC Editor references for the normative-language,
  Internet date/time, URI, HTTP status, JSON, HTTP semantics, caching, and
  HTTP/1.1 standards used by Sweden's planned source-neutral contracts.

### Fixed

- RFC verification now accepts the owner-writable files produced by a fresh
  Git checkout while retaining checksum-based tamper rejection, and shell
  syntax checks use each script's declared `sh` or Bash interpreter.

## 0.1.0 - 2026-07-28

### Added

- Dependency-free Rust workspace with two independently publishable crates:
  `sweden-core` and `sweden`.
- `no_std` facade and shared-core boundary.
- Phased crate-introduction policy that avoids publishing empty placeholders.
- Eth-style selective crate publisher with independent subcrate versions,
  facade/tag alignment, dependency ordering, and mandatory `1.0.0`
  convergence.
- Security, contribution, dependency, CI, modularity, toolchain, and release
  policies.
- Detailed implementation and release plans through the serious `1.0.0`
  production gate.

### Security

- Unsafe Rust is forbidden.
- Project crates have no third-party dependencies.
- No agency or transport crate is present, so no source or network capability is
  claimed.
- Response sizes require explicit budgets.
- Source descriptors reject a stable status paired with unreviewed access and
  expose metadata only through invariant-preserving construction and accessors.
