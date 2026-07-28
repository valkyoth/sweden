# Sweden Release Plan To 1.0

Status: planning document; `v0.1.0` repository foundation in implementation

This plan is deliberately granular. Each version is one bounded review and
pentest pass. Split a version or add a patch release whenever its scope stops
being easy to understand, test, and audit.

Tags use:

```text
v0.N.0      milestone release
v0.N.P      compatibility-preserving fix for milestone N
v1.0.0-rc.N exact 1.0-versioned production candidate
v1.0.0      first serious production-ready release for documented scope
```

Every Rust crate is publishable to crates.io once introduced. Crates enter the
workspace only when their implementation starts; empty placeholder crates are
not published. Publication order within a release is `sweden-core`, affected
shared/transport/agency crates or tools/services, then `sweden`.

## Required Gate For Every Version

Every version section has an explicit Goal, Deliverables, Verification, and
Exit criteria. Its verification is additive to:

```bash
scripts/checks.sh
cargo deny check
cargo audit
```

Release preparation also requires:

- latest stable Rust and pinned Cargo tool checks;
- Rust `1.90.0` through the pinned stable toolchain compatibility checks;
- platform checks appropriate to the changed crates;
- isolated packaging for every affected crate;
- `scripts/release_crates.py --check` against the crate version matrix;
- updated changelog, README capability claims, and release notes;
- current upstream source/spec/terms evidence where source behavior changes;
- green GitHub Actions and CodeQL default setup on the latest `PASS` commit;
- no unresolved critical or high finding;
- maintainer pentest and a current versioned repository report with
  `Status: PASS`.

## Simple Pentest And GitHub Flow

When an implementation stop is reached:

1. Codex commits the completed implementation with the version report in
   `AWAITING PENTEST` state.
2. Codex asks the maintainer to pentest that exact baseline commit.
3. The maintainer reports findings or reports that none were found.
4. Codex updates `security/pentest/vX.Y.Z.md`. Findings are fixed, documented
   in that same report, and retested until its
   status is `PASS`. A clean pentest is also documented as `PASS`.
5. The pentest outcome, fixes when needed, release metadata, and the updated
   report are committed together.
6. The project waits for GitHub Actions and CodeQL default setup.
7. If GitHub fails, Codex fixes the issue, updates the same report, commits
   again, and waits again.
8. When GitHub is green, Codex waits for the maintainer to explicitly request
   the tag.

A clean pentest may produce a report-only outcome commit. There is no
reviewed-parent rule, automatic tag, or post-green documentation commit.

## Independent Crate Versions

The `sweden` facade always uses the repository `vX.Y.Z` tag version and is
published for every release. Other crates retain independent versions and are
published only when their code, API-compatible bugfixes, dependency
requirements, or immutable package metadata require it. Unchanged subcrates
are not republished.

Before each release, update `release-crates.toml`,
`docs/CRATE_VERSION_MATRIX.md`, affected manifests and exact workspace
dependency pins. `scripts/release_crates.py --check` validates these against
Cargo metadata. Once the maintainer has requested the tag and that tag points
at the approved `HEAD`, `scripts/release_crates.py --require-tag` publishes the
selected crates in dependency order and can resume with `--start-at`.

For `v1.0.0`, the normal independent-version rule pauses once: every crate then
present in the workspace must be versioned and published as `1.0.0`. Later
releases return to independent subcrate versioning.

## v0.1.0 - Repository Foundation

Goal: establish a secure, publishable, dependency-free workspace.

Deliverables:

- Two focused initial crates, `sweden-core` and `sweden`, with complete
  crates.io metadata.
- Rust `1.97.1` pin and `1.90.0` MSRV.
- Dual MIT/Apache-2.0 licensing, CI, community files, policies, and plans.
- Dependency-free `no_std` facade and core boundaries.
- README compatibility table and byte-identical facade README.

Verification:

- Full inherited gate, both crate test suites, README identity, file-size policy,
  manifest policy, and package dry runs.
- Confirm only `sweden-core` and `sweden` are packaged and neither has an
  external dependency.

Exit criteria:

- Repository claims match implemented evidence and all required files exist.
- `v0.1.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.2.0 - Identifier And Version Primitives

Goal: make source, operation, schema, policy, and upstream versions explicit.

Deliverables:

- Validated borrowed identifiers and non-zero version wrappers.
- Stable comparison and display rules without allocation.
- Boundary and invalid-input tests.
- Core API and crate documentation.

Verification:

- Inherited gate plus exhaustive empty, length, character, and boundary tests.
- `no_std` compilation on MSRV and pinned stable.

Exit criteria:

- No unvalidated string is accepted as a stable identifier.
- `v0.2.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.3.0 - Bounded Scalar Types

