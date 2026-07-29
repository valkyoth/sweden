# Sweden Implementation Plan

Status: repository foundation implemented; product capabilities planned

Repository and facade crate: `sweden`

Current workspace version: `0.1.0`

Production target: `1.0.0`

This plan turns [the initial architecture discussion](initial-idea.md) into an
implementable, dependency-free Rust project. The bottom amendment in that
document is authoritative: every Rust crate is independently publishable, each
agency owns a crate, and `sweden` becomes a feature-gated facade as those
crates are introduced.

## 1. Product Position

Sweden is a security-first Rust ecosystem for lawful, typed access to Swedish
public APIs and public datasets. It must preserve source-specific semantics,
terms, limits, provenance, and access controls.

The 1.0 product is:

- a stable `sweden-core` contract;
- a transport-neutral `sweden-http` boundary;
- a production-ready `sweden-trafikverket` integration;
- a small feature-gated `sweden` facade;
- a repeatable agency-onboarding system demonstrated by a synthetic
  conformance source without pulling post-1.0 agencies forward;
- documentation and policy evidence suitable for serious production review.

The `0.1.0` workspace deliberately contains only `sweden-core` and `sweden`.
Future crates are created and published when their first implementation
milestone begins; the repository does not carry empty placeholder packages.

The 1.0 product is not:

- a generic government URL proxy;
- a custom TLS, OAuth, BankID, X.509, or cryptography implementation;
- a promise that every visible dataset may be cached, transformed, or relayed;
- a replacement for legal review;
- a hosted credential pool for callers who are not independently authorized.

### 1.1 Gap-analysis integration decisions

The post-`0.1.0` architecture review strengthens this plan without replacing
its established scope or gates:

- accepted: `sweden-http` is a dependency-free `no_std` contract crate;
  sockets, TLS, clocks, executors, environment lookup, and platform networking
  remain outside it;
- accepted: caller-provided transports are trusted components, not
  cryptographic sandboxes; Sweden guarantees policy and origin validation only
  inside Sweden-controlled planning and reviewed executors;
- accepted: source policy and evidence are operation-specific, expiring, and
  produce private consumable permits rather than relying on an agency-wide
  access label;
- accepted: configured limits become checked ledgers charged before I/O,
  allocation, parsing, retry, redirect, page fetch, or checkpoint advance;
- accepted: JSON, XML, and CSV are narrow first-party subsets with explicit
  rejection behavior, caller-owned scratch in borrowed mode, and final fuzz
  campaigns before 1.0;
- accepted: default, `alloc`, `std`, transport, and agency capability tiers are
  explicit and never silently enable networking, credentials, proxy discovery,
  telemetry, live tests, or hosted relaying;
- retained: the zero-third-party and safe-Rust rules. A concrete ecosystem
  HTTP/TLS adapter or mobile FFI binding requires a future explicit dependency
  or unsafe-code admission decision and is not promised for 1.0;
- retained: Trafikverket is the only production agency scope before 1.0.
  SMHI, SCB, JobTech, and Skatteverket stay on the documented post-1.0 tracks;
- retained: the existing version-by-version pentest, GitHub, tagging, and
  independent publication process.

`RELEASE_PLAN.md` remains the authoritative version sequence. External reviews
may propose replacement roadmaps, but accepted work is mapped into that
sequence explicitly rather than silently renumbering or weakening established
milestones.

## 2. Non-Negotiable Engineering Rules

### 2.1 Rust and MSRV

- Pin the latest stable Rust release for development and release gates.
- Support Rust `1.90.0` through the pinned release, currently `1.97.1`.
- Use edition 2024 and workspace resolver 3.
- Require normal development to work without nightly.
- Test all supported stable toolchains before every tag.
- Check the official stable distribution manifest before every tag.

### 2.2 No third-party project crates

Runtime, development, and build dependency tables may contain only Sweden
workspace crates. Cargo tools used to verify the repository are not linked into
project artifacts and are pinned separately in CI.

If a future requirement genuinely cannot be implemented safely without an
external crate, implementation stops. The dependency must receive an explicit
user decision and its own admission milestone before any manifest change.

### 2.3 `no_std` first

`sweden-core`, the root facade, source-policy types, codecs, and agency crates
must use `#![no_std]`. Allocation becomes an explicit feature only when a
bounded owned representation is required.

`std` is allowed in focused crates for:

- user-supplied transport adapters;
- hosted services;
- filesystem-backed fixtures and schema tooling;
- command-line release and source-review utilities.

Agency crates never acquire `std` merely for convenience.

### 2.4 Unsafe Rust

