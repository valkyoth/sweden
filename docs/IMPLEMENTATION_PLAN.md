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
- focused first-party policy, registry, and admitted codec crates;
- a transport-neutral `sweden-http` boundary;
- a generic `sweden-executor` that owns reviewed orchestration;
- a production-ready `sweden-trafikverket` integration;
- a small feature-gated `sweden` facade;
- a published `sweden-conformance` source package that proves repeatable
  agency onboarding without making synthetic semantics part of the executor
  or pulling post-1.0 agencies forward;
- documentation and policy evidence suitable for serious production review.

The `0.1.0` workspace deliberately contains only `sweden-core` and `sweden`.
Future crates are created and published when their first implementation
milestone begins; the repository does not carry empty placeholder packages.

The 1.0 product is not:

- a generic government URL proxy;
- a custom TLS, OAuth, BankID, X.509, or cryptography implementation;
- a promise that every visible dataset may be cached, transformed, or relayed;
- a replacement for legal review;
- a hosted gateway or multi-tenant service;
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
  produce opaque privately constructed authorized executions rather than
  relying on an agency-wide access label or a sealed cross-crate trait;
- accepted: `sweden-policy` contains only source-independent decision
  algorithms and contracts; the separate `sweden-registry` owns generated
  source/operation membership and the indivisible binding between a canonical
  request, exact encoder/decoder/validator profile, output/provenance type,
  limits, evidence, and finalization rules;
- accepted: official network execution remains disabled through `v0.36.0`;
  mock, synthetic, and redistributable offline evidence exercise earlier
  milestones, and opt-in live execution begins only when `v0.37.0` supplies
  the complete reviewed quota/concurrency and time semantics;
- accepted: registry authorization is not a timeless execution grant; every
  attempt revalidates its registry-bound freshness mode, policy version,
  revocation, kill switch, origin, and environment immediately before
  credentials and I/O;
- accepted: authorization, time-of-use policy validation, quota reservation,
  credential injection, and an in-flight attempt are distinct non-cloneable
  states; `AuthorizedExecution<R>` binds the quota requirement but never owns
  a pre-acquired lease;
- accepted: borrowed decoding uses a synchronous callback visitor whose event
  borrow cannot escape the callback; async callers retain events only through
  an explicit bounded owned `alloc` path;
- accepted: `sweden-core` owns only structural completion vocabulary;
  authority-bearing completion witnesses are opaque and privately constructed
  by their exact producer crate, then consumed by `sweden-executor`;
- accepted: witness type ownership alone is insufficient; `sweden-core`
  supplies only an authority-free generativity primitive and
  `sweden-executor` establishes a non-serializable
  `AttemptBrand<'attempt, R>` scope per execution. Every wire, codec, semantic,
  and finalization witness is invariantly bound to that exact attempt as well
  as its registry entry/version, operation, environment, origin, response
  profile, codec, and validator;
- accepted: canonical request plans use a closed typed header model covering
  reviewed static representation headers, protected late credential slots,
  typed cache-validator slots, explicitly dossier-permitted bounded caller
  metadata, and transport-owned framing that callers cannot set;
- accepted: event sinks return a closed continue/pause/stop/abort decision and
  are trusted caller code that may block, panic, copy data, or consume
  arbitrary CPU; Sweden cannot portably catch panics in `no_std`;
- accepted: pause occurs only after a complete event callback; a paused
  decoder owns only its already-charged bounded carry while unconsumed
  transport input remains caller-owned, and no next chunk is accepted until
  resume or termination;
- accepted: current-authority observations are non-serializable and bound to
  the originating monotonic clock/session epoch; restart, reset, wrap, or
  epoch mismatch requires a fresh observation or fails closed;
- accepted: immediate pre-I/O policy revalidation narrows but cannot eliminate
  revocation between the last authority check and a caller-owned transport
  call; stronger atomic revocation requires a future authority-issued
  per-attempt grant or controlled broker and is not claimed for the 1.0 SDK;
- accepted: fixture recording is synthetic-only by default and official
  response retention is a dossier-governed, fail-closed capability rather than
  a general testkit convenience;
- accepted: `CorpusReplay` removes conformance authority but never removes
  retention obligations; historical official bytes are purged or denied when
  their retention permission expires, while synthetic bytes remain eligible;
- accepted: configured limits become checked ledgers charged before I/O,
  allocation, parsing, retry, redirect, page fetch, or checkpoint advance;
- accepted: local ledgers and coordinated quota authority are different
  controls; multi-process or otherwise coordinated execution fails closed when
  its reviewed operation requires shared authority and none is available;
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
- retained: hosted gateway/service crates are not part of the 1.0 product.
  They may begin a separately admitted post-1.0 track only after the SDK
  boundary is stable;
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