Goal: prevent unbounded or ambiguous scalar input at API boundaries.

Deliverables:

- Bounded strings, page sizes, byte counts, retry counts, and nesting limits.
- Checked constructors and stable error paths.
- No panics or unchecked arithmetic on caller input.

Verification:

- Inherited gate plus minimum, maximum, overflow, and one-past-boundary tests.
- Deterministic arbitrary-value constructor sweep.

Exit criteria:

- Every introduced scalar can represent only reviewed valid states.
- `v0.3.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.4.0 - Error And Redaction Model

Goal: expose useful failures without leaking secrets or upstream payloads.

Deliverables:

- Non-exhaustive stable error categories and safe field paths.
- Retry advice separated from error text.
- Redacted debug/display policy and snapshot fixtures.
- Panic-free conversion tests.

Verification:

- Inherited gate plus adversarial formatting and secret-marker snapshots.
- Confirm errors allocate only behind explicit `alloc`.

Exit criteria:

- Public errors reveal no protected headers, payloads, or credential slots.
- `v0.4.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.5.0 - Request Plan Core

Goal: represent requests without performing network activity.

Deliverables:

- Method, reviewed relative path, protected header slots, and bounded body plan.
- Canonical credential-free representation.
- Explicit response and execution budgets.
- Compile-time separation from `std`.

Verification:

- Inherited gate plus request canonicalization goldens and invalid path tests.
- Prove no arbitrary scheme, authority, or absolute URL is representable.

Exit criteria:

- Agency operations can describe a bounded request without choosing transport.
- `v0.5.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.6.0 - Closed Origin Registry

Goal: make SSRF-resistant source origin selection a typed policy decision.

Deliverables:

- Source/environment origin identifiers and production/test separation.
- HTTPS-required production metadata.
- Same-origin redirect policy representation.
- Negative tests for arbitrary, downgraded, and cross-source origins.

Verification:

- Inherited gate plus SSRF-oriented host, port, scheme, and redirect fixtures.
- Review every accepted origin as static source evidence.

Exit criteria:

- Caller data cannot become an origin or credential destination.
- `v0.6.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.7.0 - Operation Policy Types

Goal: model access, hosted use, data class, cache, attribution, and retry rules.

Deliverables:

- Dependency-free policy enums and validated operation policy.
- Fail-closed `Unknown` and `ReviewRequired` states.
- Contradiction checks and decision tests.
- Policy documentation.

Verification:

- Inherited gate plus exhaustive allow/deny matrix tests.
- Confirm missing fields never produce permissive defaults.

Exit criteria:

- No operation can be registered without a complete fail-closed policy.
- `v0.7.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.8.0 - Policy Manifest Format

Goal: store source rules as reviewable, deterministic data.

Deliverables:

- Minimal first-party manifest grammar for required policy fields.
- Bounded parser with duplicate, unknown, missing, and contradictory-field
  rejection.
- Canonical formatter and round-trip fixtures.
- Dated review and expiry fields.

Verification:

- Inherited gate plus malformed corpus, depth/size limits, and round trips.
- No build-time network and no parser panic on arbitrary bytes.

Exit criteria:

- Checked-in manifests compile to exactly one deterministic policy value.
- `v0.8.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.9.0 - Provenance Envelope

Goal: keep source truth and transformation history attached to results.

Deliverables:

- Source, operation, schema, policy, retrieval, licence, and transform metadata.
- Raw/decoded/normalized/cache status distinctions.
- Bounded transformation records.
- Provenance equality and serialization test vectors.

Verification:

- Inherited gate plus missing/contradictory provenance tests.
- Confirm cache and transformation steps cannot erase original identity.

Exit criteria:

- Every future successful operation can carry complete source provenance.
- `v0.9.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.10.0 - Blocking Transport Contract

Goal: stabilize a caller-supplied synchronous execution boundary.

Deliverables:

- Create and publish the focused `sweden-http` crate.
- Credential-free request input, bounded response sink, safe metadata, and
  transport error contract.
- Deadline, redirect, and cancellation semantics.
- Deterministic recording mock.
- No concrete HTTP or TLS implementation.

Verification:

- Inherited gate plus timeout, truncation, redirect, over-budget, and partial
  response tests.
- Confirm `sweden-core` does not depend outward and the facade does not enable
  transport behavior by default.

Exit criteria:

- A mock can execute a plan without creating an arbitrary network path.
- `v0.10.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.11.0 - Async Transport Contract

