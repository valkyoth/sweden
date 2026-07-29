# Sweden Release Plan To 1.0

Status: planning document; `v0.1.0` released; next implementation milestone is
`v0.2.0`

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

After `v0.21.0`, every live command additionally requires the executor's
one-attempt quota/concurrency admission, explicit credential scope, hard
response/deadline budgets, and disabled automatic retry/redirect unless a
later reviewed milestone has enabled them. Before `v0.21.0`, live execution is
prohibited.

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

- Canonical syntax, length ceilings, and normalization rules for source,
  operation, schema, policy, and upstream identifiers.
- Validating borrowed constructors for dynamic identifiers and generated
  closed constants for reviewed identifiers.
- Reviewed source constants reserved for later binding to dossier/policy
  evidence; `SourceId::reviewed` cannot remain a general assertion API.
- Non-zero version wrappers.
- Small payload-free `ValidationError` categories for stable constructors
  instead of ambiguous `Option` failures.
- Stable comparison and display rules without allocation.
- Boundary and invalid-input tests.
- Core API and crate documentation.

Verification:

- Inherited gate plus exhaustive empty, length, character, and boundary tests.
- Compile-fail proof that downstream callers cannot mint a reviewed identifier
  or stable execution authority.
- `no_std` compilation on MSRV and pinned stable.

Exit criteria:

- No unvalidated string is accepted as a stable identifier.
- `v0.2.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.3.0 - Bounded Scalar Types

Goal: prevent unbounded or ambiguous scalar input at API boundaries.

Deliverables:

- Bounded strings, page sizes, byte counts, retry counts, and nesting limits.
- Independent wire and decoded ceilings without an ordering assumption.
- Checked non-cloneable ledgers, child-ledger accounting, tighten-only
  ceilings, and stable exhaustion errors.
- Separation between configured ceilings and consumable remaining state.
- Work-unit and allocation-count primitives for paths that byte ceilings alone
  do not bound.
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
- Closed adapter error codes and opaque diagnostic IDs; arbitrary transport
  errors, URLs, headers, and bodies are never retained as an error source.
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

- `Operation<Input>` to credential-free `CanonicalPlan<Unauthenticated>`
  typestate with private authorization transitions.
- Method, reviewed structured path/query fields, protected credential slots,
  and bounded body plan.
- `Replayable` and `OneShot` body typestates so consumption and ambiguous
  delivery constrain all later retry/redirect decisions.
- Canonical credential-free representation.
- Explicit response and execution budgets.
- Advisory pre-I/O `Cost` derived from reviewed maxima, selected projection,
  encoded request size, and query-complexity units.
- Compile-time separation from `std`.

Verification:

- Inherited gate plus request canonicalization goldens and invalid path tests.
- Prove no arbitrary scheme, authority, or absolute URL is representable.

Exit criteria:

- Agency operations can describe a bounded request without choosing transport,
  and no public constructor can mint an authorized plan or arbitrary origin.
- `v0.5.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.6.0 - Closed Origin Registry

Goal: make SSRF-resistant source origin selection a typed policy decision.

Deliverables:

- Source/environment origin identifiers and production/test separation.
- HTTPS-required production metadata with canonical ASCII host and fixed or
  default port.
- Explicit structured path/query grammar that rejects controls, ambiguous
  encoding, user-info, scheme, authority, and host input.
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

- Create and publish the focused dependency-free `sweden-policy` crate.
- Dependency-free policy enums and validated operation policy.
- Stable predicates for non-exhaustive access/status enums so callers do not
  encode wildcard policy logic.
- Fail-closed `Unknown` and `ReviewRequired` states.
- Operation-specific access, authentication, hosted-use, data-class, cache,
  attribution, transformation, redistribution, retry, and pagination rules.
- Typed cache directives such as `Forbidden`, `Private`, `Revalidate`, and
  bounded freshness; callers may narrow but never broaden them.
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
- Official evidence references, retrieval time, content digest, reviewer,
  expiry, schema inputs, operation inventory, and explicit exclusions.
- Standard cryptographic digests computed by pinned offline tooling and
  represented as opaque reviewed values in portable code.
- Reviewer/trust-root binding, monotonic policy version, downgrade/rollback
  rejection, and kill-switch invalidation; the manifest states explicitly that
  a digest proves identity, not authenticity or lawful status.

Verification:

- Inherited gate plus malformed corpus, depth/size limits, and round trips.
- No build-time network and no parser panic on arbitrary bytes.

Exit criteria:

- Checked-in manifests compile to exactly one deterministic operation policy,
  and missing dossier evidence or expiry fails closed.
- `v0.8.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.9.0 - Provenance And Execution Capability

Goal: attach source truth to results and make reviewed authority unforgeable
before any executor exists.

Deliverables:

- Source, operation, schema, policy, retrieval, licence, and transform metadata.
- Raw/decoded/normalized/cache status distinctions.
- Bounded transformation records.
- Operational private evidence-bound capability and non-cloneable one-attempt
  permit tying operation, plan, origin, environment, policy/dossier/schema
  versions, reviewer trust root, expiry, quota requirement, and kill-switch
  state to later execution.
- Monotonic policy-version and rollback/downgrade rejection.
- Provenance equality and serialization test vectors.

Verification:

- Inherited gate plus missing/contradictory provenance, forged capability,
  wrong-plan/environment/origin, stale digest, rollback, expiry, kill-switch,
  and permit-reuse tests.
- Confirm cache and transformation steps cannot erase original identity.

Exit criteria:

- Every future successful operation can carry complete provenance, and later
  executors can accept authority only through this private capability/permit
  path.
- `v0.9.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.10.0 - `no_std` Transport Contract