`sweden-core`, the root facade, policy, registry, HTTP, executor, codecs,
conformance, and agency crates must use `#![no_std]`. Allocation becomes an
explicit feature only when a bounded owned representation is required.

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
- Agency and conformance crates depend only on required shared
  core/policy/codec crates, not on one another, the registry, executor, HTTP,
  or facade.
- The facade contains wiring and re-exports, not implementations.
- Network adapters do not own source semantics.
- Policy, HTTP, and codecs remain source-independent; source-specific generated
  registry entries live only in `sweden-registry`.
- Every Rust source file, generated or handwritten, must never exceed 500
  lines.
- Review splitting once a Rust file approaches 300 lines.
- Generated files require an explicit manifest and are split by upstream
  object family before reaching the same 500-line ceiling.

### 2.6 Bounded behavior

Every operation declares limits for wire bytes, decoded bytes, nesting,
strings, headers, chunks, allocation count/bytes, work units, collection
elements, pages, records/cells, redirects, attempts, time, and retries. There
is no unbudgeted `collect_all`, response buffering, decompression, archive
extraction, or recursive parse.

Configured ceilings and consumed state are separate types. Charges use checked
arithmetic and occur before accepting bytes, allocating, transmitting an
attempt, following a redirect, fetching a page, or committing a checkpoint.
Callers may tighten reviewed ceilings but cannot raise them through a stable
public API.

Five mechanisms remain distinct:

- `Limits`: immutable operation maxima that callers may only tighten;
- `Ledger`: non-`Copy`, non-`Clone` local capacity charged before work;
- `QuotaAuthority`: caller/deployment coordination for time, concurrency, and
  shared upstream quotas;
- `PolicyAuthority`: optional caller/deployment source of current revocation
  and monotonic policy-version state;
- `AuthorizedExecution<R>`: a one-use registry-created package binding the
  exact canonical plan, registered encoder/decoder/validator profile,
  output/provenance type, finalization rules, policy, ledger, and quota
  requirement—but not an acquired quota lease. The executor never accepts an
  authorization token alongside a separately caller-selected decoder or
  semantic validator.

A per-process limiter is described as advisory unless the dossier explicitly
establishes that scope. Hosted or multi-process modes requiring coordinated
quota enforcement fail closed when the authority or trustworthy time source is
unavailable.

### 2.7 Closed request-header model

`CanonicalPlan<Unauthenticated>` admits headers only through typed categories:

- `ReviewedStaticHeader` for dossier-fixed representation choices such as
  `Accept`, `Content-Type`, API version, and `Accept-Encoding: identity`;
- `CredentialHeaderSlot` for protected late injection after origin and policy
  revalidation;
- `CacheValidatorSlot` for typed bounded `If-None-Match` and
  `If-Modified-Since` values;
- `CallerMetadataSlot` only when the exact operation dossier names the field,
  grammar, confidentiality class, and byte/count ceiling; and
- `TransportFraming`, including `Content-Length`, owned solely by the reviewed
  transport/body encoder and unavailable to caller plan construction.

Names use one canonical ASCII case-insensitive identity. Each category defines
singleton or reviewed duplicate behavior; unknown duplicates, CR/LF and other
controls, invalid field-name/value bytes, hop-by-hop headers, and aggregate or
per-field budget overflow fail closed. Representation-affecting static and
caller metadata participate in canonical request/cache identity or an
explicit reviewed `Vary` dimension. Credentials, transport framing, and
operational diagnostics never become canonical or cache-key material.

No raw header map or generic `(name, value)` escape hatch exists in an
authorized plan. Source dossiers and generated registry entries enumerate
every admitted header slot, and the executor revalidates the closed set before
credential injection and transport handoff.

### 2.8 Honest capability claims

Crates report `Foundation`, `Experimental`, or `Stable`. A source cannot become
stable until its exact operation set, source terms, schema revision, fixtures,
tests, security review, and pentest evidence are current.

`IntegrationStatus` is descriptive only. Executable stable behavior requires a
generated registry entry and an opaque privately constructed
`AuthorizedExecution<R>` bound to the complete reviewed request/response
profile, operation policy, dossier identity, schema version, review expiry,
environment, and current evidence.

## 3. Workspace Architecture

Planned 1.0 dependency direction (arrows point from dependency to consumer):