Goal: support asynchronous transports without a runtime dependency.

Deliverables:

- Standard `Future`-based trait boundary and explicit cancellation semantics.
- Borrowed body sink and backpressure contract.
- Runtime-neutral async mock.
- Blocking/async semantic parity table.

Verification:

- Inherited gate plus manual-poll, cancellation, pending, and error tests.
- Prove no executor, timer, socket, or allocation dependency is introduced.

Exit criteria:

- Blocking and async plans enforce identical source and budget policy.
- `v0.11.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.12.0 - Bounded Body Pipeline

Goal: account for bytes before any untrusted response is decoded.

Deliverables:

- Wire/decoded byte counters, chunk sink, completion state, and truncation
  detection.
- Content-length and decompression-plan policy.
- Backpressure and abort results.
- Fault-injection test support.

Verification:

- Inherited gate plus exact-limit, one-byte-over, partial, repeated-completion,
  and counter-overflow tests.

Exit criteria:

- No decoder can receive bytes that bypass the declared response budget.
- `v0.12.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.13.0 - JSON Lexical Layer

Goal: tokenize the reviewed JSON subset under strict budgets.

Deliverables:

- First-party UTF-8, string escape, number, literal, and punctuation scanner.
- Token, string, and byte limits.
- Stable lexical errors with safe offsets.
- No allocation in borrowed mode.

Verification:

- Inherited gate plus RFC-oriented valid fixtures, malformed UTF-8, invalid
  escape/number, truncation, and mutation corpus.

Exit criteria:

- Arbitrary input produces tokens or a bounded error without panic.
- `v0.13.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.14.0 - JSON Structural Layer

Goal: parse bounded JSON structure without recursion hazards.

Deliverables:

- Explicit-stack object/array parser with depth and member limits.
- Duplicate-key policy and exact-consumption mode.
- Borrowed event stream.
- Source-decoder hooks.

Verification:

- Inherited gate plus deep nesting, duplicate keys, trailing data, empty
  structures, and token-budget exhaustion tests.

Exit criteria:

- Structure parsing cannot exceed configured stack or collection budgets.
- `v0.14.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.15.0 - JSON Owned Values

Goal: add bounded owned JSON only for callers that opt into allocation.

Deliverables:

- `alloc`-gated strings, arrays, and maps with pre-allocation checks.
- Configurable unknown-field capture.
- Allocation failure and total-owned-byte errors.
- Borrowed/owned parity tests.

Verification:

- Inherited gate plus feature-isolation and allocation-budget tests.
- Default builds remain allocation-free and `no_std`.

Exit criteria:

- Owned decoding cannot allocate beyond caller-approved public limits.
- `v0.15.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.16.0 - XML Lexical Layer

Goal: tokenize the XML subset required by Trafikverket without entity risk.

Deliverables:

- Bounded element, attribute, text, comment, and declaration scanning.
- Unconditional DTD and entity-declaration rejection.
- XML character and UTF-8 validation.
- Namespace token representation.

Verification:

- Inherited gate plus XXE/entity, malformed name, invalid character, truncation,
  and mutation fixtures.

Exit criteria:

- External/internal entity expansion is unrepresentable and rejected.
- `v0.16.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.17.0 - XML Structural Decoder

Goal: provide streaming, bounded XML element processing.

Deliverables:

- Explicit-stack start/end matching and namespace scope.
- Depth, attribute, text, and token limits.
- Exact-consumption and duplicate singleton policy.
- Borrowed event interface.

Verification:

- Inherited gate plus mismatched tags, namespace shadowing, deep input, repeated
  singleton, and budget tests.

Exit criteria:

- Arbitrary XML bytes cannot trigger recursion, entity expansion, or panic.
- `v0.17.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.18.0 - Canonical XML Encoder

Goal: construct deterministic Trafikverket requests without string assembly.

Deliverables:

- Bounded element/attribute/text writer with correct escaping.
- Canonical ordering rules and invalid-character rejection.
- Secret placeholder separation.
- Golden byte fixtures.

Verification:

- Inherited gate plus every escaping boundary, output budget, duplicate, and
  canonical stability test.

Exit criteria:

- Stable request construction uses only typed encoder operations.
- `v0.18.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.19.0 - Deterministic Testkit