Goal: create a portable sans-I/O request/response boundary before execution
styles are layered on it.

Deliverables:

- Create and publish the focused `sweden-http` crate.
- `#![no_std]` contract crate with no heap, socket, timer, executor, runtime,
  filesystem, environment, DNS, or TLS dependency.
- Credential-free request input, bounded response sink, safe metadata, and
  transport error contract.
- Redirect-as-data, cancellation-state, and backpressure contracts.
- No concrete HTTP or TLS implementation.

Verification:

- Inherited gate plus timeout, truncation, redirect, over-budget, and partial
  response tests.
- Compile checks proving the contract works without `std` or allocation.
- Confirm `sweden-core` does not depend outward and the facade does not enable
  transport behavior by default.

Exit criteria:

- A mock transport can exercise the contract values without owning policy,
  authorization, credentials, source decoding, or a network path.
- `v0.10.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.11.0 - Blocking And Async Transport Traits

Goal: support synchronous and asynchronous caller transports without a runtime
dependency.

Deliverables:

- Blocking trait plus standard `Future`-based trait boundary and explicit
  cancellation semantics.
- Static dispatch by default; heterogeneous boxed transport convenience only
  behind explicit `alloc`, with object-safety and MSRV behavior documented.
- Borrowed body sink and backpressure contract.
- Runtime-neutral async mock.
- Blocking/async semantic parity table.

Verification:

- Inherited gate plus manual-poll, cancellation, pending, and error tests.
- Prove no executor, timer, socket, or allocation dependency is introduced.

Exit criteria:

- Blocking and async transport contracts expose identical cancellation,
  backpressure, body-budget, and safe-error semantics; policy enforcement
  remains executor-owned.
- `v0.11.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.12.0 - Bounded Body Pipeline

Goal: account for bytes before any untrusted response is decoded.

Deliverables:

- Independent wire/decoded consumable ledgers, chunk sink, completion state,
  and truncation detection.
- Header-byte/count, chunk-count, decoded-work, and UTF-8 carry limits.
- Identity request encoding by default; unsupported response
  `Content-Encoding` is rejected and adapter-side transparent decompression is
  forbidden.
- If decompression is later admitted, its boundary sits between independent
  wire and decoded ledgers and charges output before exposure.
- Bounded status, media type, charset, header/trailer, and informational
  response handling plus content-length preflight.
- Backpressure and abort results.
- Fault-injection test support.
- Pre-charge before chunk acceptance or decoded-fragment exposure.

Verification:

- Inherited gate plus exact-limit, one-byte-over, partial, repeated-completion,
  and counter-overflow tests.

Exit criteria:

- No decoder can receive bytes that bypass the declared response budget, and
  wire and decoded ceilings remain independently configurable.
- `v0.12.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.13.0 - JSON Lexical Layer

Goal: tokenize the reviewed JSON subset under strict budgets.

Deliverables:

- Create and publish the focused dependency-free `sweden-codec-json` crate.
- First-party UTF-8, string escape, exact JSON number, literal, and punctuation
  scanner.
- Independent raw-string, decoded-string, number-digit, exponent-digit, token,
  work-unit, and byte limits.
- Leading BOM rejection and no permissive platform float parsing.
- Unicode scalar validation, including surrogate-pair handling and rejection
  of isolated surrogates.
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

- Iterative object/array parser using a caller-provided stack with depth,
  member, element, and total-token limits.
- Operation-selected duplicate-key policy that rejects duplicates by default.
- Exact consumption after permitted trailing whitespace.
- Borrowed event stream with caller scratch for decoded strings.
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

- `alloc`-gated strings, arrays, and maps with checked pre-charge before every
  reserve or growth.
- Checked integer/float conversion with overflow, non-finite, range, and
  operation-selected negative-zero policy.
- Configurable unknown-field capture.
- Allocation failure, logical-owned-byte, and cumulative-allocation errors.
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

- Create and publish the focused dependency-free `sweden-codec-xml` crate.
- Bounded element, attribute, text, comment, and declaration scanning.
- Early unconditional rejection of `DOCTYPE`, internal subsets, entity
  declarations, and external identifiers before any subset is scanned.
- No entity table, external resolver, XInclude/resource callback, or
  unreviewed markup declaration.
- Only the five predefined entities and bounded numeric character references.
- XML character and UTF-8 validation.
- Conflicting or unsupported XML encoding declarations are rejected.
- Explicit processing-instruction, XML-declaration placement, CDATA, and
  comment policy; comments containing `--` are rejected.
- Decoded-byte charge before character-reference expansion.
- Namespace token representation.

Verification:

- Inherited gate plus XXE/entity, malformed name, invalid character, truncation,
  and mutation fixtures, including every chunk split across DTD, entity,
  declaration, CDATA, and processing-instruction prefixes.

Exit criteria:

- External/internal entity expansion is unrepresentable and rejected.
- `v0.16.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.17.0 - XML Structural Decoder

Goal: provide streaming, bounded XML element processing.

Deliverables:

- Iterative start/end matching with caller-provided stack and namespace scope.
- Depth, attribute, text, and token limits.
- Exact expanded-name matching, duplicate expanded-attribute rejection,
  reserved-prefix enforcement, exact-consumption, and duplicate singleton
  policy.
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
- Checked output pre-charge before any write, including escaped expansion.
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
- Fail-closed recording for unknown header classes and transport conformance
  fixtures for redirect, proxy, decompression, cancellation, and truncation.
- Closed adapter diagnostics that cannot preserve an arbitrary underlying
  error as `source()`.
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
- Deterministic source/operation registry, closed identifier constant,
  capability, documentation, and contradiction-test generation.
- Manifest hashes and generated-file headers.
- Generated-file inventory with family-based splitting that keeps every Rust
  source file below 500 lines.
- Fail-closed scaffold for a new agency.
- Generated crate-introduction and phase data checked against the release and
  version plans.
- Explicit ceilings for input bytes, files, nodes, depth, references, generated
  items, output bytes, and work units.
- Rejection of cyclic/remote references, path traversal, normalized identifier
  collisions, duplicate wire names, malformed Unicode, Rust/rustdoc/string
  injection, unsupported constructs, and output explosion.
- Canonical path handling and deterministic ordering.

Verification:

- Inherited gate plus regeneration-with-no-diff, every ceiling, cyclic/remote
  reference, collision, traversal, injection, explosion, Unicode, and
  malformed manifest tests.
- Build remains network-free.

Exit criteria:

- One checked-in source definition deterministically yields all declared
  artifacts.
- `v0.20.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.21.0 - Synthetic Conformance Source

Goal: introduce the single generic executor owner and prove the full
architecture before touching production credentials.

Deliverables:

- Create and publish dependency-free `no_std` `sweden-executor`.
- Generic authorization transitions, capability/quota-permit consumption,
  late credential injection, redirect/retry state machines, response
  sink/decoder driving, and optional `alloc`-gated `Client<T, C, Q, K>`.
- Local-only source with open, denied, oversized, malformed, rate-limited, and
  stale-policy operations.
- Blocking and async mock execution.
- JSON and XML fixture paths.
- Generated docs, policy tests, expiry tests, permit-consumption tests, and a
  deliberately non-conforming trusted-transport demonstration.
- Minimal live-execution guard requiring one admitted attempt, concurrency
  admission, operation/source quota decision, explicit credential scope, hard
  response/deadline budgets, and no automatic retry or redirect.

Verification:

- Inherited gate plus end-to-end allow/deny, provenance, redaction, and budget
  scenarios.

Exit criteria:

- The local source exercises every shared boundary without external I/O, the
  executor is the sole owner of generic execution, and no later live command
  can bypass the minimal live-execution guard.
- `v0.21.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.22.0 - Trafikverket Source Dossier

Goal: freeze current official evidence before implementation.

Deliverables:

- Create and publish the initial dependency-free, `no_std`
  `sweden-trafikverket` crate.
- Official documentation, terms, origins, access, licence, rate, attribution,
  privacy, and support inventory per candidate operation.
- Retrieval dates, content hashes, review expiry, and responsible reviewer.
- Operation/object inventory with explicit exclusions and schema/spec inputs.
- At most four named 1.0 model-slice slots mapped to
  `v0.31.0..=v0.34.0`; an unused slot is an explicit no-slice decision and
  additional families remain post-1.0.
- Authentication, redirect, retry, pagination, cache, transformation,
  redistribution, hosted-use, and data-class decisions for each candidate
  operation.

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
- Late API-key injection into a narrow Sweden-controlled execution sink.
- Credential-free canonical request and cache identity plus a distinct
  ephemeral wire target for source-mandated query credentials.
- Secret/provider types without revealing `Debug`, `Display`, `Hash`,
  serialization, `Copy`, `Clone`, or public byte getters.
- Redaction, full-URL exclusion, and wrong-origin negative tests.
- Explicit trust statement for caller transports and deployment-owned DNS,
  TLS, proxy, logging, and egress controls.

Verification:

- Inherited gate plus marker-secret snapshots across errors, debug, requests,
  hashes, fixtures, and mock recordings.

Exit criteria:

- Sweden-controlled execution can send a credential only to its reviewed
  environment and source; arbitrary caller transports remain trusted and are
  never described as sandboxed.
- `v0.23.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.24.0 - Trafikverket Raw Reviewed Operation

Goal: prove one exact official query through a typed, bounded raw boundary.

Deliverables:

- One registered operation ID and request/response policy.
- Typed inputs, canonical XML, strict response envelope, and synthetic fixture.
- Use the `v0.9.0` non-cloneable capability and execution permit bound to the
  operation, environment, origin, policy/dossier/schema evidence, and review
  expiry.
- Mock execution and an opt-in official conformance command gated by the
  `v0.21.0` one-attempt quota/concurrency/credential/deadline guard; retries and
  redirects remain disabled until their reviewed enforcement exists.
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

- Page request/cursor types and an explicit consumable collection ledger.
- Page, record, wire-byte, decoded-byte, retry, allocation, and overall
  deadline limits charged before work.
- Streaming-first explicit `next_page` contract; no `Iterator` hides I/O.
- Resume and early-stop behavior, unchanged-cursor rejection, and bounded
  cursor-cycle detection.

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
- Deterministic generator and hand-written patch overlay whose manifest records
  rationale, evidence reference, reviewer, affected items, and deterministic
  diff.
- Overlay restrictions preventing silent changes to wire truth,
  authentication, origin, access policy, or reviewed resource maxima.
- Add/remove/type/requiredness/auth/terms change classification.
- No automatic merge or build-time fetch.

Verification:

- Inherited gate plus no-diff regeneration and intentionally changed schema
  classification fixtures.
- Reject unmanifested overlays and overlays that alter protected wire,
  authentication, origin, policy, or maximum fields.

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

## v0.35.0 - Trafikverket Stable Slice Completeness Audit

Goal: audit the dossier-bounded slice set without adding a catch-all object
family.

Deliverables:

- Complete coverage matrix for the named slice slots implemented in
  `v0.31.0..=v0.34.0`, including an explicit no-slice decision for any unused
  slot.
- Explicit unsupported list and stable facade review.
- No arbitrary object or field names in stable APIs.
- No generated model, operation, or object family added in this milestone.

Verification:

- Inherited gate plus operation-by-operation fixture, policy, documentation,
  and compatibility checks.

Exit criteria:

- Every claimed stable object was introduced in an earlier named slice
  milestone, is documented, and has complete evidence; everything else is
  unsupported for 1.0.
- `v0.35.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.36.0 - Trafikverket Change Checkpoints