```text
sweden-core ─┬─> sweden-policy
             ├─> sweden-http
             ├─> sweden-codec-json
             └─> sweden-codec-xml

sweden-core + sweden-policy + sweden-codec-json + sweden-codec-xml
    ├─> sweden-conformance
    └─> sweden-trafikverket

sweden-core + sweden-policy
    └─> sweden-registry

selected agency/conformance crate
    └─> sweden-registry matching optional feature

sweden-core + sweden-policy + sweden-http + sweden-registry
    └─> sweden-executor

sweden-core + sweden-registry + sweden-executor + sweden-trafikverket
    └─> sweden
```

The facade aligns selected registry and agency features. Agency and conformance
crates use shared core/policy/codec contracts but do not depend on
`sweden-registry`, `sweden-http`, or `sweden-executor`; they emit
transport-neutral operation plans and wire profiles. `sweden-registry` may
depend one-way on an agency/conformance crate behind the matching feature to
bind those concrete profiles. This preserves an acyclic graph. SMHI, SCB,
JobTech, and Skatteverket are post-1.0 additions and therefore do not appear in
the 1.0 graph.

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
| `sweden-policy` | `no_std` | Source-independent policy evaluation and authority contracts |
| `sweden-registry` | `no_std` | Closed generated source/operation membership and authorized execution binding |
| `sweden-http` | `no_std` | Sans-I/O blocking/async transport contracts and bounded sinks |
| `sweden-executor` | `no_std`/`alloc` | Generic reviewed execution and optional client orchestration |
| `sweden-conformance` | `no_std`/`alloc` | Synthetic source operations and wire profiles for published conformance tests |
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
| `0.7.0` | `sweden-policy` |
| `0.9.0` | `sweden-registry` |
| `0.10.0` | `sweden-http` |
| `0.13.0` | `sweden-codec-json` |
| `0.16.0` | `sweden-codec-xml` |
| `0.19.0` | `sweden-testkit` |
| `0.20.0` | `sweden-schema` |
| `0.21.0` | `sweden-executor`, `sweden-conformance` |
| `0.22.0` | `sweden-trafikverket` |
| `0.51.0` | `sweden-codec-csv` only if a reviewed 1.0 operation requires CSV |
| Post-`1.0.0` | Remaining named agency crates on their own tracks |

The facade crate is the repository release clock: `sweden` always equals the
`vX.Y.Z` tag and publishes for that release. Subcrates have independent
versions and publish only for their own code, bugfix, dependency, or immutable
metadata changes. At `v1.0.0`, every crate then in the workspace converges to
`1.0.0`. The current state is recorded in
[CRATE_VERSION_MATRIX.md](CRATE_VERSION_MATRIX.md) and mechanically checked by
`scripts/release_crates.py`.

### 3.1 Boundary ownership

| Crate | Owns | Must not own |
| --- | --- | --- |
| `sweden-core` | IDs, limits/ledgers, safe errors, canonical plan and closed header categories, borrowed-event contracts, authority-free invariant attempt generativity, structural completion traits/status, provenance vocabulary | concrete completion witnesses, policy decisions, I/O, credentials, agency models |
| `sweden-policy` | source-independent dossier evaluation, revocation/expiry logic, cache/quota requirement contracts | source registry data, transport calls, credentials, source decoding |
| `sweden-registry` | generated closed membership, exact profile/header compatibility, opaque `AuthorizedExecution<R>`, epoch-bound authority observation, invocation of the bound validator, private branded semantic witness | generic policy algorithms, transport calls, credentials, wire implementations |
| `sweden-http` | blocking/async transport, response sink, redirect-as-data, safe transport codes, private branded `WireComplete` witness | authorization, retries, credential injection, agency semantics |
| `sweden-executor` | time-of-use revalidation, generative attempt scope, quota reservation/commit/release transitions, late credentials, redirect/retry state machines, exact-branded-witness consumption, private `Finalized<R>`/complete provenance, `Client<T, C, Q, P, K>` | concrete HTTP/TLS, ambient discovery, source-specific wire truth, synthetic source semantics |
| Codec crate | bounded syntax, event visitor, private branded codec-specific completion witness such as `JsonComplete` or `XmlComplete` | wire/semantic completion, I/O, policy authority |
| `sweden-conformance` | synthetic operations, encoders, decoders, validators, output types, and fixtures | registry authority, generic execution, production source claims |
| Agency crate | typed operation metadata, inputs, encoding, decoding, semantic validation | authority issuance, sockets, TLS, generic execution, other agencies |
| `sweden` | feature wiring, aliases, and re-exports | implementation logic |

The executor is generic over caller transport/clock/quota/credential resources
and the registry-produced authorized execution package. Agency crates never
depend on the registry or executor. The executor does not depend directly on
an agency crate; registry features provide the one-way binding and the facade
or application aligns them.