Goal: make every operation testable without live services.

Deliverables:

- Create and publish the focused `sweden-testkit` crate.
- Mock transport, scripted faults, bounded recording, replay metadata, and
  secret/header allowlists.
- Synthetic fixture builder.
- First-party deterministic mutation runner.

Verification:

- Inherited gate plus self-tests proving redaction, limit enforcement, fault
  order, and replay determinism.

Exit criteria:

- Tests cannot accidentally record credentials or unbounded payloads.
- `v0.19.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.20.0 - Source Onboarding Compiler

Goal: turn reviewed operation metadata into code, policy, docs, and tests.

Deliverables:

- Create and publish the offline `sweden-schema` tool crate.
- Deterministic source/operation registry generation.
- Manifest hashes and generated-file headers.
- Fail-closed scaffold for a new agency.

Verification:

- Inherited gate plus regeneration-with-no-diff and malformed manifest tests.
- Build remains network-free.

Exit criteria:

- One checked-in source definition deterministically yields all declared
  artifacts.
- `v0.20.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.21.0 - Synthetic Conformance Source

Goal: prove the full architecture before touching production credentials.

Deliverables:

- Local-only source with open, denied, oversized, malformed, rate-limited, and
  stale-policy operations.
- Blocking and async mock execution.
- JSON and XML fixture paths.
- Generated docs and policy tests.

Verification:

- Inherited gate plus end-to-end allow/deny, provenance, redaction, and budget
  scenarios.

Exit criteria:

- The local source exercises every shared boundary without external I/O.
- `v0.21.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.22.0 - Trafikverket Source Dossier

Goal: freeze current official evidence before implementation.

Deliverables:

- Create and publish the initial dependency-free, `no_std`
  `sweden-trafikverket` crate.
- Official documentation, terms, origins, access, licence, rate, attribution,
  privacy, and support inventory.
- Retrieval dates, content hashes, review expiry, and responsible reviewer.
- Operation/object inventory with explicit exclusions.
- Hosted-use decision for each candidate operation.

Verification:

- Inherited gate plus manual source-review checklist and policy contradiction
  checks.

Exit criteria:

- No Trafikverket code relies on an undocumented or unreviewed source claim.
- `v0.22.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.23.0 - Trafikverket Environment And Credential Plan

Goal: separate origins and credentials before any request can execute.

Deliverables:

- Test/production environment types and closed origins.
- Late API-key injection contract.
- Credential-free canonical request and cache identity.
- Redaction and wrong-origin negative tests.

Verification:

- Inherited gate plus marker-secret snapshots across errors, debug, requests,
  hashes, fixtures, and mock recordings.

Exit criteria:

- A credential can be sent only to its reviewed environment and source.
- `v0.23.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.24.0 - Trafikverket Raw Reviewed Operation

Goal: prove one exact official query through a typed, bounded raw boundary.

Deliverables:

- One registered operation ID and request/response policy.
- Typed inputs, canonical XML, strict response envelope, and synthetic fixture.
- Mock execution and opt-in official conformance command.
- Explicit experimental status.

Verification:

- Inherited gate plus request golden, response limits, upstream error, and
  redaction tests.

Exit criteria:

- Exactly one documented operation works without opening an arbitrary raw URL.
- `v0.24.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.25.0 - Trafikverket Field Metadata

Goal: represent upstream fields and availability without generated API leakage.

Deliverables:

- Value kind, wire name, nullability, operators, version availability, and
  sensitivity metadata.
- Stable hand-written field facade.
- Unknown future-field representation.
- Invalid metadata tests.

Verification:

- Inherited gate plus operator/type, version, duplicate wire name, and unknown
  field tests.

Exit criteria:

- Field metadata cannot authorize an invalid operator or version.
- `v0.25.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.26.0 - Trafikverket Filter AST

Goal: make filters typed and complexity-bounded.

Deliverables:

- Comparisons, Boolean composition, list/range forms, and null semantics.
- Predicate count, depth, and estimated-cost budgets.
- No raw stable filter fragments.
- Canonical filter encoding.

Verification:

- Inherited gate plus compile-time type examples, complexity exhaustion,
  canonical goldens, and mutation tests.

Exit criteria:

- Stable filters cannot express unsupported field/operator combinations.
- `v0.26.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.27.0 - Trafikverket Projection And Ordering