Goal: model reviewed incremental-update semantics without exactly-once claims.

Deliverables:

- Bounded opaque checkpoint and cursor bound to source, operation, environment,
  object family, schema version, policy version, and safe redaction rules.
- Advance only after successful decoding and explicit caller acknowledgement;
  resume, deduplication guidance, and invalidation.
- At-least-once contract.
- Explicit statement that durable downstream processing and checkpoint
  persistence remain caller responsibilities.
- Crash/restart fixtures.

Verification:

- Inherited gate plus crash before/after commit, stale version, wrong source,
  duplicate delivery, and truncation tests.

Exit criteria:

- Sweden never issues or persists an advanced checkpoint before successful
  decoding and explicit caller acknowledgement; durable downstream processing
  remains the caller's responsibility.
- `v0.36.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.37.0 - Rate Limit And Retry Enforcement

Goal: make source protection executable for direct and coordinated SDK
deployments.

Deliverables:

- Minimum interval, fixed window, token budget, concurrency, and daily cap
  semantics required by reviewed operations.
- Idempotency-aware retries, `Retry-After`, jitter input, and total deadline.
- Retry authorization bound to method semantics, operation policy,
  `Replayable`/`OneShot` body state, and delivery ambiguity.
- Non-cloneable rate/retry permits charged before each attempt and keyed by
  source, operation, environment, origin, credential/data partition, and
  reviewed policy revision.
- `QuotaAuthority` contract for caller/deployment time, concurrency, and
  coordinated shared-quota decisions.
- Caller-injected monotonic time for deadlines, intervals, and backoff, plus
  separately trusted UTC/civil time for calendar windows and policy expiry.
- Explicit rollback, forward-jump, unavailable-time, and restart behavior,
  direct-mode advisory-per-client semantics, and fail-closed coordinated
  behavior when required time or quota authority is unavailable.
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

- Policy-versioned non-secret keys, raw/derived distinction,
  `Fresh`/`StaleWithin`/`CacheOnly` modes, and purge dimensions.
- Caller-supplied collision-resistant key function where hashing is required,
  with canonical identity comparison before accepting a collision-sensitive
  hit.
- Explicit non-secret data/credential partition input reserved from day one.
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
- One common typed `Operation::plan` path; optional `.send()` and `.collect()`
  methods remain thin orchestration over explicit transport and budget inputs.
- Source-specific `Page`, time-window, cell-partition, and `ChangeBatch`
  continuations rather than one universal pagination abstraction.
- Public naming and feature review.
- No generic HTTP surface, hidden global client, implicit runtime, implicit
  paging, implicit retry, or implicit network behavior.
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
- Explicit default/`alloc`/`std`/transport/agency capability table and adapter
  error semantics.
- Default facade features that do not silently enable allocation, networking,
  credentials, proxy discovery, telemetry, live tests, or hosted relaying.
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
- Worst-case budget documentation, including maximum bytes, attempts, pages,
  records, allocation, parser/query work units, and work per operation.
- Low-bandwidth and intermittent-connectivity profiles suitable for
  automotive and mobile integration review.
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
- Compile-tested examples labeled by `no_std/no_alloc`, `no_std+alloc`, `std`
  orchestration, or external-adapter capability tier, plus migration policy.
- Current source dossier hashes.
- Claim/implementation cross-check, including crate-introduction versions,
  feature tiers, roadmap phases, and post-1.0 agency scope.

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
- Metadata-driven workspace discovery so validation does not remain hard-coded
  to the initial two crates.
- Manifest enforcement proving generated and handwritten Rust sources share
  the same 500-line maximum.
- Publish rollback and owner checklist.

Verification:

- Inherited gate plus isolated `cargo package` for every crate using local
  registry patches only for already-versioned workspace dependencies.

Exit criteria:

- Every crate archive is independently complete and within registry limits.
- `v0.44.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.45.0 - Executor Authority Integration Audit

Goal: prove one crate owns generic execution and no composition path duplicates
or bypasses its authority transitions.

Deliverables:

- Dependency-DAG and ownership audit for core, policy, HTTP, executor, codecs,
  Trafikverket, and facade.
- Blocking/async parity for capability validation, quota-permit consumption,
  late credentials, sink/decoder driving, and completion.