### 3.2 Cross-crate authority model

Rust has no friend-crate mechanism, so Sweden does not claim that a core trait
can be sealed while remaining implementable by an agency crate. The concrete
trust design is:

1. `sweden-core` exposes validating descriptive IDs, canonical plan types, and
   public structural operation/decoder contracts. Downstream crates may
   implement those contracts; doing so grants no execution authority.
2. `sweden-policy` evaluates source-independent policy inputs. It owns no
   generated source list and cannot turn a descriptive ID into authority.
3. `sweden-registry` owns the generated closed reviewed-operation entries
   compiled from canonical manifests. Public keys may select an existing entry
   but no downstream crate can add one.
4. After policy evaluation, the registry validates the complete canonical plan
   and constructs a public opaque `AuthorizedExecution<R>`. Its fields and
   constructors remain private to `sweden-registry`; it is non-`Copy`,
   non-`Clone`, and indivisibly binds one encoder profile and canonical plan,
   expected status/media profile, exact registered decoder and semantic
   validator, output/provenance type, limits, evidence, environment, origin,
   authority decision, and finalization behavior.
5. `sweden-executor` accepts only `AuthorizedExecution<R>` and drives the
   behavior it embeds. It has no public path that accepts a permit plus an
   arbitrary decoder, validator, media profile, or output type. Custom
   operation implementations, descriptive IDs, dossier-shaped values, and
   structurally valid plans cannot mint or substitute authority.

Registry and policy compatibility is explicit. Workspace dependencies and
facade features select one compatible `sweden-policy`/`sweden-registry` pair;
Cargo type identity prevents packages from crossing crate versions, and
runtime registry/policy version or digest mismatches fail closed. Adding a
reviewed entry or feature requires a registry release. Removing an entry,
changing security policy, or revoking evidence advances the monotonic
registry/policy version and invalidates older authorization packages inside an
updated deployment or when a trusted monotonic `PolicyAuthority` reports that
state. An offline old binary cannot learn a new revocation; it remains bounded
only by its compiled expiry and any authority it was configured to require. No
compatibility shim may translate an authorization package between registry
versions.

A hostile downstream test package attempts each forbidden construction and
custom trait implementation. Compile-fail tests prove non-construction and
runtime tests prove that structurally valid unregistered plans are denied.

### 3.3 Completion witness ownership

Completion uses the same explicit cross-crate ownership discipline:

1. `sweden-core` defines only descriptive completion traits/status vocabulary;
   it exposes no constructor for an authority-bearing completion token.
2. `sweden-core` exposes an authority-free higher-ranked generativity
   primitive so dependency-neutral producer crates can share an invariant
   `AttemptBrand<'attempt, R>` type. Creating a structural brand alone grants
   no witness, authorization, or execution power.
3. `sweden-executor` establishes one such non-serializable brand inside an
   executor-owned higher-ranked closure scope for each authorized execution.
   The lifetime cannot escape or be recreated in safe Rust, and every producer
   receives only the branded capability needed for that attempt.
4. `sweden-http` privately constructs opaque `WireComplete<'attempt, R>` after
   the exact bounded response, trailer, and transport-finalization checks,
   carrying that attempt brand.
5. Each codec privately constructs its own non-interchangeable branded witness
   (`JsonComplete<'attempt, R>`, `XmlComplete<'attempt, R>`, or an admitted
   future codec witness) after exact syntax consumption. A codec cannot
   construct another codec's witness.
6. `sweden-registry` invokes the exact validator embedded in
   `AuthorizedExecution<R>` and privately creates a semantic witness bound to
   the same attempt brand, registry entry/version, operation, environment,
   origin, response profile, exact codec/validator, output type, and schema.
7. Within the higher-ranked scope, `sweden-executor` can privately create
   `Finalized<R>` and `Complete` provenance only by consuming wire, codec, and
   semantic witnesses carrying the same invariant brand. The public finalized
   result may leave the scope; the brand and partial witnesses cannot.

Public structural trait implementations grant no completion authority.
Hostile packages must fail to forge a witness, substitute a codec witness,
combine valid witnesses from two concurrent attempts of the same operation,
or bypass the registered validator.

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
advisory cost + reviewed maximum inspection
        ↓
opaque registry-created AuthorizedExecution<R>
binding exact encoder, decoder, validator, output,
limits, evidence, quota requirement, and finalization
        ↓
executor-created generative AttemptBrand<'attempt, R>
binding every later state and completion witness
        ↓
PolicyRevalidated<R>
        ↓
