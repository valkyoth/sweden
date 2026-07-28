# Sweden Threat Model

Status: initial foundation threat model

## Assets

- Caller and upstream credentials.
- Correct source, origin, operation, schema, terms, and rate policy.
- Integrity and provenance of official data.
- Personal, sensitive, confidential, and security-relevant response data.
- Tenant isolation in future hosted services.
- Release artifacts, source snapshots, fixtures, and dependency integrity.

## Adversaries

- A caller supplying malicious paths, filters, payloads, budgets, or cursors.
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

## Trust Boundaries

- Caller input to validated agency operation.
- Operation to executable source policy.
- Policy-approved operation to canonical request plan.
- Request plan to caller-supplied transport.
- Closed origin selection to late credential injection.
- Upstream bytes to bounded codec.
- Decoded syntax to source semantic validation.
- Raw source data to normalized or transformed data.
- Source result to cache and hosted response.
- Tenant context to credentials, limiter, cache, and audit storage.
- Official schema/terms input to checked-in reviewed snapshot.
- Cargo tools and GitHub Actions to release evidence.

## Baseline Mitigations

- No arbitrary production host.
- No concrete network or credential path at foundation.
- No unsafe Rust or third-party project crate.
- `no_std` core and facade boundaries; future agency boundaries inherit this
  requirement.
- Explicit budgets and fail-closed validation.
- Source and operation-specific policy.
- Payload-free logging by default.
- Human review for upstream schema or terms changes.
- Pinned toolchain and GitHub Action revisions.
- Maintainer pentest and a current versioned report committed with the release
  work before every tag.

## Residual Risks

- `no_std`, dependency-free code, and safe Rust do not prevent logic errors.
- A caller-supplied transport remains trusted to enforce TLS and origin
  requirements.
- Source policy metadata can be wrong or become stale.
- Official data can contain malicious, misleading, or personal content.
- Compilation on a platform does not prove runtime networking behavior there.
- Foundation tests do not establish production readiness.

Residual risks are narrowed release by release and must remain visible in
release notes.