- Compile-fail boundaries proving the facade contains wiring only, agency
  crates do not depend on HTTP/executor, and callers cannot construct
  authorized states.
- No generic URL, header, method, transport, or raw-execution bypass.

Verification:

- Inherited gate plus forbidden-edge, forged-state, double-execution,
  cancellation, and blocking/async equivalence tests.

Exit criteria:

- `sweden-executor` is the sole generic execution owner and every public
  convenience path delegates to it.
- `v0.45.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.46.0 - Credential Lifecycle Audit

Goal: close every Sweden-owned credential path before security stabilization.

Deliverables:

- Source/environment/operation/scope-bound provider and private one-use wire
  injection review.
- Canonical pre-authentication cache/fingerprint identity and ephemeral
  secret-bearing wire representation.
- Closed diagnostics, fixture/replay rejection, memory-lifetime guidance, and
  explicit residual exposure statement for arbitrary transports, panic/crash
  infrastructure, allocators, and the host process.
- No hosted gateway-key or tenant credential surface.

Verification:

- Inherited gate plus distinctive markers through planning, execution, debug,
  errors, cache, metrics, fixtures, replay, cancellation, and adapter failures.

Exit criteria:

- No credential-leaking path exists in Sweden-owned code, and documentation
  does not extend that guarantee to arbitrary transports or deployment state.
- `v0.46.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.47.0 - Body Replay Redirect And Retry State Machine

Goal: prevent unsafe retransmission or credential forwarding after delivery
becomes ambiguous.

Deliverables:

- `Replayable` and `OneShot` request-body typestates with private consumption.
- Retry permission jointly bound to method semantics, operation policy,
  attempt result, and body replayability.
- Redirect resolution with fresh origin/policy/quota checks and no automatic
  credential forwarding.
- Explicit ambiguous-delivery, authentication-challenge, cancellation, and
  partial-write states.

Verification:

- Inherited gate plus consumed/one-shot resend compile failures, partial write,
  ambiguous result, auth challenge, redirect loop/cross-origin, and
  credential-forwarding tests.

Exit criteria:

- A consumed or ambiguously delivered body cannot be resent unless the
  reviewed operation explicitly proves replayability and issues fresh
  authority.
- `v0.47.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.48.0 - Quota Cache And Kill-Switch Integration

Goal: integrate direct-SDK quota and cache behavior without claiming a hosted
service.

Deliverables:

- Local advisory limiter and coordinated `QuotaAuthority` topology contracts.
- Monotonic/civil clock behavior, source kill switch, policy-expiry transition,
  and cache directive enforcement.
- Fail-closed coordination/store boundary for deployments that opt into shared
  quota state.
- Deterministic outage, clock, restart, stampede, and quota-amplification
  simulation with no tenant/gateway product surface.

Verification:

- Inherited gate plus limiter/cache outage, stale policy, kill switch,
  rollback/jump, retry amplification, and local-versus-coordinated tests.

Exit criteria:

- Direct SDK guarantees are explicit, and optional coordination failures
  cannot exceed a reviewed source policy.
- `v0.48.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.49.0 - Payload-Free SDK Diagnostics

Goal: make direct SDK execution diagnosable without collecting payloads,
credentials, or unbounded identifiers.

Deliverables:

- Allowlisted diagnostic event fields, bounded opaque IDs, metric labels, and
  redacted support record schema.
- Explicit secure-debug policy kept off by default.
- Personal-data, source-payload, URL/header, credential, and high-cardinality
  rejection.
- No hosted audit-log, tenant, gateway, or telemetry-export implementation.

Verification:

- Inherited gate plus marker secrets/payloads across every Sweden-owned
  diagnostic sink, cardinality bounds, and arbitrary-adapter error tests.

Exit criteria:

- Default diagnostics contain bounded identifiers and counts only; hosted
  observability remains outside 1.0.
- `v0.49.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.50.0 - Security Architecture Review

Goal: perform a complete pre-beta adversarial review of implemented boundaries.

Deliverables:

- Updated threat model, attack-surface inventory, abuse cases, and control map.
- SSRF, parser, secret, executor authority, body replay, policy drift, rate,
  cache, supply-chain, and release reviews.
- Explicit trust-boundary review separating Sweden-controlled validation from
  arbitrary transport, DNS, TLS, proxy, clock, credential-store, and
  deployment behavior.
- Independent review report and tracked remediation.
- Unsafe/dependency attestation.

Verification:

- Inherited gate plus extended mutation runs and every security regression suite.

Exit criteria:

- No critical/high finding remains and medium findings have explicit treatment.
- `v0.50.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.51.0 - CSV Admission Decision And Bounded Codec

Goal: decide from the frozen Trafikverket dossier whether CSV belongs in 1.0,
and admit it only through its own focused crate.

Deliverables:

- Admission record naming each reviewed 1.0 operation that requires CSV, or an
  explicit post-1.0 deferral when no such operation exists.
- If admitted, create and publish `sweden-codec-csv`; CSV may not be placed in
  core, JSON, XML, executor, or an agency crate.
- If admitted, streaming first-party parsing with operation-fixed delimiter,
  quote, line-ending, header, blank-record, BOM, encoding, field, row, record,
  byte, and allocation rules.
- If admitted, deterministic raw writer and separate spreadsheet-safe export
  neutralizing formula-leading `=`, `+`, `-`, and `@`, including after admitted
  leading whitespace/control prefixes.

Verification:

- Inherited gate plus dossier-to-admission consistency.
- When admitted: malformed quotes, mixed line endings, oversized fields,
  excess records, dialect confusion, formula injection, mutation corpus,
  isolated `no_std`/`alloc`, and package checks.
- When deferred: prove no 1.0 crate, feature, fixture, fuzz gate, or production
  claim requires CSV.

Exit criteria:

- CSV is either a fully separate bounded crate required by a named operation or
  is explicitly absent from the 1.0 product and final mandatory fuzz scope.
- `v0.51.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.52.0 - Consumable Resource Ledger Audit

Goal: prove that configured ceilings become one-way consumed state at every
untrusted boundary.

Deliverables:

- Control map covering transport, wire/decoded body, parsing, allocation,
  retries, redirects, pages, records/cells, collection, encoding, CPU work
  units, and checkpoints.
- Checked pre-charge semantics, stable exhaustion errors, and tighten-only
  public ceiling changes.
- Non-`Copy`, non-`Clone` permits where copying would duplicate authority or
  budget.
- Parent/child and cross-layer accounting tests proving that conversion
  between ledgers neither refunds nor double-spends capacity; ambiguous
  network results never refund an attempt permit.

Verification:

- Inherited gate plus exact-limit, one-over, overflow, cancellation,
  partial-progress, replay, and copied-state compile-fail cases.

Exit criteria:

- Every resource-consuming stable path names its ledger and charges before the
  corresponding work or authority use.
- `v0.52.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.53.0 - Transport Trust And Conformance Audit

Goal: state and test the exact guarantees at the caller-owned transport
boundary.

Deliverables:

- Public trust statement: arbitrary transports are trusted components and are
  not sandboxed by trait design.
- Reviewed-adapter conformance suite for closed origin, redirect-as-data,
  disabled automatic proxy behavior, bounded decompression, cancellation,
  timeout, redaction, and error translation.
- Deployment checklist for DNS, TLS, certificates, proxy, egress, logging,
  clock, and credential-store controls.
- Separate guarantee tables for Sweden-controlled executors, conforming
  adapters, and arbitrary caller implementations.
- Hostile-plugin isolation guidance using a separate process, restricted
  credential broker, bounded IPC, and deployment-owned egress controls.

Verification:

- Inherited gate plus intentionally malicious/non-conforming transport tests
  demonstrating the boundary of the guarantee without weakening planner
  validation.

Exit criteria:

- Documentation and types make no cryptographic isolation claim that a caller
  transport or deployment can invalidate.
- `v0.53.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.54.0 - Evidence-Bound Capability Stabilization Audit

Goal: stabilize and audit the capability/permit guarantee implemented at
`v0.9.0`; do not introduce a second authority mechanism.

Deliverables:

- Audit the existing generated private capability binding to source,
  operation, plan, environment, origin, reviewer trust root, policy/dossier
  identity, schema version, monotonic policy version, and review expiry.
- Audit the existing non-cloneable execution permit through executor, retry,
  redirect, and next-page consumption.
- Fail-closed invalidation when evidence changes/expires, versions roll back,
  trust roots change, or a kill switch activates.
- `IntegrationStatus` retained only as descriptive metadata.

Verification:

- Inherited gate plus stale digest, wrong operation, wrong environment, expiry,
  schema drift, forged status, and permit-reuse tests.

Exit criteria:

- The single `v0.9.0` authority path is stable, fully integrated, and cannot be
  minted through a general public constructor.
- `v0.54.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.55.0 - Capability-Tier Isolation

Goal: prove every feature tier adds only its declared capability.

Deliverables:

- Audited default, `alloc`, `std`, transport, agency, test, live-test, and
  hosted feature matrix.
- Compile guards against accidental sockets, filesystem, environment, runtime,
  proxy, telemetry, or credential behavior in lower tiers.
- Additive feature and facade propagation rules with no hidden default
  activation.
- Independent minimal examples for each supported tier.
- MSRV checks for each supported feature combination, not only aggregate
  all-feature builds.

Verification:

- Inherited gate plus powerset-oriented feature checks, isolated packages,
  `no_std` targets, and default-feature capability assertions.

Exit criteria:

- Enabling a lower capability tier cannot silently activate a higher one.
- `v0.55.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.56.0 - Sans-I/O And Low-Bandwidth Qualification

Goal: make constrained, mobile, and intermittent integrations predictable.

Deliverables:

- Caller-owned scratch and buffer sizing guidance for borrowed parsers and
  bounded sinks.
- Stable `NeedRequestCapacity` and `NeedScratch`-style errors with computable
  minimum sizes before partial semantic commitment where feasible.
- Per-operation maximum bytes, attempts, redirects, pages, records,
  allocations, and elapsed-work profiles.
- Advisory pre-I/O cost reports separated clearly from authoritative ledgers.
- Pause, resume, cancellation, checkpoint, and partial-delivery semantics with
  no implicit retry or paging.
- Automotive/mobile integration note that remains portable to future Aesynx
  support without importing an OS assumption.

Verification:

- Inherited gate plus tiny-buffer, short-write, interruption, resume,
  cancellation, and low-bandwidth deterministic simulations.

Exit criteria:

- A caller can determine worst-case local and network work before execution.
- `v0.56.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.57.0 - Orchestration Ergonomics

Goal: improve common use without creating a second unsafe execution path.

Deliverables:

- One typed `Operation::plan` primitive shared by blocking, async, mock, and
  custom transports.