QuotaLeaseAcquired<R>
        ↓
late credential injection into a reviewed execution sink
        ↓
CredentialInjected<R>
        ↓
quota admission committed as AttemptCommitted<R>
        ↓
transport invoked as AttemptInFlight<R>
        ↓
trusted caller-owned transport boundary
        ↓
bounded body sink
        ↓
bound source decoder and semantic validation
        ↓
provenance-wrapped result
```

Every live attempt traverses these private, non-`Copy`, non-`Clone` states.
The executor creates them inside a higher-ranked attempt scope; no state or
partial completion witness can be moved to another attempt, even when two
concurrent executions have identical Rust types and operation metadata.
`PolicyRevalidated<R>` checks the bound freshness requirement, expiry,
registry/policy version, revocation, kill switch, origin, and environment
against trustworthy time and any required authority. The check occurs before
quota acquisition, immediately before credential acquisition, and again
before I/O; retry delays, redirects, and page transitions return to policy
revalidation rather than reusing an earlier decision. Callers may tighten a
freshness requirement but cannot downgrade it.

The closed freshness requirement is either
`CompiledUntil { not_after }`, which requires trustworthy current time and
fails after the compiled evidence expiry, or
`CurrentAuthorityRequired { minimum_version, maximum_staleness }`, which also
requires a current authenticated authority observation. For 1.0, observations
are non-serializable `AuthorityObservation<'epoch>` values tied to an opaque
registry-created `FreshnessEpoch`. The caller authority returns descriptive
state, but only `sweden-registry` validates it against the bound registry and
clock and privately constructs the observation. Staleness is measured only
within that epoch. Restart, counter reset/wrap, an epoch mismatch, missing
state, rollback, wrong registry, or excessive staleness requires a fresh
observation or fails closed. Persisted observations are not admitted without a
future authenticated absolute-expiry and authority-sequence design.

This immediate revalidation is the strongest direct-SDK guarantee without a
transactional policy broker. A revocation can still occur after the final
authority observation and before the external transport actually sends. The
SDK documents that residual race and does not call it atomic revocation;
deployments requiring that property need a controlled broker or a future
authority-issued one-attempt grant coupled to transport admission.

`QuotaLeaseAcquired<R>` is an uncommitted attempt reservation plus concurrency
lease acquired as late as possible. Credential-provider failure or a final
pre-I/O policy denial cancels the unused reservation and releases concurrency
at most once; it does not spend a network-attempt budget. Transition to
`AttemptCommitted<R>` records an atomic quota commit inside `QuotaAuthority`
immediately before transport invocation. That commit cannot be atomic with an
external network call: a crash after commit but before invoking the transport
conservatively spends the attempt. `AttemptInFlight<R>` begins only when the
transport is invoked. From commit onward, cancellation, failure, panic, or
ambiguous delivery spends the attempt, while concurrency is recovered by an
unwind/drop guard where the platform actually unwinds, otherwise by a fenced
release or lease expiry.

Every blocking, async, mock, borrowed, owned, and custom-transport path starts
with the same typed operation and canonical plan. Convenience APIs may
orchestrate that path but cannot introduce a parallel “easy” execution
surface. `Client<T, C, Q, P, K>` is introduced and owned by
`sweden-executor` at `v0.21.0`; it may hold caller-supplied transport, clock,
quota authority, policy authority, and credential provider but discovers none
from ambient process state.

Pre-I/O `Cost` estimates expose reviewed maxima and selected query/projection
work, but remain advisory; consumable ledgers are authoritative. Page,
offset, time-window, cell-partition, and change-checkpoint continuations remain
source-specific types. No `Iterator` hides network I/O and no universal
pagination abstraction erases upstream semantics.

The API must make it impossible to select an arbitrary production origin.
Credentials are inserted only after the origin is validated and are excluded
from debug output, cache keys, canonical hashes, errors, and fixtures.
Authorized executions, revalidated states, quota reservations/leases,
credential-injected states, and in-flight attempts are non-`Copy`,
non-`Clone`, operation-, environment-, and origin-bound. Retries, redirects,
and subsequent pages require fresh checks, charges, and authorization.

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
   retry, and pagination rules. Inventory every request header by the closed
   category, duplicate/budget rule, confidentiality, canonical/cache identity
   participation, reviewed `Vary` dimension, or transport-owned exclusion.
4. Pin official schema/specification inputs with retrieval metadata and hashes.
5. Define explicit request, response, collection, retry, redirect, allocation,
   and time budgets.