Unsafe Rust is forbidden. If future platform FFI requires it, the work stops
for an explicit architecture and security decision. It may not be introduced
inside parser, policy, authentication, agency, or facade code.

### 2.5 Modularity

- One agency or upstream API per crate.
- Agency crates depend on `sweden-core`, not on one another or the facade.
- The facade contains wiring and re-exports, not implementations.
- Network adapters do not own source semantics.
- Policy and codecs remain source-independent.
- Non-generated Rust files must never exceed 500 lines.
- Review splitting once a Rust file approaches 300 lines.
- Generated files require an explicit manifest and are split by upstream
  object family.

### 2.6 Bounded behavior

Every operation declares limits for wire bytes, decoded bytes, nesting,
strings, collection elements, pages, records, redirects, time, and retries.
There is no unbudgeted `collect_all`, response buffering, decompression, archive
extraction, or recursive parse.

Configured ceilings and consumed state are separate types. Charges use checked
arithmetic and occur before accepting bytes, allocating, transmitting an
attempt, following a redirect, fetching a page, or committing a checkpoint.
Callers may tighten reviewed ceilings but cannot raise them through a stable
public API.

### 2.7 Honest capability claims

Crates report `Foundation`, `Experimental`, or `Stable`. A source cannot become
stable until its exact operation set, source terms, schema revision, fixtures,
tests, security review, and pentest evidence are current.

`IntegrationStatus` is descriptive only. Executable stable behavior requires a
private, generated capability bound to the operation policy, dossier digest,
schema version, review expiry, environment, and current evidence.

## 3. Workspace Architecture

Planned 1.0 dependency direction:

```text
sweden-core
    ↑
    ├── sweden-http
    └── sweden-trafikverket
             ↑
           sweden
```

The facade depends on selected crates only through feature flags. Agency crates
do not depend on `sweden-http`; they emit transport-neutral operation plans.
SMHI, SCB, JobTech, and Skatteverket are post-1.0 additions and therefore do
not appear in the 1.0 graph.

At `0.1.0`, the implemented graph is only:

```text
sweden-core
    ↑
  sweden
```

Planned shared crates are introduced only when the existing boundary is too
large for one audit:

| Crate | Environment | Planned responsibility |
| --- | --- | --- |
| `sweden-core` | `no_std` | IDs, errors, limits, policy and provenance contracts |
| `sweden-codec-json` | `no_std`/`alloc` | Bounded first-party JSON tokens and decoding |
| `sweden-codec-xml` | `no_std`/`alloc` | Bounded first-party XML without DTD/entities |
| `sweden-codec-csv` | `no_std`/`alloc` | Bounded first-party record parsing |
| `sweden-policy` | `no_std` | Executable source and operation policy |
| `sweden-http` | `no_std` | Sans-I/O blocking/async transport contracts and bounded sinks |
| `sweden-testkit` | `std` | Deterministic mock, replay, mutation, and fixture support |
| `sweden-schema` | `std` | Offline deterministic schema processing |
| `sweden-*` agency crates | `no_std`/`alloc` | Source-owned operation and payload semantics |
| `sweden` | `no_std` | Feature-gated public re-exports |

Every crate is a crates.io package when introduced. Tool and service crates are
public when added.

Initial crate-introduction schedule:

| Version | Crate introduced and published |
| --- | --- |
| `0.1.0` | `sweden-core`, `sweden` |
| `0.10.0` | `sweden-http` |
| `0.19.0` | `sweden-testkit` |
| `0.20.0` | `sweden-schema` |
| `0.22.0` | `sweden-trafikverket` |
| `0.51.0` | `sweden-codec-csv` only if the dedicated boundary is justified |
| Post-`1.0.0` | Remaining named agency crates on their own tracks |

The facade crate is the repository release clock: `sweden` always equals the
`vX.Y.Z` tag and publishes for that release. Subcrates have independent
versions and publish only for their own code, bugfix, dependency, or immutable
metadata changes. At `v1.0.0`, every crate then in the workspace converges to
`1.0.0`. The current state is recorded in
[CRATE_VERSION_MATRIX.md](CRATE_VERSION_MATRIX.md) and mechanically checked by
`scripts/release_crates.py`.

## 4. Request Lifecycle

```text
validated caller input
        ↓
typed agency operation
        ↓
closed source + operation identifier
        ↓
operation policy + dossier evidence preflight
        ↓
credential-free canonical request plan
        ↓
private consumable policy/rate permit
        ↓
late credential injection into a reviewed execution sink
        ↓
trusted caller-owned transport boundary
        ↓
bounded body sink
        ↓
source decoder and semantic validation
        ↓
provenance-wrapped result
```