- Audit the optional generic `Client<T, C, Q, K>` owned by
  `sweden-executor` over caller-supplied transport, clock, quota authority, and
  credential provider, with no ambient discovery or facade implementation.
- Optional `.send()` and `.collect()` helpers that only orchestrate explicit
  transport, credential sink, clock, and budget inputs.
- Typed response access to provenance, freshness, transformation, and source
  error state.
- Compile-tested direct, streaming, constrained, and custom-transport examples.

Verification:

- Inherited gate plus API equivalence, hidden-I/O, hidden-retry,
  hidden-allocation, and unbudgeted-collection reviews.

Exit criteria:

- The convenient path is the same reviewed path and exposes every authority or
  resource input that affects security.
- `v0.57.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.58.0 - Adapter And Binding Admission Boundary

Goal: resolve ecosystem integration expectations without weakening the
dependency, unsafe-code, or trust policies.

Deliverables:

- Audited caller-bridge examples for maintained external HTTP/TLS stacks.
- Decision record confirming that concrete ecosystem adapters and mobile FFI
  bindings remain outside 1.0 under the current zero-third-party and
  safe-Rust rules.
- Explicit future admission criteria for any dependency, unsafe block, FFI
  surface, platform credential store, or background runtime.
- Documentation that avoids zero-boilerplate or official-adapter claims the
  repository does not implement.

Verification:

- Inherited gate plus example compilation, dependency-tree attestation,
  unsafe-code scan, and trust-claim review.

Exit criteria:

- Users have an honest integration path and no unreviewed adapter or binding is
  smuggled into the 1.0 scope.
- `v0.58.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.59.0 - Pre-Legal Completeness Gate

Goal: close technical and evidence gaps before legal/privacy readiness begins.

Deliverables:

- Operation-by-operation matrix linking code, policy, dossier, schema,
  fixtures, resource ledgers, feature tier, documentation, and review expiry.
- Explicit unsupported and deferred inventory, including archive formats,
  dependencies, FFI, concrete network adapters, and post-1.0 agencies.
- Current contradiction tests and plan/metadata consistency report.
- Remediation list with no silent carry-over into legal review.

Verification:

- Inherited gate plus generated-matrix reproducibility and deliberate
  missing/stale/contradictory-entry failures.

Exit criteria:

- Every admitted operation has complete technical evidence and every missing
  capability is explicitly deferred.
- `v0.59.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.60.0 - Legal And Privacy Readiness

Goal: admit only operations with current lawful direct-SDK decisions while
keeping hosted service operation explicitly deferred.

Deliverables:

- Operation-level source dossiers, attribution, retention, redistribution,
  transformation, direct-use decisions, and explicit hosted-use deferral.
- Privacy/minimization guidance, deletion and retention behavior for
  caller-owned SDK data, and acceptable-use policy.
- No gateway tenant, hosted credential, subprocessor, or service-production
  claim.
- Fail-closed unknown data class.

Verification:

- Inherited gate plus policy expiry, deletion, retention, attribution,
  direct-only, forbidden-hosted, and unknown-class tests.

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

- Recovery does not exceed upstream policy or lose credential/source
  isolation.
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

## v0.91.0 - Cross-Topology Quota Simulation

Goal: validate rate, retry, cache, and direct/coordinated SDK policy across
realistic execution topologies.

Deliverables:

- Deterministic single-process, multi-process, multi-client, outage, and clock
  anomaly simulations.
- Evidence distinguishing local advisory limiting from coordinated shared
  quota enforcement.
- Required `QuotaAuthority` behavior and fail-closed unavailability for every
  topology whose dossier requires shared coordination.
- Retry amplification, cache stampede, cancellation, and kill-switch
  regression scenarios.
- Updated deployment and capacity assumptions.

Verification:

- Inherited gate plus repeated seeded simulations proving source and
  deployment ceilings are never exceeded by reviewed coordinated execution.

Exit criteria:

- Every supported topology states where coordination lives and fails closed
  when required coordination is unavailable.
- `v0.91.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.92.0 - Final JSON Fuzz Campaign

Goal: independently harden the frozen JSON lexical, structural, and typed
decode paths.

Deliverables:

- Final seed corpora for tokenization, Unicode, numbers, nesting, duplicates,
  owned values, and source envelopes.
- Extended mutation and out-of-process fuzz campaign with minimized
  regressions committed.
- Resource-exhaustion, parser-agreement, and panic review.
- Exact tool versions, commands, duration, and corpus hashes recorded.

Verification:

- Inherited gate plus full corpus replay on MSRV and pinned stable with no
  unresolved crash, panic, hang, or budget bypass.

Exit criteria:

- The frozen JSON surface has current reproducible adversarial evidence.
- `v0.92.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.93.0 - Final XML And CSV Fuzz Campaign

Goal: independently harden the frozen XML and admitted CSV paths.

Deliverables:

- XML corpora for declarations, DTD/entity rejection, namespaces, expanded
  attributes, character references, matching, and budgets.
- When CSV was admitted at `v0.51.0`, corpora for each admitted dialect,
  quoting, line endings, encoding, formula-leading output, records, and
  budgets; otherwise recorded proof that no CSV target or claim exists.
- Extended mutation and out-of-process fuzz campaigns with minimized
  regressions committed.
- Exact tool versions, commands, duration, and corpus hashes recorded.

Verification:

- Inherited gate plus full corpus replay on MSRV and pinned stable with no
  unresolved crash, panic, hang, expansion, ambiguity, or budget bypass.

Exit criteria:

- The frozen XML surface and, only when admitted at `v0.51.0`, the CSV surface
  have current reproducible adversarial evidence.
- `v0.93.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.94.0 - Cross-Platform Resource Qualification

