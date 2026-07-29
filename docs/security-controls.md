# Sweden Security Controls

## Active Foundation Controls

- `#![forbid(unsafe_code)]` in every first-party crate.
- Dependency-free `no_std` core and facade crates.
- No default network, TLS, credential, filesystem, clock, or telemetry path.
- Explicit response budgets.
- No agency or transport implementation before its planned review milestone.
- Foundation status prevents production-complete claims.
- Cargo source, license, duplicate, and advisory deny policy.
- Latest-stable and tool freshness release checks.
- SHA-pinned GitHub Actions.
- GitHub CodeQL default setup policy.
- Generated and handwritten Rust source limit of 500 lines.
- Byte-identical repository and facade READMEs.
- Exact-version release notes and pentest stop.

## Required Before Untrusted Parsing

- token, depth, string, collection, and decoded-byte budgets;
- exact-consumption semantics;
- malformed, truncated, oversized, and mutation tests;
- DTD/entity rejection for XML;
- decoded-name duplicate detection for JSON and explicit XML 1.0 lexical,
  QName, namespace, comment, CDATA, processing-instruction, declaration, and
  reference-digit budgets;
- archive traversal and decompression-ratio controls where applicable;
- no-panic arbitrary input evidence.

## Required Before Network Execution

- closed origin registry;
- HTTPS-only production policy;
- reviewed redirect rules;
- late credential injection;
- safe proxy policy;
- indivisible registry authorization binding the exact request encoder,
  response profile, decoder, validator, output/provenance type, limits, and
  finalization;
- explicit deadline mode, retries, and rate budgets without claiming a clock
  can preempt a stalled transport;
- complete reviewed interval/window/concurrency enforcement and fenced lease
  recovery; official network execution remains prohibited through `v0.36.0`;
- response byte accounting before decode;
- SSRF and credential-destination tests.

## Required Before Hosted Multi-Tenancy

- explicit tenant context;
- tenant-scoped credentials, cache keys, quotas, and audit events;
- operation allowlist;
- policy expiry and source kill switch;
- payload-free telemetry;
- retention and deletion controls;
- cross-tenant negative tests;
- backup, restore, and incident exercises.