The API must make it impossible to select an arbitrary production origin.
Credentials are inserted only after the origin is validated and are excluded
from debug output, cache keys, canonical hashes, errors, and fixtures.
Permits are non-`Copy`, non-`Clone`, operation-, environment-, and
origin-bound, and consumed by execution. Retries, redirects, and subsequent
pages require fresh charges or permits.

A caller-owned transport can still copy credentials, ignore deadlines, choose
another destination, or log data. Sweden does not claim to sandbox arbitrary
trait implementations. DNS, TLS, proxy, and network-egress enforcement are
adapter and deployment responsibilities; stronger isolation requires a
separate process boundary.

## 5. Source Onboarding

No agency implementation begins with endpoint code. It begins with a dated
source dossier:

1. Confirm official owner, documentation, origins, versions, support channel,
   terms, licence, attribution, and change policy.
2. Pin retrieval time, content digest, reviewer, review expiry, and explicit
   exclusions for every evidence source.
3. Classify each operation—not merely the agency—by environment, origin,
   method, path, redirects, authentication, hosted use, personal-data risk,
   caching, transformation, redistribution, attribution, rate, concurrency,
   retry, and pagination rules.
4. Pin official schema/specification inputs with retrieval metadata and hashes.
5. Define explicit request, response, collection, retry, redirect, allocation,
   and time budgets.
6. Build synthetic fixtures before admitting redistributable official
   fixtures.
7. Implement generated policy contradiction tests, request goldens, and
   negative parser tests before live execution.
8. Add opt-in low-rate live tests only when the source permits them.
9. Generate documentation from the same reviewed operation metadata.
10. Stop for maintainer pentest and keep the versioned repository report
    current.

Policy expiry fails closed. An expired source review cannot silently continue
hosted relaying.

## 6. Codec Strategy

The no-third-party rule requires focused first-party codecs. They are not
general-purpose replacements for ecosystem parsers.

JSON work is split into:

- byte validation and UTF-8 boundary;
- bounded tokenization with exact JSON number grammar;
- raw/decoded string ceilings, escape validation, Unicode scalar and surrogate
  handling;
- iterative structure with token, depth, member, element, and duplicate-key
  policy;
- exact consumption after trailing whitespace;
- borrowed events with caller scratch rather than allocation merely to
  unescape;
- bounded owned values behind `alloc`, charged before reserve;
- source-specific typed decoding;
- mutation and differential fixtures generated out of process where useful.

XML work is split into:

- strict UTF-8/XML character validation;
- bounded iterative tokenization with caller-provided stack;
- namespace handling, exact start/end matching, and duplicate expanded
  attribute rejection;
- unconditional early rejection of DTD, entity declarations, and external
  identifiers; only predefined entities and bounded numeric references are
  admitted;
- canonical escaping and deterministic output;
- source-specific streaming decode.

CSV is a separate milestone. Each operation fixes its delimiter, quoting,
line-ending, header, blank-record, BOM, and encoding rules; no dialect is
guessed. Spreadsheet-safe export neutralizes formula-leading `=`, `+`, `-`,
and `@`. Archive support remains unadmitted until it receives its own bounded
security milestone. Unsupported constructs fail closed.

## 7. Trafikverket 1.0 Track

Trafikverket is the flagship because it exercises registration, credential
injection, typed query XML, substantial model coverage, pagination/change
semantics, freshness, and strict source budgets.

The implementation sequence is:

1. Current official source dossier and terms evidence.
2. Closed production/test origins and environment separation.
3. Redacted API-key provider boundary.
4. One reviewed raw operation through mock transport.
5. Typed field metadata and object definitions.
6. Filter AST with field/operator compatibility.
7. Projection, ordering, paging, and cost budgets.
8. Canonical bounded XML encoder.
9. Strict response envelope and upstream error decoder.
10. Offline deterministic model generation.
11. Small object-family slices with fixture and live evidence.
12. Change/checkpoint behavior where officially supported.
13. Compatibility, performance, memory, documentation, security, and legal
    stabilization.

No object-family milestone may exceed one reviewable model slice. Unsupported
objects remain explicit rather than hidden behind broad completion claims.

## 8. Transport And TLS Boundary

This repository does not implement TLS. `sweden-http` defines request and
response contracts for a transport supplied by the application or deployment.
The crate itself remains `no_std`: traits, `core::future::Future`, structured
plans, cancellation state, and caller-owned sinks do not require sockets or
`std`.

The transport boundary requires:

- a closed, source-selected origin;
- HTTPS for production origins;
- same-origin or explicitly reviewed redirects;
- late credential injection;
- response byte accounting before decode;
- explicit deadlines and cancellation;
- safe error categories without raw headers or URLs;
- no automatic proxy-environment use unless the caller explicitly enables it.

