# Sweden Version Plan

The canonical per-version plan is [RELEASE_PLAN.md](RELEASE_PLAN.md). It gives
every version an explicit goal, deliverables, verification, exit criteria, and
maintainer pentest stop followed by the baseline, outcome, and GitHub loop.

High-level phases:

| Versions | Phase |
| --- | --- |
| `0.1.0` | Two-crate (`sweden-core` and `sweden`) repository, policy, CI, and documentation foundation |
| `0.2.0..=0.9.0` | Identifiers, bounded types, request planning, executable operation policy, evidence, and provenance |
| `0.10.0..=0.12.0` | Dependency-free `no_std` transport contracts and consumable body ledgers |
| `0.13.0..=0.18.0` | Dependency-free bounded JSON/XML codecs |
| `0.19.0..=0.21.0` | Testkit, offline source compiler, generic executor, and synthetic conformance |
| `0.22.0..=0.36.0` | Trafikverket dossier, query model, generated slices, and checkpoints |
| `0.37.0..=0.44.0` | Rate/cache policy, ergonomics, platforms, fuzzing, docs, packaging |
| `0.45.0..=0.49.0` | Executor, credential, replay, quota/cache, and SDK diagnostic stabilization |
| `0.50.0` | Full security architecture review |
| `0.51.0..=0.59.0` | CSV, ledger/trust/capability audits, feature isolation, constrained-device qualification, ergonomics, and completeness |
| `0.60.0` | Legal and privacy readiness |
| `0.70.0` | Reliability and recovery |
| `0.80.0` | Limited public beta |
| `0.90.0` | First frozen release candidate |
| `0.91.0..=0.94.0` | Cross-topology simulation, final codec fuzzing, and platform/resource qualification |
| `0.95.0` | Independent-assessment remediation |
| `0.96.0..=0.98.0` | Supply-chain/source refresh and exact public-surface freeze |
| `0.99.0` | Final acceptance |
| `1.0.0-rc.1` | Exact production candidate |
| `1.0.0` | Serious production release for documented scope |

## Gap-Analysis Integration Map

The post-`0.1.0` gap analysis strengthened the existing sequence; it did not
replace the established product scope, crate policy, or release gate.

| Accepted concern | Versions that establish it | Later proof point |
| --- | --- | --- |
| Closed identifiers and operation-specific policy | `0.2.0`, `0.7.0..=0.9.0` | `0.20.0`, `0.54.0`, `0.59.0` |
| Cross-crate authority without a false sealing claim | design at `0.2.0`, closed policy registry at `0.7.0`, opaque permits at `0.9.0` | hostile downstream proof at `0.9.0`/`0.21.0`, ownership audit at `0.45.0` |
| Payload-free validation and one shared operation path | `0.2.0`, `0.4.0`, `0.5.0` | `0.39.0`, `0.56.0`, `0.57.0` |
| Consumable pre-charged resource ledgers | `0.3.0`, `0.12.0`, parser and paging milestones | `0.52.0`, `0.56.0`, `0.94.0` |
| Local ledgers versus coordinated quota authority | ledgers at `0.3.0`, minimal authority contract at `0.9.0`, executor use at `0.21.0` | algorithms at `0.37.0`/`0.48.0`, proof at `0.52.0`/`0.91.0` |
| Stable external clock/quota/policy contract shapes | `0.9.0`, consumed by `0.21.0` | algorithms at `0.37.0`, trust audit at `0.53.0` |
| `no_std` sans-I/O transport contract | `0.10.0..=0.12.0` | `0.53.0`, `0.55.0`, `0.58.0` |
| Strict first-party JSON/XML and conditionally admitted CSV subsets | `0.13.0..=0.18.0`, `0.51.0` | `0.92.0`, `0.93.0` |
| Evidence-bound expiring capabilities and permits | operational at `0.9.0`, used by `0.21.0..=0.24.0` | stabilization at `0.54.0`, then `0.59.0`, `0.97.0` |
| Explicit generic executor ownership | `0.21.0` | `0.45.0`, `0.57.0`, `0.98.0` |
| No live request before minimum quota/concurrency admission | `0.21.0`, `0.24.0` | full enforcement at `0.37.0`, topology proof at `0.91.0` |
| Honest trusted-transport boundary | `0.10.0`, `0.21.0`, `0.23.0` | `0.53.0`, `0.58.0` |
| Provisional versus finalized streaming | `0.12.0`, `0.21.0` | checkpoint/cache enforcement at `0.36.0`, `0.38.0` |
| Initial adversarial fuzzing before official responses | `0.21.0` | broad baseline at `0.42.0`, final campaigns at `0.92.0`, `0.93.0` |
| Trust limits for every caller authority | contract-specific milestones | combined matrix and malicious doubles at `0.53.0` |
| Explicit default/`alloc`/`std`/transport tiers | all crate introductions and `0.40.0` | `0.55.0`, `0.98.0` |
| Mobile, automotive, and future Aesynx portability | `0.40.0`, `0.41.0` | `0.56.0`, `0.94.0` |
| Metadata-driven modularity and claim checking | `0.20.0`, `0.43.0`, `0.44.0` | `0.59.0`, `0.98.0` |
| Hostile schema/generator and overlay boundary | `0.20.0`, `0.30.0` | `0.42.0`, `0.59.0`, `0.96.0` |
| One 500-line ceiling for generated and handwritten Rust | every crate introduction | `0.20.0`, `0.44.0`, `0.98.0` |
| Hosted gateway/service product remains post-1.0 | removed from `0.45.0..=0.49.0` | separate service-crate `0.1.0..=0.5.0` track after explicit admission |
| Independent final assurance | per-version maintainer pentest from day one | `0.91.0..=0.99.0`, `1.0.0-rc.1` |

Two recommendations were deliberately not accepted as roadmap replacements:
Trafikverket remains the only production agency before 1.0, and concrete
third-party HTTP/TLS adapters or unsafe FFI bindings remain outside 1.0 unless
the project explicitly changes its zero-third-party or safe-Rust policy.

Crates enter the workspace only when implementation starts; no empty
placeholder package is published. Focused crates enter only when their
implementation begins: `sweden-policy` in `0.7.0`, `sweden-http` in `0.10.0`,
`sweden-codec-json` in `0.13.0`, `sweden-codec-xml` in `0.16.0`,
`sweden-testkit` in `0.19.0`, `sweden-schema` in `0.20.0`,
`sweden-executor` in `0.21.0`, and `sweden-trafikverket` in `0.22.0`.
`sweden-codec-csv` enters at `0.51.0` only if a named reviewed 1.0 operation
requires CSV; otherwise CSV and its crate are deferred post-1.0. Other named
agency crates begin after `1.0.0` on their independent stabilization tracks.

The repository version is the `sweden` facade version and the `vX.Y.Z` tag.
Subcrates advance independently only when required, as tracked in
[CRATE_VERSION_MATRIX.md](CRATE_VERSION_MATRIX.md). At `v1.0.0`, every crate
then present converges to `1.0.0`.

Patch releases may be inserted at any point and inherit the same gates. A
version number is a review boundary, never a deadline or permission to merge
unfinished work.

`RELEASE_PLAN.md` is authoritative when a summary and a detailed milestone
ever diverge. Each milestone retains the same simple stop: implementation and
report baseline, maintainer pentest, report/fix commit, GitHub green, then an
explicit maintainer request to tag.