Goal: confirm frozen limits and behavior on every day-one platform class.

Deliverables:

- Linux, Windows, macOS, FreeBSD, Android, and iOS compile/test evidence
  appropriate to each crate.
- MSRV and pinned-stable stack, allocation, binary-size, and throughput
  measurements for representative bounded paths.
- Low-memory, low-bandwidth, cancellation, endianness, pointer-width, and
  line-ending regression evidence.
- Updated platform capability and unsupported tables.

Verification:

- Inherited gate plus reproducible qualification runs and investigation of
  every threshold regression.

Exit criteria:

- No documented platform depends on an unstated resource or OS assumption.
- `v0.94.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.95.0 - Release Candidate Remediation

Goal: resolve findings from an independent assessment of the exact frozen
candidate without expanding the product.

Deliverables:

- Assessment scope and baseline commit, findings, fixes, regression tests,
  refreshed docs/evidence, and compatibility report.
- Independent assessment artifact naming and authenticating the exact frozen
  candidate commit; this supplements rather than changes the routine
  maintainer pentest/GitHub/tag flow.
- Explicit deferral of non-blocking new ideas.

Verification:

- Inherited gate plus every assessment finding reproducer, retest evidence, and
  full release-candidate suite.

Exit criteria:

- Independent findings are resolved or explicitly risk-treated without
  unreviewed feature growth.
- `v0.95.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.96.0 - Supply-Chain And Release Provenance Audit

Goal: prove the frozen source, tools, packages, and release process are
reproducible and reviewable.

Deliverables:

- CodeQL/default-analysis, dependency/unsafe attestation, tool pin, generated
  input, source archive, and package-content evidence.
- SBOM and provenance artifacts appropriate to the dependency-free workspace.
- Reproducible package hashes and dependency-order publication rehearsal.
- Release-script audit proving unchanged subcrates are not selected.
- External GitHub/CodeQL result attestation tied to the reviewed candidate,
  without claiming local scripts can prove remote state.

Verification:

- Inherited gate plus clean-machine generation, isolated packaging, artifact
  comparison, and release dry run without publishing.

Exit criteria:

- Every intended release artifact is traceable to reviewed source and selected
  crate metadata.
- `v0.96.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.97.0 - Final Source Legal And Privacy Refresh

Goal: ensure no frozen operation relies on stale source, terms, licence,
privacy, or hosted-use evidence.

Deliverables:

- Fresh retrievals, hashes, reviewers, expiries, schema references, terms,
  attribution, retention, redistribution, and data-class decisions.
- Explicit confirmation or removal of each direct-use capability and continued
  hosted-service deferral.
- Updated privacy, deletion, acceptable-use, support, and incident contacts.
- Fail-closed test for every changed or expired decision.

Verification:

- Inherited gate plus operation-by-operation evidence review and deliberate
  stale/missing/contradictory evidence failures.

Exit criteria:

- Every admitted operation has current technical and legal authority evidence.
- `v0.97.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

## v0.98.0 - Public Surface And Scope Freeze

Goal: lock the exact public API, feature graph, stable operations, and claims
that may enter final acceptance.

Deliverables:

- Rustdoc public-API snapshot, feature matrix, stable operation/object matrix,
  unsupported inventory, and SemVer report.
- Documentation/examples checked against the same generated metadata.
- Confirmation that only Trafikverket is production agency scope for 1.0.
- Change-control rule allowing only release-blocking fixes through 1.0.

Verification:

- Inherited gate plus API diff, feature powerset, documentation claim, package,
  and stable-operation consistency checks.

Exit criteria:

- Any subsequent surface or scope change requires explicit re-review and a new
  frozen baseline.
- `v0.98.0 implementation stop reached. Run the maintainer pentest and update the repository report.`

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

- Stable `sweden-core`, `sweden-policy`, `sweden-http`,
  `sweden-codec-json`, `sweden-codec-xml`, `sweden-executor`,
  `sweden-trafikverket`, and `sweden` facade, plus every other shared/tool
  crate admitted by the frozen scope.
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

### Post-1.0 Hosted Service Admission Track

Hosted service work is not automatically authorized by reaching 1.0. If the
maintainer later admits it, focused service crates begin their own independent
`0.x` lines and each implementation stage is assigned an explicit future
repository minor with the normal pentest/GitHub/tag gate. No placeholder crate
is created beforehand.

| Service-crate version | Post-1.0 stage |
| --- | --- |
| `0.1.0` | Service operation registry and worker boundary using stable SDK metadata |
| `0.2.0` | Explicit tenant/project/agreement context and cross-tenant denial |
| `0.3.0` | Gateway key verification, scope, rotation, revocation, and audit events |
| `0.4.0` | Coordinated hosted quota/cache storage, partitioning, purge, and kill switch |
| `0.5.0` | Payload-free observability plus production legal, privacy, reliability, security, and pentest admission |

A test-only verifier or storage mock cannot support a hosted production claim.
Concrete service crate boundaries, dependencies, and repository versions are
frozen in a separate architecture decision before `0.1.0` implementation
begins.