6. Build synthetic fixtures before admitting redistributable official
   fixtures.
   Recording is synthetic-only by default. An official response may be
   retained only when its operation dossier explicitly permits retention and
   redistribution; personal or sensitive classifications fail closed instead
   of relying on best-effort scrubbing.
   Recorded metadata binds source, operation, schema, policy, retrieval time,
   classification, and the retention/redistribution decision.
   `ConformanceReplay` rejects expired or mismatched evidence and is the only
   replay mode that may support current conformance/provenance claims.
   `CorpusReplay` treats synthetic or lawfully retained historical bytes only
   as hostile parser/fuzz input; it cannot authorize I/O, create current
   provenance, populate caches, or advance checkpoints.
   Corpus admission rechecks the operation's retention permission at use time:
   once permission expires or is withdrawn, official bytes are purged from the
   managed corpus or denied access. Only synthetic or still-lawfully-retained
   bytes may run in corpus mode.
7. Implement generated policy contradiction tests, request goldens, and
   negative parser tests before live execution.
8. Keep official network execution disabled through `v0.36.0`; use mock,
   synthetic, and legally redistributable offline fixtures during onboarding.
   Beginning at `v0.37.0`, add opt-in low-rate live tests only when the source
   permits them and the executor can require the complete reviewed
   rate/window/concurrency authority, explicit credential scope, honest
   deadline mode, and operation-approved retry/redirect behavior.
9. Generate documentation from the same reviewed operation metadata.
10. Stop for maintainer pentest and keep the versioned repository report
    current.

Policy expiry fails closed. An expired source review cannot silently continue
hosted relaying.
Evidence digests are standard cryptographic values computed by pinned offline
tooling and carried as opaque reviewed bytes in `no_std` code; Sweden does not
invent a runtime provenance hash. A digest proves byte identity only, not
official origin, authenticity, currency, review, or lawful use. Stable
capabilities additionally bind reviewer/trust-root evidence, monotonic policy
version, rollback/downgrade detection, expiry, and kill-switch state.

Compiled policy can age out through expiry but cannot discover a new
revocation by itself. Immediate revocation requires a caller-supplied trusted
`PolicyAuthority` or coordinated current-state provider. Without one, Sweden
claims only compiled-policy identity and fail-closed expiry. Preventing an
older binary from running likewise requires an external monotonic authority;
static `no_std` code cannot guarantee deployment freshness.

## 6. Codec Strategy

The no-third-party rule requires focused first-party codecs. They are not
general-purpose replacements for ecosystem parsers.

Borrowed streaming uses a synchronous GAT-based visitor contract:
`EventFamily::Event<'event>` and
`EventSink<F>::on_event<'event>(F::Event<'event>) -> SinkControl`.
`SinkControl` is closed: `Continue`, `Pause`, `StopEarly`, or
`Abort(SafeSinkCode)`. Decoder `push`/`finish` invokes the visitor before
returning, so the borrow cannot escape the callback. `Pause` preserves only a
bounded resumable provisional state at a completed event boundary. Unconsumed
input stays in caller storage; only an explicitly bounded decoder carry may be
retained, and it has already been charged exactly once. While paused the
decoder rejects another transport chunk, and resume cannot recharge or expose
the same bytes/work twice. `StopEarly` and `Abort` terminate without
completion. Arbitrary callback errors are collapsed to the closed safe code
and never retained as `Error::source()`. Async transports may drive the same
visitor while handling a chunk, but there is no
`Stream<Item = Event<'_>>` claim. Callers that must retain events opt into an
explicit bounded owned `alloc` path.

The sink is a caller-owned trust boundary. It can block, panic, copy borrowed
data, or consume arbitrary CPU. Sweden cannot catch unwinds portably in
`no_std` and makes no callback isolation claim. A panic/cancellation after
attempt commit spends the attempt; concurrency cleanup depends on an actual
unwind guard or lease expiry.

Streaming events remain provisional until three opaque producer-owned
witnesses exist: HTTP wire completion, the exact registered codec completion,
and registry-owned source-semantic completion. Each carries the same
executor-generated invariant attempt brand. Only `sweden-executor` can consume
their same-attempt, registry-bound combination to construct `Finalized<R>` and
`Complete` provenance. Provisional data cannot enter a cache or advance a
cursor/checkpoint. Callers that act on provisional events before composite
finalization explicitly own the downstream rollback/compensation risk.

JSON work is split into:

- byte validation and UTF-8 boundary;
- bounded tokenization with exact JSON number grammar;
- raw/decoded string ceilings, escape validation, Unicode scalar and surrogate
  handling;
- iterative structure with token, depth, member, element, work-unit, and
  duplicate-key policy;
