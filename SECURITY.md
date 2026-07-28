# Security Policy

Sweden is security-sensitive API and public-data infrastructure. Treat source
policy, request construction, credentials, redirects, parsing, decompression,
pagination, caching, provenance, rate limits, CI, releases, and dependency
changes as high risk until reviewed and tested.

## Current Security Scope

Version `0.1.0` is a repository foundation. It contains no concrete network or
TLS client and no production-ready agency integration. A report that an
unimplemented feature is absent is not a vulnerability; an unsafe API,
credential leak, policy bypass, parser flaw, or supply-chain weakness is.

## Routine Checks

Run these regularly and before releases:

```bash
scripts/checks.sh
scripts/check_latest_tools.sh
scripts/release_0_1_gate.sh
cargo deny check
cargo audit
```

GitHub Actions run CI. GitHub CodeQL default setup must be enabled in repository
security settings. Do not add an advanced CodeQL workflow while default setup
is active. See [GitHub security settings](docs/github-security-settings.md).

## Release Gate

Every release uses one simple loop:

1. Implementation and local verification finish.
2. Codex stops and asks the maintainer to pentest the version.
3. The maintainer reports findings, or reports that no findings were found.
4. Codex keeps `security/pentest/vX.Y.Z.md` current, fixes findings, and reruns
   the local gates until the report can say `Status: PASS`.
5. Implementation, fixes, release metadata, and the pentest report are committed
   together.
6. The project waits for GitHub Actions and CodeQL default setup.
7. If GitHub finds an issue, Codex fixes it, updates the same pentest report,
   commits again, and the project waits again.
8. If GitHub is green, the maintainer may explicitly ask Codex to tag.

There is no separate report-only commit, reviewed-parent rule, or post-green
metadata commit. No tag is created or pushed without the maintainer's explicit
request. See [the release plan](docs/RELEASE_PLAN.md).

## Dependency Policy

Project crates currently use no third-party dependencies. `deny.toml` denies
unknown registries, unknown Git sources, wildcard versions, duplicate versions,
and known advisories. A future exception requires a dedicated admission
release, current-version verification, license and maintenance review,
feature/`std` review, source inspection, tests, security documentation, and a
pentest of the affected boundary.

Tooling dependencies are pinned in CI and checked for freshness before release.

## Reporting

Do not publish exploitable security details in an issue. Use GitHub private
security advisories for `valkyoth/sweden`, or contact the repository maintainer
privately if that channel is unavailable.