Goal: bound selected fields and deterministic result ordering.

Deliverables:

- Typed projections, ordering directions, field limits, and duplicate checks.
- Version-availability validation.
- Canonical encoder integration.
- Stable errors and examples.

Verification:

- Inherited gate plus wrong-object, duplicate, unavailable, and over-budget
  projection/order tests.

Exit criteria:

- Projection and ordering cannot bypass field or query budgets.
- `v0.27.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.28.0 - Trafikverket Paging And Collection Budgets

Goal: prevent accidental unbounded result collection.

Deliverables:

- Page request/cursor types and explicit collection budget.
- Page, record, byte, and elapsed limits.
- Streaming-first iteration contract.
- Resume and early-stop behavior.

Verification:

- Inherited gate plus zero/overflow, repeated cursor, endless source, early
  stop, and each budget exhaustion test.

Exit criteria:

- No public all-pages helper exists without an explicit total budget.
- `v0.28.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.29.0 - Trafikverket Response Envelope

Goal: decode success and upstream failures without semantic ambiguity.

Deliverables:

- Strict outer envelope, source error, version, request ID, and partial-body
  states.
- Bounded unknown-field policy.
- Safe schema mismatch paths.
- Raw/decoded provenance attachment.

Verification:

- Inherited gate plus truncation, mixed success/error, duplicate singleton,
  unknown field, and oversized envelope fixtures.

Exit criteria:

- A malformed or ambiguous envelope always fails closed.
- `v0.29.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.30.0 - Trafikverket Schema Snapshot Tooling

Goal: make upstream model changes reviewable and reproducible.

Deliverables:

- Offline snapshot manifest with official reference, time, version, hash, and
  licence.
- Deterministic generator and hand-written patch overlay.
- Add/remove/type/requiredness/auth/terms change classification.
- No automatic merge or build-time fetch.

Verification:

- Inherited gate plus no-diff regeneration and intentionally changed schema
  classification fixtures.

Exit criteria:

- Generated models can be reproduced entirely from reviewed checked-in inputs.
- `v0.30.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.31.0 - Trafikverket Rail Model Slice A

Goal: implement one small, named rail object family.

Deliverables:

- Generated wire types, stable hand-written facade, semantic validation, and
  synthetic fixtures for the selected family.
- Operation inventory coverage entry and known gaps.
- Streaming decode example.

Verification:

- Inherited gate plus request goldens, valid/invalid fixture corpus, unknown
  field, and live canary where permitted.

Exit criteria:

- Only the documented slice is marked experimental; no wider coverage claim.
- `v0.31.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.32.0 - Trafikverket Rail Model Slice B

Goal: add the next independently reviewable rail object family.

Deliverables:

- Types, facade, validation, fixtures, examples, and coverage delta.
- Cross-slice identifier consistency tests.
- Upstream version constraints.

Verification:

- Inherited gate plus slice-specific semantic, malformed, budget, and canary
  evidence.

Exit criteria:

- Slice B adds no unreviewed behavior to slice A.
- `v0.32.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.33.0 - Trafikverket Road Model Slice A

Goal: implement one small, named road object family.

Deliverables:

- Types, facade, bounded geo/time values, validation, fixtures, and examples.
- Explicit coordinate/unit semantics.
- Coverage and unsupported tables.

Verification:

- Inherited gate plus coordinate, unit, time, schema, and live canary tests
  where permitted.

Exit criteria:

- Only the reviewed road slice is exposed as experimental.
- `v0.33.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.34.0 - Trafikverket Road Model Slice B

Goal: add the next independently reviewable road object family.

Deliverables:

- Types, facade, validation, fixtures, examples, and coverage delta.
- Cross-slice identifier and unit compatibility.
- Updated source dossier.

Verification:

- Inherited gate plus slice-specific malformed, semantic, budget, and canary
  evidence.

Exit criteria:

- Slice B remains isolated and does not weaken prior validation.
- `v0.34.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.35.0 - Trafikverket Remaining Stable Slice Inventory

Goal: close declared 1.0 object coverage in small generated modules.

Deliverables:

- Final selected slices, explicit unsupported list, raw registered escape
  hatch, and complete coverage matrix.
- Per-slice fixtures and stable facade review.
- No arbitrary object or field names in stable APIs.

Verification:

- Inherited gate plus operation-by-operation fixture, policy, documentation,
  and compatibility checks.

Exit criteria:

- Every claimed stable object is documented and every gap is explicit.
- `v0.35.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.36.0 - Trafikverket Change Checkpoints

Goal: model reviewed incremental-update semantics without exactly-once claims.

Deliverables:

- Opaque source/environment/object-scoped checkpoint.
- Commit-after-processing flow, resume, deduplication guidance, and invalidation.
- At-least-once contract.
- Crash/restart fixtures.

Verification:

- Inherited gate plus crash before/after commit, stale version, wrong source,
  duplicate delivery, and truncation tests.

Exit criteria:

- No acknowledged-but-unprocessed checkpoint advance is possible.
- `v0.36.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.37.0 - Rate Limit And Retry Enforcement

Goal: make source protection executable for direct and hosted callers.

Deliverables:

- Minimum interval, fixed window, token budget, concurrency, and daily cap
  semantics required by reviewed operations.
- Idempotency-aware retries, `Retry-After`, jitter input, and total deadline.
- Fail-closed limiter failure policy.

Verification:

- Inherited gate plus deterministic clock, 429, limiter outage, concurrency,
  retry storm, and deadline exhaustion tests.

Exit criteria:

- Retry behavior cannot exceed operation or source budgets.
- `v0.37.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.38.0 - Cache And Freshness Contracts

Goal: represent cache decisions without violating source terms.

Deliverables:

- Policy-versioned non-secret keys, raw/derived distinction, freshness modes,
  and purge dimensions.
- Tenant partition input reserved from day one.
- Provenance preservation across hits.
- Prohibited-cache tests.

Verification:

- Inherited gate plus credential exclusion, collision, stale, policy-change,
  raw/derived, and purge tests.

Exit criteria:

- Policy denial always overrides caller cache preference.
- `v0.38.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.39.0 - Public API Ergonomics Review

Goal: make safe bounded behavior the obvious API path.

Deliverables:

- Blocking, async, custom transport, streaming, provenance, error, and mock
  examples.
- Public naming and feature review.
- No hidden global client or implicit network behavior.
- Semver surface report.

Verification:

- Inherited gate plus compile-tested examples, feature matrix, and independent
  usability review.

Exit criteria:

- Common tasks require no raw strings, arbitrary URLs, or unbudgeted helpers.
- `v0.39.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.40.0 - Cross-Platform Baseline

Goal: prove portable crate boundaries before the API freezes.

Deliverables:

- Linux, Windows, macOS, FreeBSD, Android, and iOS compile evidence.
- Platform capability table and adapter error semantics.
- Endianness, pointer-width, time, path, and line-ending review.
- Aesynx future-adapter design note without unfinished integration.

Verification:

- Inherited gate plus native CI tests and cross-target checks for portable
  crates.

Exit criteria:

- No agency or core API assumes a specific operating system.
- `v0.40.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.41.0 - Performance And Memory Budgets

Goal: establish measurable resource ceilings before production claims.

Deliverables:

- Parser, encoder, streaming, request-plan, and model-size benchmarks.
- Stack and allocation measurements.
- Worst-case budget documentation.
- Regression thresholds using first-party harnesses.

Verification:

- Inherited gate plus reproducible benchmark fixtures and boundary measurements
  on MSRV and pinned stable.

Exit criteria:

- Every stable path has a documented memory and work budget.
- `v0.41.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.42.0 - Parser Fuzz And Mutation Baseline

Goal: harden every handwritten byte parser without project dependencies.

Deliverables:

- Deterministic mutation harnesses and committed seed corpora.
- JSON, XML, policy, response, checkpoint, and provenance targets.
- Crash minimization and regression workflow.
- Extended out-of-process fuzz runbook.

Verification:

- Inherited gate plus fixed-duration local mutation smoke and documented
  extended run with zero unresolved crashes.

Exit criteria:

- Every untrusted parser has corpus replay and no-panic mutation coverage.
- `v0.42.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.43.0 - Documentation And Source Evidence Audit

Goal: prove public documentation matches executable metadata and current sources.

Deliverables:

- Generated operation, access, rate, attribution, data-class, and compatibility
  tables.
- Compile-tested examples and migration policy.
- Current source dossier hashes.
- Claim/implementation cross-check.

Verification:

- Inherited gate plus doc-link, generated-doc reproducibility, doctest, and
  source-review expiry checks.

Exit criteria:

- No public stable claim lacks code, tests, policy, and source evidence.
- `v0.43.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.44.0 - Independent Package Boundary Audit

Goal: prove every crate works as a crates.io package.

Deliverables:

- Dependency-order package script and package-content allowlists.
- README/licence/source inclusion checks.
- No undeclared path-only or GitHub-only crate.
- Publish rollback and owner checklist.

Verification:

- Inherited gate plus isolated `cargo package` for every crate using local
  registry patches only for already-versioned workspace dependencies.

Exit criteria:

- Every crate archive is independently complete and within registry limits.
- `v0.44.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.45.0 - Service Operation Registry

Goal: reuse SDK operation and policy metadata without creating a generic proxy.

Deliverables:

- Published `sweden-service-core` crate with explicit operation allowlist.
- Request context, policy preflight, and source worker boundary.
- No arbitrary URL, header, or method route.
- Synthetic service tests only.

Verification:

- Inherited gate plus unregistered operation, arbitrary host/header, expired
  policy, and over-budget denial tests.

Exit criteria:

- Service dispatch can invoke only registered reviewed operations.
- `v0.45.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.46.0 - Tenant And Project Context

Goal: make future hosted authority explicit in every service call.

Deliverables:

- Tenant, project, key ID, entitlement, quota, agreement, and correlation
  context types.
- No ambient or thread-local security context.
- Mandatory context in cache, limiter, credential, and audit interfaces.
- Cross-tenant negative fixtures.

Verification:

- Inherited gate plus missing/wrong tenant, key, agreement, cache, limiter, and
  audit routing tests.

Exit criteria:

- A service operation cannot access protected state without explicit context.
- `v0.46.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.47.0 - Gateway Key And Scope Model

Goal: define revocable public gateway credentials without storing plaintext.

Deliverables:

- Key format, one-time display, verifier boundary, scopes, rotation overlap,
  revocation, and safe last-used metadata.
- No URL credentials.
- Test-only verifier implementation.
- Audit events.

Verification:

- Inherited gate plus wrong scope, revoked, expired, rotated, malformed,
  cross-project, logging, and timing-policy tests.

Exit criteria:

- Plaintext gateway keys never enter persistent or diagnostic interfaces.
- `v0.47.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.48.0 - Hosted Cache And Limiter Semantics

Goal: bind source rules and tenant isolation to hosted execution.

Deliverables:

- Atomic limiter storage interface, fail-closed strict limits, single-flight
  cache fill, partitioning, and purge.
- Source kill switch and policy-expiry transition.
- Deterministic store-failure simulation.

Verification:

- Inherited gate plus cross-tenant collision, store outage, stampede, kill
  switch, stale policy, and quota-amplification tests.

Exit criteria:

- A storage failure cannot flood an authority or cross tenant boundaries.
- `v0.48.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.49.0 - Payload-Free Observability

Goal: make operations diagnosable without collecting source payloads or secrets.

Deliverables:

- Allowlisted log fields, metric labels, trace fields, audit events, and
  redacted support bundle schema.
- Explicit secure-debug policy kept off by default.
- Personal-data and high-cardinality label rejection.

Verification:

- Inherited gate plus marker secrets/payloads across every diagnostic sink.
- Cardinality and retention policy review.

Exit criteria:

- Default observability contains identifiers and counts, never payload content.
- `v0.49.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.50.0 - Security Architecture Review

Goal: perform a complete pre-beta adversarial review of implemented boundaries.

Deliverables:

- Updated threat model, attack-surface inventory, abuse cases, and control map.
- SSRF, parser, secret, tenant, policy drift, rate, cache, supply-chain, and
  release reviews.
- Independent review report and tracked remediation.
- Unsafe/dependency attestation.

Verification:

- Inherited gate plus extended mutation runs and every security regression suite.

Exit criteria:

- No critical/high finding remains and medium findings have explicit treatment.
- `v0.50.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.60.0 - Legal And Privacy Readiness

Goal: admit only operations with current lawful hosted/direct-use decisions.

Deliverables:

- Operation-level source dossiers, attribution, retention, redistribution,
  transformation, and hosted-use decisions.
- Privacy records, deletion paths, DPIA decision, acceptable-use policy, and
  subprocessor inventory for hosted scope.
- Fail-closed unknown data class.

Verification:

- Inherited gate plus policy expiry, deletion, retention, attribution, direct
  only, agreement expiry, and unknown-class tests.

Exit criteria:

- Every enabled operation has current technical and legal evidence.
- `v0.60.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.70.0 - Reliability And Recovery

Goal: prove bounded behavior during sustained failures and operational recovery.

Deliverables:

- Load, upstream outage, limiter outage, cache stampede, schema mismatch,
  backup/restore, rollback, and source-disable exercises.
- Capacity and recovery objectives.
- Incident and source-authority contact runbooks.

Verification:

- Inherited gate plus repeatable failure drills under source limits.

Exit criteria:

- Recovery does not exceed upstream policy or lose tenant/source isolation.
- `v0.70.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.80.0 - Limited Public Beta

Goal: validate the stable facade and Trafikverket scope with controlled users.

Deliverables:

- Limited beta, support process, migration feedback, compatibility report, and
  documented telemetry review.
- No new broad capability during the beta window.
- Source feedback and issue triage.

Verification:

- Inherited gate plus clean-install scenarios and beta issue regression suite.

Exit criteria:

- No unresolved architectural, security, legal, or usability blocker remains.
- `v0.80.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.90.0 - Release Candidate One

Goal: freeze the intended 1.0 API and exact production scope.

Deliverables:

- API freeze, current Trafikverket schema/terms, complete docs, SBOM/provenance
  evidence, migration guide, and production configuration review.
- Only bug, security, legal, and documentation changes after this point.

Verification:

- Inherited gate plus clean machine, package, platform, live conformance,
  recovery, and semver audits.

Exit criteria:

- The exact candidate is suitable for independent final assessment.
- `v0.90.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.95.0 - Release Candidate Remediation

Goal: resolve RC1 findings without expanding the frozen product.

Deliverables:

- RC1 fixes, regression tests, refreshed docs/evidence, and compatibility report.
- Explicit deferral of non-blocking new ideas.

Verification:

- Inherited gate plus every RC1 finding reproducer and full release-candidate
  suite.

Exit criteria:

- RC1 findings are resolved without unreviewed feature growth.
- `v0.95.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.99.0 - Final Acceptance

Goal: assemble and verify all exact 1.0 acceptance evidence.

Deliverables:

- Signed checklist, current source/legal reviews, release archives, package
  order rehearsal, rollback rehearsal, and final non-claim review.
- No known release blocker.

Verification:

- Inherited gate plus final independent security assessment, full compatibility
  matrix, live conformance, clean install, and recovery drill.

Exit criteria:

- Every 1.0 criterion is supported by current reproducible evidence.
- `v0.99.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v1.0.0-rc.1 - Exact Production Candidate

Goal: test the exact `1.0.0` code and metadata under a prerelease version.

Deliverables:

- `1.0.0-rc.1` versions, packages, docs, provenance, and deployment candidate.
- No code change from final acceptance except version/metadata necessities.

Verification:

- Full inherited gate, every supported Rust/platform/package test, current live
  conformance, GitHub/CodeQL review, and maintainer pentest recorded in the
  versioned report.

Exit criteria:

- Only release-blocking remediation may follow.
- `v1.0.0-rc.1 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v1.0.0 - Serious Production Release

Goal: release the first serious production-ready Sweden ecosystem for its
documented Trafikverket and shared-core scope.

Deliverables:

- Stable `sweden-core`, `sweden-http`, `sweden-trafikverket`, and `sweden`
  facade.
- Complete declared Trafikverket operation/object matrix.
- Current source policy, provenance, rate, retry, cache, and attribution
  behavior.
- Full documentation, release evidence, and migration policy.

Verification:

- Full inherited gate plus exact equivalence to the accepted release candidate,
  final crate publication rehearsal, source status review, and maintainer final
  pentest recorded in the versioned report.

Exit criteria:

- All production guarantees are evidence-backed and no release blocker exists.
- `v1.0.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## Post-1.0 Agency Tracks

New agencies are created and published only when work on their own crate `0.x`
line begins. They use the same per-version format and pentest rule; no
placeholder exists before then. The root facade adds a feature only after that
agency crate reaches its documented stable release.

Planned facade milestones:

| Facade release | Agency crates admitted after independent stabilization |
| --- | --- |
| `1.1.0` | `sweden-smhi` |
| `1.2.0` | `sweden-scb` |
| `1.3.0` | `sweden-jobtech` open-data operations |
| `1.4.0` | `sweden-skatteverket` open-data operations |
| `1.5.0+` | Further one-source crates after individual source review |

Partner, targeted, paid, and legally sensitive operation families remain
separate from open-data features and direct-only unless exact hosted permission
is recorded. Adding an agency never justifies turning `sweden` into a monolith.