- bounded caller scratch for exact decoded key scalar sequences, with
  collision-safe comparison or bounded re-decoding and explicit work-unit
  charging; lexical spellings such as `"a"` and `"\u0061"` are the same key;
- exact consumption after trailing whitespace;
- borrowed events with caller scratch rather than allocation merely to
  unescape;
- bounded owned values behind `alloc`, charged before reserve, with separate
  limits and observations for logical decoded bytes, requested capacity,
  container capacity, and allocation count; allocator rounding, metadata, and
  physical heap use remain outside the crate's hard guarantee;
- source-specific typed decoding;
- mutation and differential fixtures generated out of process where useful.

XML work is split into:

- strict UTF-8 and XML 1.0 character validation; XML 1.1 is rejected;
- bounded iterative tokenization with caller-provided stack;
- explicit byte/work ceilings for names and QNames, namespace URIs, active
  namespace bindings, comments, CDATA, processing instructions, declarations,
  and numeric character-reference digits;
- namespace handling, exact start/end matching, and duplicate expanded
  attribute rejection;
- unconditional early rejection of DTD, entity declarations, and external
  identifiers; only predefined entities and bounded numeric references are
  admitted;
- canonical escaping and deterministic output;
- source-specific streaming decode.

CSV is admitted before 1.0 only if a reviewed Trafikverket operation requires
it. If admitted, it always lives in `sweden-codec-csv`; otherwise the crate and
all mandatory CSV fuzz claims move post-1.0. Each admitted operation fixes its
delimiter, quoting, line-ending, header, blank-record, BOM, and encoding rules;
no dialect is guessed. Spreadsheet-safe export neutralizes formula-leading
`=`, `+`, `-`, and `@`, including after admitted leading whitespace/control
prefixes. Raw machine export remains a distinct API. Archive support remains
unadmitted until it receives its own bounded security milestone. Unsupported
constructs fail closed.

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
11. Small object-family slices with fixture and offline conformance evidence;
    live evidence begins at `v0.37.0`.
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
- an explicit deadline mode: `TransportEnforced` when the adapter owns
  preemption, `RuntimeRace` when the caller races execution with a timer or
  cancellation future, or `Cooperative` when the executor can only inspect
  time/cancellation after control returns or yields;
- safe error categories without raw headers or URLs;
- no automatic proxy-environment use unless the caller explicitly enables it.

Static dispatch is the default transport path. Heterogeneous boxed transports
are a separate `alloc`-gated convenience and must document object-safety and
MSRV behavior. Adapter errors are collapsed immediately into a closed safe code
and opaque diagnostic ID; an arbitrary adapter error is never retained as an
error source because it may contain headers, URLs, bodies, or credentials.

Because project crates cannot depend on third-party HTTP/TLS clients, concrete
ecosystem adapters are not admitted under the current policy. Users bridge
their maintained transport through the public trait. A future adapter requires
an explicit dependency-policy change.

Reviewed adapters must return redirects as data, disable automatic proxy
discovery and redirects by default, avoid unmetered decompression and
buffering, and translate adapter errors immediately into closed safe
categories. These are conformance properties, not guarantees about arbitrary
implementations.

Canonicalization and redirect fixtures cover encoded separators and dot
segments, duplicate query keys, backslashes, Unicode-equivalent spellings,
fragments, scheme-relative locations, and encoded controls. Every case is
either uniquely represented by the operation grammar or rejected before
credential injection; generic URL normalization is not delegated to the
transport.

A clock value alone never creates a hard deadline. A blocking transport that
does not return and an async transport that never wakes cannot be preempted by
the portable executor in `Cooperative` mode. Documentation, errors, tests, and
provenance preserve the selected deadline mode; hard-deadline language is
reserved for an adapter or runtime that actually supplies preemption.
Never-returning conformance cases run in bounded subprocesses or under an
external watchdog so the test suite itself cannot hang.

The same trust distinction applies to all external authorities:

| External boundary | Sweden-owned behavior | Conforming implementation | Arbitrary implementation |
| --- | --- | --- | --- |
| Transport | closed plans and safe wire handoff | conformance-tested origin/TLS/proxy/redirect behavior | may send, retain, or fabricate anything |
| Clock | explicit monotonic/UTC requirements, ephemeral epoch, and fail-closed unknown/reset/wrap | passes rollback/jump/restart/epoch tests | may lie, stall, reset, or reuse an epoch |
| Quota authority | permit required where policy demands coordination | atomic reviewed admission semantics | may over-admit |
| Credential provider | narrow source/environment/scope request | returns only correctly scoped credentials | may return, retain, or log secrets |
| Cache/state store | typed directives, partitions, and bounded values | honors denial, purge, version, and collision rules | may retain forbidden/stale data |
| Policy/kill-switch authority | version/expiry/revocation and observation epoch are validated | supplies authenticated current state bound to the requested epoch | may suppress revocation, roll back, or replay an old observation |
| Allocator | logical/requested/container budgets checked where observable | documents rounding, metadata, and failure behavior | may consume more physical memory than requested |
| Event sink callback | borrow lifetime, bounded decisions, and safe error collapse | returns promptly and honors pause/stop/abort | may copy data, block, panic, or consume arbitrary CPU |

