# Sweden Version Plan

The canonical per-version plan is [RELEASE_PLAN.md](RELEASE_PLAN.md). It gives
every version an explicit goal, deliverables, verification, exit criteria, and
maintainer pentest stop followed by the baseline, outcome, and GitHub loop.

High-level phases:

| Versions | Phase |
| --- | --- |
| `0.1.0` | Two-crate (`sweden-core` and `sweden`) repository, policy, CI, and documentation foundation |
| `0.2.0..=0.12.0` | Shared types, executable policy, provenance, transport, and body limits |
| `0.13.0..=0.18.0` | Dependency-free bounded JSON/XML codecs |
| `0.19.0..=0.21.0` | Testkit, offline source compiler, and synthetic conformance |
| `0.22.0..=0.36.0` | Trafikverket dossier, query model, generated slices, and checkpoints |
| `0.37.0..=0.44.0` | Rate/cache policy, ergonomics, platforms, fuzzing, docs, packaging |
| `0.45.0..=0.49.0` | Optional hosted service boundaries and tenant-safe operations |
| `0.50.0` | Full security architecture review |
| `0.60.0` | Legal and privacy readiness |
| `0.70.0` | Reliability and recovery |
| `0.80.0` | Limited public beta |
| `0.90.0..=0.99.0` | API freeze, remediation, and final acceptance |
| `1.0.0-rc.1` | Exact production candidate |
| `1.0.0` | Serious production release for documented scope |

Crates enter the workspace only when implementation starts; no empty
placeholder package is published. The first additions are `sweden-http` in
`0.10.0`, `sweden-testkit` in `0.19.0`, `sweden-schema` in `0.20.0`, and
`sweden-trafikverket` in `0.22.0`. Other named agency crates begin after
`1.0.0` on their independent stabilization tracks.

The repository version is the `sweden` facade version and the `vX.Y.Z` tag.
Subcrates advance independently only when required, as tracked in
[CRATE_VERSION_MATRIX.md](CRATE_VERSION_MATRIX.md). At `v1.0.0`, every crate
then present converges to `1.0.0`.

Patch releases may be inserted at any point and inherit the same gates. A
version number is a review boundary, never a deadline or permission to merge
unfinished work.
