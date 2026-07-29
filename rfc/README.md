# Sweden RFC Reference Copies

This directory contains exact unmodified plain-text copies downloaded from the
[RFC Editor](https://www.rfc-editor.org/). They are local normative references
for Sweden implementation review, requirements, tests, and security analysis.

The project and its maintainers claim no copyright in these RFC documents.
Each file retains its original notices and remains governed by the
[IETF Trust Legal Provisions](https://trustee.ietf.org/license-info), not the
repository's MIT/Apache-2.0 software licence.

## Tracked Baseline

| RFC | Role |
| --- | --- |
| RFC 2119 and RFC 8174 | normative requirement language |
| RFC 3339 | Internet date/time profile used when an operation selects it |
| RFC 3986 | URI syntax and percent encoding |
| RFC 6585 | additional HTTP status codes, including 429 |
| RFC 8259 | JSON |
| RFC 9110 | HTTP semantics, methods, fields, status, validators, redirects |
| RFC 9111 | HTTP caching |
| RFC 9112 | HTTP/1.1 message/framing trust boundary |

This is Sweden's source-neutral pre-1.0 set, not a ceiling. A named Swedish
agency operation or format adds another standard only after review proves it
is applicable.

## Integrity Lock

- `SOURCES` is the reviewed URL/role allowlist.
- `SHA256SUMS` pins every exact file.
- `scripts/fetch-rfcs.sh` downloads only missing allowlisted HTTPS sources and
  verifies their expected digest before installation.
- `scripts/verify-rfcs.sh` rejects missing, extra, empty, changed, or writable
  files.
- `scripts/lock-rfcs.sh` reapplies the local read-only guard.
- `scripts/test-rfc-sources.py` tests the complete baseline.
- `.gitattributes` disables line-ending normalization and whitespace cleanup
  for the exact RFC text.

Git does not portably preserve read-only permissions, so checksums remain the
authoritative integrity control.

## Update Procedure

1. Establish why the standard is required by a named capability.
2. Add its exact RFC Editor URL and role to `SOURCES`.
3. Download to a temporary location, inspect identity and provenance, and
   compute SHA-256 independently.
4. Add the unchanged file and digest together.
5. Run the RFC checks and full repository gate.
6. Update affected requirements, plans, security notes, and release notes.

Published RFCs are immutable. Corrections are separate errata/review records;
the local RFC bytes are never edited.

The workspace root is virtual and publishable crates live under `crates/`.
RFC text must never enter a crate package.