Stronger guarantees require Sweden-controlled implementations or deployment
isolation. Traits alone do not make these authorities trustworthy.

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

Each example declares its capability tier (`no_std/no_alloc`,
`no_std+alloc`, `std` orchestration, or external adapter). MSRV checks cover
each supported feature combination rather than only the aggregate feature
graph.

Parsers additionally need:

- deterministic mutation tests;
- corpus replay;
- stack-depth evidence;
- exact-consumption tests;
- no-panic arbitrary byte tests;
- differential evidence against independently generated fixtures when that
  does not add a project dependency.

`ConformanceReplay` and `CorpusReplay` are never interchangeable. Corpus runs
exercise parser safety only and cannot create current evidence, final
provenance, caches, checkpoints, or execution authority. Corpus admission also
checks retention validity: expired or withdrawn official bytes are purged or
denied rather than preserved merely because replay is non-authoritative.

Live tests are prohibited through `v0.36.0`. Once admitted at `v0.37.0`, they
are opt-in, rate-limited by the reviewed authority, secret-safe, and never a
substitute for deterministic fixtures.

## 11. Security Work From Day One

Security deliverables grow with implementation:

- repository threat model and security policy;
- dependency/source deny policy;
- no-unsafe and no-third-party validators;
- payload-free logging rules;
- secret type and redaction snapshots before credentials;
- parser budgets before parser exposure;
- producer-owned completion witnesses and hostile-forgery tests before final
  provenance;
- event-sink control, safe error, pause/abort, and panic-boundary tests before
  callback exposure;
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
- coordinated deployments require a reviewed `QuotaAuthority`, while local
  advisory limiting is never described as an agency-wide quota guarantee;
- there is no arbitrary-origin path in Sweden-owned execution;
- no credential-leaking path exists in Sweden-owned planning, execution,
  diagnostics, caching, fixtures, or replay code; arbitrary transports remain
  trusted and outside this guarantee;
- caller-owned transport trust and Sweden-controlled executor guarantees are
  documented without cryptographic-sandbox claims;
- operation-level policy, dossier, provenance, and expiry evidence gates every
  stable capability;
- every attempt passes non-downgradable time-of-use freshness revalidation,
  epoch-valid authority observation, late quota reservation, and an
  authority-local commit whose pre-network crash gap is conservatively spent;
- direct execution documents the residual final-check/transport revocation
  race and never claims atomic revocation without a controlled broker or
  authority-issued one-attempt grant;
- downstream operation implementations and descriptive IDs cannot mint
  registry membership or execution authority;
- every authorized request uses the closed dossier-generated header model;
  no caller-set framing, protected-header override, hop-by-hop field, or raw
  header escape hatch reaches transport;
- borrowed events cannot escape their visitor callback, and provisional stream
  events cannot create complete provenance, cache entries, or checkpoint
  advances before private HTTP, exact-codec, registry-semantic, and
  executor-final witness consumption with one invariant attempt brand;
- valid completion witnesses from concurrent same-operation attempts cannot
  be mixed, and paused decoding owns and charges input/carry exactly once;
- official fixture retention follows its operation-level classification,
  retention, and redistribution decision, and authoritative replay
  additionally requires current matching evidence;
- historical official bytes enter only powerless `CorpusReplay` while their
  retention permission remains valid; expiry or withdrawal purges or denies
  them, and corpus mode never creates current conformance, provenance, caches,
  checkpoints, or execution;
- external clock, quota, policy, credential, cache/state, and kill-switch
  authority trust is documented and tested without extending Sweden-owned
  guarantees to arbitrary implementations;
- borrowed, `alloc`, `std`, and transport feature boundaries are verified;
- generated and handwritten Rust source files all remain below 500 lines;
- public docs contain no unsupported production claims;
- independent security review and maintainer pentest findings are resolved in
  the versioned repository report;
- release evidence, GitHub CI, and CodeQL default setup are green.

Only then may the project describe `1.0.0` as serious production-ready
infrastructure for its documented scope.