Because project crates cannot depend on third-party HTTP/TLS clients, concrete
ecosystem adapters are not admitted under the current policy. Users bridge
their maintained transport through the public trait. A future adapter requires
an explicit dependency-policy change.

Reviewed adapters must return redirects as data, disable automatic proxy
discovery and redirects by default, avoid unmetered decompression and
buffering, and translate adapter errors immediately into closed safe
categories. These are conformance properties, not guarantees about arbitrary
implementations.

## 9. Platform Plan

Portable crates avoid OS assumptions and are checked for:

- Linux;
- Windows;
- macOS;
- FreeBSD and NetBSD as distributed BSD compile targets;
- Android;
- iOS.

Other BSD targets are added as toolchains and runners permit. Aesynx requires
no implementation today; the transport-neutral and `no_std` boundaries prevent
an OS-specific lock-in and reserve a future `sweden-aesynx` adapter crate.

## 10. Testing Plan

Every public behavior needs:

- positive unit tests;
- boundary-value tests;
- invalid and truncated input tests;
- allocation/size/depth budget tests;
- redaction tests where credentials or personal data can appear;
- deterministic request golden tests;
- fixture decode tests;
- operation policy allow/deny tests;
- compile tests for feature combinations and `no_std`;
- supported-toolchain checks;
- platform checks;
- isolated package checks.

Capability checks cover:

- default: no allocation, I/O, clock, randomness, credentials, or globals;
- `alloc`: bounded ownership without implying `std`;
- `std`: orchestration interfaces without implying networking;
- transport/agency features: no silent allocation, credentials, proxy,
  telemetry, filesystem, live-test, or hosted-relay activation.

Parsers additionally need:

- deterministic mutation tests;
- corpus replay;
- stack-depth evidence;
- exact-consumption tests;
- no-panic arbitrary byte tests;
- differential evidence against independently generated fixtures when that
  does not add a project dependency.

Live tests are opt-in, rate-limited, secret-safe, and never a substitute for
deterministic fixtures.

## 11. Security Work From Day One

Security deliverables grow with implementation:

- repository threat model and security policy;
- dependency/source deny policy;
- no-unsafe and no-third-party validators;
- payload-free logging rules;
- secret type and redaction snapshots before credentials;
- parser budgets before parser exposure;
- SSRF and redirect tests before transport execution;
- cache partition tests before caching;
- tenant tests before hosted multi-tenancy;
- policy expiry and kill-switch tests before hosted relaying;
- release notes and pentest handoff for every version.

Security documentation is release evidence, not aspirational prose.

## 12. Documentation And Releases

Every crate owns a README with the common Sweden header and a precise statement
of current capability. Repository `README.md` and `crates/sweden/README.md`
must remain byte-identical.

Every release requires:

- changelog entry;
- version-specific release notes;
- updated capability and compatibility tables;
- updated source/spec evidence when applicable;
- local checks;
- supported-Rust checks;
- package checks;
- dependency and advisory checks;
- maintainer pentest, remediation, and a current versioned report;
- an exact implementation baseline committed before the maintainer pentest;
- pentest outcome, remediation, and the current report committed before GitHub;
- green GitHub Actions and CodeQL default setup on the latest `PASS` commit;
- explicit maintainer authorization before tagging.

The granular version sequence and exact stop language live in
[the release plan](RELEASE_PLAN.md).

## 13. Definition Of Serious 1.0

`1.0.0` is admitted only when:

- shared contracts and the facade are semver-reviewed;
- every public crate packages independently;
- Rust `1.90.0` through `1.97.1` pass;
- portable crates pass platform compilation;
- `no_std` and all feature combinations pass;
- Trafikverket’s declared stable object/operation matrix is complete;
- source terms, schema evidence, and live conformance are current;
- there are no unbudgeted parser, body, page, retry, or time paths;
- JSON, XML, and CSV admitted subsets have final corpus and fuzz evidence;
- every budget is a consumable pre-charged ledger rather than passive metadata;
- there is no arbitrary-origin or credential-leaking path;
- caller-owned transport trust and Sweden-controlled executor guarantees are
  documented without cryptographic-sandbox claims;
- operation-level policy, dossier, provenance, and expiry evidence gates every
  stable capability;
- borrowed, `alloc`, `std`, and transport feature boundaries are verified;
- public docs contain no unsupported production claims;
- independent security review and maintainer pentest findings are resolved in
  the versioned repository report;
- release evidence, GitHub CI, and CodeQL default setup are green.

Only then may the project describe `1.0.0` as serious production-ready
infrastructure for its documented scope.
