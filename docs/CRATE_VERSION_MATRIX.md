# Crate Version Matrix

Status: `v0.1.0` release candidate; pentest passed; awaiting GitHub

Sweden uses independent crate versions. The `sweden` facade is the release
clock and always has exactly the version represented by the repository tag.
Subcrates are published only when their own package or a published dependency
requirement changes.

## Version Rules

| Change kind | Subcrate version rule | Publish? |
| --- | --- | --- |
| `code` | Increment the crate's independent minor and reset patch to zero. | Yes |
| `bugfix` | Increment the crate's current patch exactly once. | Yes |
| `dependency` | Stay on the current minor line and increase the patch. | Yes |
| `metadata` | Use the repository milestone version for an immutable package-metadata correction. | Yes |
| `unchanged` | Keep the previous published version. | No |

The facade is different: `sweden` must equal every `vX.Y.Z` tag and must be
published for that release. At `v1.0.0`, every crate then present in the
workspace must converge to `1.0.0` and be published, regardless of its
independent pre-1.0 line.

`dependency` means implementation and public API did not meaningfully change,
but a manifest must move because a workspace dependency left the published
compatible range. `bugfix` preserves the public API. `metadata` is not a way
to hide implementation changes.

`scripts/release_crates.py --check` validates this file's machine-readable
counterpart, `release-crates.toml`, against Cargo metadata and refuses
accidental lockstep publication.

## v0.1.0 Tracking Table

| Crate | Published | Planned | Change | Publish | Reason |
| --- | --- | --- | --- | --- | --- |
| `sweden-core` | Not yet published | `0.1.0` | `code` | Yes | Initial dependency-free shared contracts. |
| `sweden` | Not yet published | `0.1.0` | `code` | Yes | Initial dependency-free facade over `sweden-core`. |

Update this table, `release-crates.toml`, affected Cargo manifests, dependency
pins, release notes, and the pentest report together whenever a crate changes
release state. Add a crate to the publisher's dependency order only when that
crate's implementation begins.
