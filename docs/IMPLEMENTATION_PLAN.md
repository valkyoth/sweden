# Sweden Implementation Plan

Status: repository foundation implemented; product capabilities planned

Repository and facade crate: `sweden`

Current workspace version: `0.2.0` release candidate

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
- accepted: authorization, time-of-use policy validation, credential/provider
  selection, quota reservation, credential injection, and an in-flight attempt
  are distinct non-cloneable states; `AuthorizedExecution<R>` binds the quota
  requirement but never owns a pre-acquired lease;
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
- accepted: response finalization is selected by a closed registered
  status/outcome profile; body success, `304 Not Modified`, reviewed empty
  success, redirect, and source error paths have distinct proof requirements,
  and only a body success or a fully matched cached finalized value can return
  successful `Finalized<R>` provenance;
- accepted: quota partition identity is dossier-selected, not caller-created;
  a generated `QuotaScope` recipe binds source/global, origin/environment,
  coordinated deployment/IP, credential-pool, operation, or an explicitly
  reviewed combination, and credential-pool identity comes opaquely from the
  credential provider without hashing secret bytes;
- accepted: data-entitlement/cache isolation is independent of quota pooling;
  a dossier-generated `DataAccessScope` resolves to an authority-derived
  opaque `AccessPartitionId`, with a registry-owned public partition for
  anonymous public data and a provider-owned entitlement partition for
  credentialed data. Caller namespaces may narrow but never merge partitions;
- accepted: selecting the initial non-secret provider binding establishes the
  execution's starting access assertion and does not consume
  `AccessRebindLedger`. Every later provider access assertion pre-charges
  exactly one rebind unit before invoking the provider. Failure to obtain that
  unit is `AccessUnstable`; after a successful assertion changes partition,
  failure to pre-charge the unchanged parent `CacheLookupWork` is
  `CacheWorkExhausted`. This transition order fixes error precedence when both
  ledgers have no remaining capacity;
- accepted: non-secret credential/access binding and secret materialization
  are separate;
  `CredentialBindingSelected<R>` contains only an opaque provider token,
  quota/access partitions, generation, and expiry. A matching one-use
  `SecretLease` is materialized only after cache miss, quota admission, and
  final policy revalidation, then injected immediately;
- accepted: cache lookup is part of the executor state machine before quota or
  network work. `sweden-executor` owns optional blocking/async `CacheStore`
  contracts through an explicit `NoCache` default, while registry-bound
  policy owns cache permission/identity and `v0.38.0` supplies full algorithms;
- accepted: every cache path—fresh, stale-within, cache-only, miss, and
  `304`—revalidates kill switch, cache permission, evidence/version,
  `DataAccessScope`, `AccessPartitionId`, and data classification before
  returning or reusing a value;
- accepted: cache validators are derived only from the already-selected cache
  entry and inserted late by executor-owned cache logic; they are not caller
  input or part of the base cache key and remain bound to the exact partition,
  policy/schema/registry versions, and reviewed `Vary` identity;
- accepted: `BodyWireBytes` means content-coded body bytes delivered by the
  adapter after TLS and HTTP transfer framing are removed but before any
  content decoding/decompression; it excludes TLS records, HTTP/2 or HTTP/3
  frames, chunk framing, headers, and other network overhead, so true
  bandwidth enforcement remains an adapter/deployment capability;
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
- accepted: XML has a dedicated consumable `XmlWork` ledger and progress
  invariant covering namespace traversal, QName/end-tag/common-prefix
  comparison, duplicate expanded attributes, and character references; every
  parser step consumes input, emits one event, requests input, or exhausts a
  ledger;
- accepted: a registry-bound `DataHandlingProfile` makes reviewed
  classification, cache, recording, transform, export, retention/purge, and
  sensitive-field diagnostic behavior executable and survives in finalized
  results. Once decoded data is returned, caller copying/logging remains
  outside SDK enforcement;
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
elements, pages, records/cells, redirects, attempts, time, retries, and
provider-driven binding/access rebinds. There is no unbudgeted `collect_all`,
response buffering, decompression, archive extraction, recursive parse, or
authority-driven restart loop.

Configured ceilings and consumed state are separate types. Charges use checked
arithmetic and occur before accepting bytes, allocating, transmitting an
attempt, following a redirect, fetching a page, or committing a checkpoint.
Callers may tighten reviewed ceilings but cannot raise them through a stable
public API.

Seven mechanisms remain distinct:

- `Limits`: immutable operation maxima that callers may only tighten;
- `Ledger`: non-`Copy`, non-`Clone` local capacity charged before work;
- `QuotaAuthority`: caller/deployment coordination for time, concurrency, and
  shared upstream quotas under the registry-bound `QuotaScope` recipe; callers
  cannot mint a fresh production partition to escape prior consumption;
- `PolicyAuthority`: optional caller/deployment source of current revocation
  and monotonic policy-version state;
- `DataHandlingProfile`: registry-bound executable classification, access,
  cache, recording, transform/export, retention, and diagnostic restrictions;
- `CacheStore`: optional caller-owned bounded storage behind executor-owned
  blocking/async contracts; it never decides policy or access identity;
- `AuthorizedExecution<R>`: a one-use registry-created package binding the
  exact canonical plan, registered encoder/decoder/validator profile,
  output/provenance type, finalization rules, policy, ledgers, quota/data-access
  scopes, cache requirements, and `DataHandlingProfile`—but not an acquired
  quota lease or selected cache entry. The executor never accepts an
  authorization token alongside a separately caller-selected decoder,
  semantic validator, access partition, or handling profile.

`sweden-core` introduces a checked non-cloneable `RestartLedger<Tag>` at
`v0.3.0`; the tag distinguishes consumption domains but grants no authority.
The registry binds a reviewed `AccessRebindLimit`, and the executor privately
constructs and owns its `AccessRebindLedger` instance for the complete
cache-resolution state machine. A caller-created generic ledger cannot replace
it.
The initial binding establishes the starting assertion without charging this
ledger. Before every subsequent provider access revalidation/reselection, the
executor pre-charges exactly one rebind unit; unavailable capacity returns
closed `AccessUnstable` before the provider is called. If the successful
assertion changes partition, the executor then pre-charges the existing parent
`CacheLookupWork`; unavailable lookup capacity returns
`CacheWorkExhausted`. This ordered transition is the deterministic precedence
when both ledgers are exhausted. Neither ledger is replaced, refunded, or
recreated during expiry, epoch change, provider restart, cache-fill wakeup,
`304`, or `CacheOnly` handling. Either exhaustion discards the current
candidate and cannot fall back to an earlier partition or entry. The total
deadline remains a secondary time bound, not the only protection against a
fast oscillation loop.

A per-process limiter is described as advisory unless the dossier explicitly
establishes that scope. Hosted or multi-process modes requiring coordinated
quota enforcement fail closed when the authority or trustworthy time source is
unavailable.

`QuotaScope` is source-dossier data, not a free-form key. Its closed recipes
cover source-global, origin/environment, coordinated deployment/IP,
credential-pool, operation, and specifically reviewed compositions. For
credential-pool scope, the conforming provider includes an opaque
`CredentialPartitionId` in the non-secret credential binding. The identifier
is stable across rotation and aliases that share one upstream pool; raw
credential bytes are never materialized merely to choose it and are never
hashed, serialized, logged, or accepted as partition material.
Source-global, deployment/IP, and other cross-client recipes require a
coordinated authority and fail closed when it is unavailable. Cache/data
partitioning remains a separate typed decision and cannot silently subdivide a
quota scope.

`DataAccessScope` separately describes data entitlement. Anonymous public
operations receive a registry-owned global `AccessPartitionId`; credentialed
operations receive a provider-owned opaque partition in the same non-secret
binding as the quota identity. Rotation preserves it only when entitlement is
unchanged. A caller may add a bounded local namespace that narrows storage but
cannot construct, merge, replace, or broaden authoritative partitions.

### 2.7 Closed request-header model

`CanonicalPlan<Unauthenticated>` admits headers only through typed categories:

- `ReviewedStaticHeader` for dossier-fixed representation choices such as
  `Accept`, `Content-Type`, API version, and `Accept-Encoding: identity`;
- `CredentialHeaderSlot` for protected late injection after origin and policy
  revalidation;
- `CacheValidatorSlot` for typed bounded `If-None-Match` and
  `If-Modified-Since` values selected only from an already-matched cache entry
  and filled late by executor-controlled cache logic, never by the caller;
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
Cache-validator values are excluded from the base cache key so revalidation
does not create a new identity; their slot is instead bound to the selected
entry's exact base key, authoritative `AccessPartitionId`, local narrowing
namespace, validator, policy/schema/registry versions, classification/
`DataHandlingProfile`, and reviewed `Vary` identity.

No raw header map or generic `(name, value)` escape hatch exists in an
authorized plan. Source dossiers and generated registry entries enumerate
every admitted header slot, and the executor revalidates the closed set before
credential injection and transport handoff.

### 2.8 Cache and access boundary

`sweden-registry` supplies minimal non-secret cache entry, identity,
`DataAccessScope`, `AccessPartitionId`, and permission vocabulary at `v0.9.0`.
`sweden-executor` introduces `BlockingCacheStore` and `AsyncCacheStore` with
semantic parity at `v0.21.0`. The explicit default is `NoCache`; no feature
silently enables storage. `Client<T, C, Q, P, K, S = NoCache>` owns `S`, while
one-shot execution accepts the same store contract explicitly.

Cache entry trust is closed rather than inferred from a store trait:

- `EphemeralOpaque` retains the executor's non-serializable, privately
  constructed `Finalized<R>` inside one process and one opaque `CacheEpoch`;
  restart or epoch mismatch invalidates freshness and the entry cannot be
  reconstructed from bytes.
- `AuthenticatedPersistent` is an explicit externally trusted capability. It
  must authenticate the complete entry and supply trusted absolute expiry
  plus rollback-resistant authority state before the registry can reconstruct
  an entry through a private validation path. Sweden ships no dependency-free
  persistent authority for 1.0.
- `UntrustedBytes` may be decoded and semantically checked again under normal
  input budgets, but cannot deserialize or mint `Finalized<R>`, satisfy a
  `304`, or claim source provenance. It is corpus-like input, not a
  provenance-preserving cache hit.

No generic serialization/deserialization implementation exists for
`Finalized<R>`. A store returning more than one candidate that exactly matches
the full identity is ambiguous and fails closed; insertion order or “first
match” never decides authority.

The store is a caller-owned trust boundary. A conforming implementation:

- returns at most the requested bounded number of collision candidates;
- supports atomic revisioned complete-entry replacement and fail-closed purge;
- never returns a partially written entry as valid;
- collapses storage failures to closed safe codes; and
- preserves the canonical identity, authoritative access partition,
  versions, classification, provenance, validator, `Vary`, and expiry fields.

The executor, not the store, derives cache permission and base identity,
charges `CacheLookupWork`, compares every candidate's full canonical identity,
and rejects excess candidates before unbounded collision work. Every fresh,
stale-within, cache-only, miss, and `304` decision rechecks current kill
switch, policy/evidence/registry/schema versions, cache permission,
`DataAccessScope`, `AccessPartitionId`, and classification.

A fresh/stale decision also consumes a closed cache-time proof. Ephemeral
entries use monotonic ticks only within their originating `CacheEpoch`;
restart, reset/wrap, epoch mismatch, future creation time, or invalid
subtraction invalidates the entry. Persistent/shared entries require the
explicit authenticated capability, trusted absolute expiry, and a
rollback-resistant authority sequence; caller/store timestamps alone carry no
authority. Upstream `Date`, `Age`, `Expires`, and `Cache-Control` may narrow
the dossier maximum but never broaden it unless the exact operation review
explicitly says otherwise. Malformed or future-dated metadata fails closed.

A fresh hit returns a prior `Finalized<R>` only after those checks. A
cache-only miss returns a bounded miss without quota or I/O. A stale/miss path
selects a validator from the exact candidate, then continues to quota
admission. Cache errors follow the registered fail-closed/explicit-network-
fallback policy and never turn an unvalidated candidate into a hit.

Conforming stores declare bounded total entries, owned/encoded bytes, key and
validator bytes, authoritative and local partition cardinality, and
eviction/purge/expiry-cleanup work. Exhaustion returns stable `StoreFull`.
Ordinary insertion failure preserves an otherwise successful live result;
failure to purge data now forbidden by policy or handling rules is a surfaced
policy/storage violation and cannot be silently downgraded.

An optional fenced `CacheFillLease` may coalesce requests only within one
exact shareability domain. Its key is a registry-produced opaque
`CacheFillIdentity`, not a caller string or hash alone. The identity binds the
full canonical request, authoritative access partition, local narrowing
namespace, environment, origin, schema/registry/policy versions,
classification, `DataHandlingProfile`, reviewed representation/`Vary`
dimensions, and raw versus exact transformed-result identity. Unrelated local
namespaces, policy revisions, representations, or transform outputs never
share leadership.

Waiters hold no quota or credential lease; only the elected filler proceeds to
upstream admission. Every lease carries a monotonically fenced publication
token in the store's coordination epoch. Complete-entry publication atomically
checks that fence, and `304` validator/metadata publication additionally
compare-and-swaps the exact selected `CacheEntryRevision`. Expired leaders,
store restart/epoch mismatch, revision mismatch, wrap/reset, or takeover
return closed `StaleFence`/`RevisionChanged` without writing. Publication and
release are idempotent only for the same fence/revision and exact complete
entry; a conflicting repeated publication fails closed. Cancellation after a
live response but before publication preserves the live result but cannot
perform an unfenced write.
Cancellation or expiry allows one bounded fenced takeover. Cross-process
coalescing is claimed only by a coordinated store, and unsupported stores
honestly claim no stampede protection.

### 2.9 Honest capability claims

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
| `sweden-policy` | source-independent dossier evaluation, revocation/expiry logic, closed response-outcome, cache/data-access/quota-scope, and data-handling requirement contracts | source registry data, transport calls, credentials, source decoding |
| `sweden-registry` | generated closed membership, exact profile/header/outcome/quota/access/data-handling compatibility, opaque `AuthorizedExecution<R>`, global public access partitions, epoch-bound authority observation, invocation of bound success/error validators, private branded semantic witness | generic policy algorithms, cache storage, transport calls, credentials, wire implementations |
| `sweden-http` | blocking/async transport, normalized response metadata and body sink, redirect-as-data, safe transport codes, private branded `WireComplete` witness | authorization, retries, credential injection, agency semantics, network-bandwidth claims |
| `sweden-executor` | time-of-use revalidation, generative attempt scope, non-secret credential/access binding, optional blocking/async cache-store contracts and lookup state, quota-scope resolution, late `SecretLease` materialization/injection, status-specific response dispatch, redirect/retry state machines, exact-branded-witness consumption, private `Finalized<R>`/complete provenance, `Client<T, C, Q, P, K, S>` | concrete cache backend or HTTP/TLS, ambient discovery, source-specific wire truth, synthetic source semantics |
| Codec crate | bounded syntax, event visitor, private branded codec-specific completion witness such as `JsonComplete` or `XmlComplete` | wire/semantic completion, I/O, policy authority |
| `sweden-conformance` | synthetic operations, encoders, decoders, validators, output types, and fixtures | registry authority, generic execution, production source claims |
| Agency crate | typed operation metadata, inputs, encoding, decoding, semantic validation, generated sensitive-field handling metadata | authority issuance, cache stores, sockets, TLS, generic execution, other agencies |
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
   authority decision, quota/data-access scopes, cache/data-handling decisions,
   and finalization behavior.
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

### 3.4 Status-specific response outcomes

Completion is a closed registry-selected algebra, not one universal
wire-plus-codec-plus-semantic recipe:

- `BodyResponse<R>` requires same-attempt `WireComplete`, the exact registered
  codec witness, and registered semantic completion before new
  `Finalized<R>`/`Complete` provenance exists.
- `NotModified<R>` requires a complete reviewed `304` response and an
  already-selected previously `Finalized<R>` cache entry. The executor
  revalidates current cache permission and exactly matches cache key,
  authoritative access partition, local narrowing namespace, validator,
  schema, policy and registry versions, `DataHandlingProfile`, classification,
  and every reviewed `Vary` dimension. It returns the prior finalized value
  with a revalidation record; it never manufactures new semantic completion or
  runs the normal body decoder.
- `EmptyResponse<R>` is admitted only when the registry profile names that
  status and a `NoBody` semantic output. It requires wire completion with an
  exactly empty body and cannot substitute for a body-bearing success.
- `RedirectResponse` contains only bounded normalized redirect metadata for
  the redirect state machine. It never creates success provenance, cache
  content, or a source result.
- `SourceErrorResponse<E>` uses the exact registered bounded source-error
  status/media/decoder/profile and produces only a safe typed source error.
  Even a fully decoded error cannot create `Finalized<R>` success provenance.

All variants carry the same invariant attempt brand and exact registered
status profile. Unknown, conflicting, or wrong-profile statuses fail closed.
The executor alone dispatches the algebra; neither a transport, cache, codec,
nor caller may reinterpret one outcome as another.

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
non-secret credential/access binding selection:
provider token + binding epoch + quota/access partitions + generation/expiry
        ↓
CredentialBindingSelected<R>
        ↓
cache permission + canonical identity derived and bounded lookup
        ↓
CacheResolved<R>
        ├─ fresh hit → pre-charge rebind → provider assertion
        │       ├─ no rebind capacity → AccessUnstable before provider access
        │       ├─ provider unavailable/expired → deny
        │       ├─ same partition/current → AccessRevalidated<R>
        │       │       └─ prior Finalized<R>
        │       └─ changed → pre-charge parent lookup ledger
        │               └─ repeat under new partition or CacheWorkExhausted
        ├─ cache-only miss → bounded miss, no quota or I/O
        └─ stale/miss → exact entry validator or no validator
        ↓
QuotaLeaseAcquired<R>
        ↓
final policy/binding revalidation after any quota wait
        ↓
SecretLeaseMaterialized<R>
        ↓
immediate credential injection into a reviewed execution sink
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
closed registered response-outcome dispatch
        ├─ body → exact codec + semantic validation
        ├─ 304 → access revalidation + exact entry revision/CAS
        ├─ reviewed empty → exact NoBody profile
        ├─ redirect → bounded redirect state, never success
        └─ source error → registered error profile, never success
        ↓
typed outcome; success provenance only for admitted success branches
        ↓
DataHandlingProfile + current cache permission
        ↓
optional atomically fenced revisioned publication or fail-closed purge
```

Every live attempt traverses these private, non-`Copy`, non-`Clone` states.
The executor creates them inside a higher-ranked attempt scope; no state or
partial completion witness can be moved to another attempt, even when two
concurrent executions have identical Rust types and operation metadata.
`PolicyRevalidated<R>` checks the bound freshness requirement, expiry,
registry/policy version, revocation, kill switch, origin, and environment
against trustworthy time and any required authority. The check occurs
immediately before non-secret credential/access binding, before cache return,
before quota acquisition, and again before secret materialization/I/O; retry
delays, redirects, and page transitions return to policy
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

`CredentialBindingSelected<R>` contains no secret bytes. It holds either the
registered anonymous binding or an opaque provider token plus the bound
`CredentialBindingEpoch<'provider>`, `CredentialPartitionId`,
`AccessPartitionId`, provider generation, and expiry. The executor establishes
the non-serializable invariant epoch for one borrowed provider session;
generations are meaningful only inside it. A conforming provider terminates
that session on backend restart and never resets, wraps, or reuses a generation
within an epoch. Restart, reset/wrap, replay, epoch mismatch, or provider
replacement forces a new binding selection. The token is non-`Copy`,
non-`Clone`, non-serializable, and consumed by one secret materialization or
one terminal cached return. A caller-owned provider can lie about its session;
the executor does not claim to sandbox it.

A conforming provider preserves the quota partition across rotations/aliases
sharing one upstream rate pool, while preserving the access partition only
when data entitlement is unchanged.

`CacheResolved<R>` proves current cache permission, authoritative access
partition, data-handling profile, canonical identity, collision-candidate/work
budgets, selected entry revision, shared parent `CacheLookupWork`,
`AccessRebindLedger`, and lookup outcome. Before every
credential-partitioned cached return—including an immediate hit, a cache-fill
waiter wakeup, and a `304`—the executor pre-charges exactly one
`AccessRebindLedger` unit, consumes the earlier binding, and asks the provider
to reselect/revalidate non-secret access. Initial binding selection alone does
not charge this ledger. If that pre-charge fails, the provider is not called
and the result is `AccessUnstable`. The result becomes a private
`AccessRevalidated<R>` only when its epoch/generation/expiry is current and
its `AccessPartitionId` exactly matches the candidate. A changed partition
discards the candidate, then pre-charges the same parent cache-work ledger; if
that second charge fails, the result is `CacheWorkExhausted`, otherwise lookup
repeats under the new partition without replenishing either ledger. This
ordering gives `AccessUnstable` precedence before provider access and
`CacheWorkExhausted` only after a changed-partition result. Expiry, epoch
churn, provider restart, or another later assertion consumes further rebind
capacity; neither exhaustion permits older-partition/candidate fallback.
Revocation or provider unavailability fails closed for protected data.
Anonymous global entries use the registry-owned access assertion but still
pass the same closed state. `CacheOnly` uses identical rebind limits; a miss
terminates without quota. Only stale/miss paths continue.

For a `304`, access revalidation occurs again after the network wait and before
the prior finalized value is returned. Any permitted validator/metadata update
uses compare-and-swap against the selected `CacheEntryRevision`; a concurrent
replacement or stale fill fence preserves the selected prior finalized result
but rejects the update.

`QuotaLeaseAcquired<R>` is an uncommitted attempt reservation plus concurrency
lease acquired as late as possible. Its authority key is resolved from the
registry-bound `QuotaScope` and `CredentialBindingSelected<R>`, never from a
caller-selected partition.

After any quota wait, the executor revalidates policy and asks the provider to
materialize one non-cloneable `SecretLease` for the opaque binding. The
provider must attest the same quota/access partitions and generation and a
still-valid expiry. Mismatch, expiry, rotation with changed identity, or
provider revocation cancels the unused reservation once and restarts at policy
and binding selection. A valid lease is injected immediately and not retained
for retry. Credential-binding failure occurs before quota reservation; later
materialization, policy, or injection failure cancels the unused reservation
and releases concurrency at most once. None spends a network-attempt budget.
Transition to
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
surface. `Client<T, C, Q, P, K, S = NoCache>` is introduced and owned by
`sweden-executor` at `v0.21.0`; it may hold caller-supplied transport, clock,
quota authority, policy authority, credential provider, and explicit cache
store but discovers none from ambient process state. Blocking and async cache
contracts match the corresponding execution style; `NoCache` is the default
and performs no storage.

Pre-I/O `Cost` estimates expose reviewed maxima and selected query/projection
work, but remain advisory; consumable ledgers are authoritative. Page,
offset, time-window, cell-partition, and change-checkpoint continuations remain
source-specific types. No `Iterator` hides network I/O and no universal
pagination abstraction erases upstream semantics.

The API must make it impossible to select an arbitrary production origin.
Credentials are inserted only after the origin is validated and are excluded
from debug output, cache keys, canonical hashes, errors, and fixtures.
Authorized executions, policy/access-revalidated states, credential/access
bindings, cache decisions/fill leases/publication fences, quota
reservations/leases, secret leases, credential-injected states, and in-flight
attempts are non-`Copy`, non-`Clone`, operation-, environment-, and
origin-bound. Retries, redirects, and subsequent pages require fresh checks,
charges, and authorization.

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
   Enumerate each accepted response status/outcome and one generated
   `QuotaScope`, including provider pool identity/rotation semantics and
   required coordination. Define `DataAccessScope` and a complete
   `DataHandlingProfile` for cache, recording, transform, export,
   retention/purge, and sensitive-field diagnostic behavior.
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
`AuthorizedExecution<R>` carries the registry-bound `DataHandlingProfile`, and
`Finalized<R>` preserves it with provenance. Each Sweden-owned cache,
testkit/fixture, transformation/export, retention/purge, diagnostic, and
generated sensitive-field `Debug` path consumes the relevant closed decision
instead of reinterpreting prose. Callers may narrow handling or local
namespaces but cannot broaden permissions or merge access partitions. Once a
decoded value is returned, Sweden cannot prevent caller code from copying,
logging, retaining, transforming, or redistributing it; documentation states
that trust boundary without weakening Sweden-owned enforcement.
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

For a registered body-bearing success, streaming events remain provisional
until three opaque producer-owned witnesses exist: HTTP wire completion, the
exact registered codec completion, and registry-owned source-semantic
completion. Each carries the same executor-generated invariant attempt brand.
Only `sweden-executor` can consume their same-attempt, registry-bound
combination to construct new `Finalized<R>` and `Complete` provenance.
`304`, no-body, redirect, and source-error statuses use their distinct closed
outcome rules and cannot manufacture this body-success chain. Provisional data
cannot enter a cache or advance a cursor/checkpoint. Callers that act on
provisional events before composite finalization explicitly own the downstream
rollback/compensation risk.

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
- a consumable `XmlWork` ledger charged before namespace-scope traversal,
  QName resolution, long common-prefix/end-tag comparisons, duplicate
  expanded-attribute checks, and each character-reference digit/step;
- a progress invariant: every parser transition must consume at least one
  input byte, emit exactly one event, return `NeedInput`, or exhaust a declared
  ledger—no unchanged state may spin;
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

Adversarial XML evidence includes many attributes with long common prefixes,
deep namespace shadowing, repeated long end tags and character references, and
one-byte chunking. Finite input/attribute limits are not treated as a substitute
for computational-work accounting.

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
9. Strict status-specific response outcomes: body success, reviewed no-body,
   bounded redirect metadata, cache revalidation, and upstream error decoder.
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

`BodyWireBytes` is deliberately a Sweden/adapter handoff metric, not an
on-the-network metric. It counts content-coded response body octets delivered
to the sink after TLS and HTTP transfer framing are removed and before any
content decoding or decompression. TLS records, HTTP/2 and HTTP/3 frames,
chunk framing, request/response headers, retransmission, and other protocol
overhead are excluded. Header/trailer bytes have their own bounded metadata
budgets. Enforcing actual link/network bandwidth requires a conforming
transport or deployment control and is never inferred from `BodyWireBytes`.

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
categories. They return closed normalized response metadata and fail closed on
conflicting `Content-Length` or transfer-framing claims, duplicate singleton
`Content-Type`, `Content-Encoding`, `Location`, or validator fields,
unreviewed informational responses, and forbidden or ambiguous trailers.
These are conformance properties, not guarantees about arbitrary
implementations, which may fabricate metadata or misreport byte counts.

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

The selected total deadline covers every executor wait: policy refresh,
credential binding/materialization, cache lookup/fill wait/replacement/purge,
quota acquisition, and transport. In `Cooperative` mode each boundary receives
the remaining budget and must cooperate; Sweden cannot forcibly preempt a
never-waking external future. Cancellation drops uncommitted cache-fill and
quota leases with fenced at-most-once cleanup, never retains a `SecretLease`,
and follows the existing committed/in-flight ambiguity rules. Cache insertion
timeout normally preserves a successful live value, while required purge
timeout is a policy/storage violation. Runtime-backed hard cancellation must
document and test which external operations it actually interrupts.

The same trust distinction applies to all external authorities:

| External boundary | Sweden-owned behavior | Conforming implementation | Arbitrary implementation |
| --- | --- | --- | --- |
| Transport | closed plans, normalized metadata contract, and body-wire ledger after transfer framing | conformance-tested origin/TLS/proxy/redirect, singleton/framing, and byte-accounting behavior | may send, retain, fabricate metadata, or misreport body/network bytes |
| Clock | explicit monotonic/UTC requirements, ephemeral epoch, and fail-closed unknown/reset/wrap | passes rollback/jump/restart/epoch tests | may lie, stall, reset, or reuse an epoch |
| Quota authority | permit required for the registry-bound `QuotaScope` | atomic reviewed admission under the exact generated partition recipe | may over-admit or treat forged partitions as fresh |
| Credential provider | narrow source/environment/scope binding and private provider-session epoch, then access revalidation/one-use materialization | returns non-secret quota/access partitions and epoch-bound generation/expiry, reports restart, revalidates protected-cache access, then returns a matching short-lived `SecretLease`; preserves only identities whose pool/entitlement is unchanged | may return, retain, replay, or log secrets/tokens, lie/change identity, reuse an epoch/generation, disappear during revalidation, or cause partition explosion |
| Cache/state store | executor derives policy/full fill identity/time, rejects duplicate exact candidates, and never deserializes `Finalized<R>` | honors declared capacity, atomic fenced revisioned replacement/purge, CAS, idempotent release/publication, safe errors, exact metadata/access partitions, entry trust level, cache/coordination epoch or authenticated expiry, and candidate ceilings | may retain/forge forbidden or stale data, cross partitions/epochs, lie about time/capacity, accept stale fences/revisions, return partial entries, or amplify collisions |
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
- progress-invariant and computational-work exhaustion tests;
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
- status-specific body/`304`/empty/redirect/source-error outcome substitution
  tests before any response can finalize;
- generated quota-scope and opaque credential-partition forgery/rotation/
  aliasing/multi-client tests before live execution;
- authority-derived access-partition, bounded cache-candidate/work, accidental
  shared-store, and fresh/stale/cache-only/`304` revalidation tests before
  cache exposure;
- post-cache-wait credential access revalidation, binding epoch/generation ABA,
  token consumption, changed-partition restart, and provider-unavailable
  denial tests before credential-protected cache return;
- finite access-rebind restart ledgers, shared parent cache-work continuity,
  A/B partition oscillation, repeated expiry/epoch/provider restart,
  uncharged initial binding, exactly charged later assertions, deterministic
  `AccessUnstable`/`CacheWorkExhausted` precedence, `CacheOnly`, and
  no-fallback tests before relookup;
- cache-entry trust, non-deserializable provenance, cache epoch/restart/time
  rollback, duplicate-exact-candidate, capacity/partition-cardinality, and
  conditional-validator tests before cache stabilization;
- full-shareability fill identity, atomically fenced publication, entry
  revision/CAS, expired-leader late-write, and idempotent publication/release
  tests before cache-fill coalescing;
- cancellation and never-returning tests for cache, policy, quota, and
  credential authorities as well as transport, with fenced cleanup evidence;
- two-phase non-secret credential binding and late `SecretLease`
  expiry/rotation/revocation/mismatch tests before credentials;
- event-sink control, safe error, pause/abort, and panic-boundary tests before
  callback exposure;
- SSRF and redirect tests before transport execution;
- cache partition tests before caching;
- executable `DataHandlingProfile` tests before cache, fixture, transform,
  export, retention, or sensitive-field diagnostics can claim support;
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
- XML namespace/QName/attribute/end-tag/reference work is charged through
  `XmlWork`, and every parser step satisfies the consume/event/need-input/
  exhaust progress invariant;
- `BodyWireBytes` is documented and tested as adapter-delivered content-coded
  body bytes rather than total network bandwidth, and conforming metadata
  normalization rejects conflicting framing/singleton/trailer state;
- coordinated deployments require a reviewed `QuotaAuthority`, while local
  advisory limiting is never described as an agency-wide quota guarantee;
- every production quota permit uses the dossier-generated `QuotaScope`;
  callers cannot mint partitions, credential-pool IDs survive shared-pool
  rotation/aliasing without secret-derived keys, and cross-client scopes
  require coordination;
- every cache entry uses the dossier-generated `DataAccessScope` and
  authority-derived `AccessPartitionId`; caller namespaces only narrow, cache
  candidates/comparison work are bounded, and every hit/mode revalidates
  current access, classification, evidence, policy, and kill switch;
- provenance-preserving built-in cache entries are opaque, in-process, and
  bound to one `CacheEpoch`; no generic byte deserializer mints
  `Finalized<R>`, duplicate exact candidates fail closed, and persistent/shared
  provenance requires an explicitly trusted authenticated capability with
  rollback-resistant absolute-expiry authority;
- cache freshness uses its declared clock/epoch contract, upstream cache
  metadata can only narrow dossier freshness by default, and rollback,
  forward jump, restart, epoch mismatch, future timestamp, or malformed time
  cannot silently extend reuse;
- cache capacity, partition cardinality, eviction, cleanup, and purge work are
  bounded; optional fill coalescing is fenced and never lets waiters hold
  quota or credential leases;
- credential bindings are non-serializable, non-cloneable, one-use, and bound
  to one `CredentialBindingEpoch`; every protected cached return revalidates
  provider access after waits, requires the same access partition, and
  restarts lookup or denies on expiry/revocation/restart/ABA/unavailability;
- provider-driven access revalidation/relookup is bounded by one
  registry-limited `AccessRebindLedger` and the unchanged parent cache-work
  ledger; initial binding is uncharged, each later assertion pre-charges the
  rebind ledger before provider access, and changed-partition relookup then
  pre-charges cache work. The ordered errors are `AccessUnstable` and
  `CacheWorkExhausted` respectively; neither ledger is recreated on restart
  and neither failure permits earlier-partition/candidate fallback;
- `CacheFillIdentity` contains the complete shareability domain, publication
  atomically checks a monotonic fence, `304` updates compare-and-swap the
  selected entry revision, and stale leaders/revisions cannot write;
- credential binding contains no secret material; a one-use `SecretLease`
  materialized after quota wait must match binding identity/generation/expiry
  and is injected immediately or the reservation is cancelled and selection
  restarts;
- there is no arbitrary-origin path in Sweden-owned execution;
- no credential-leaking path exists in Sweden-owned planning, execution,
  diagnostics, caching, fixtures, or replay code; arbitrary transports remain
  trusted and outside this guarantee;
- caller-owned transport trust and Sweden-controlled executor guarantees are
  documented without cryptographic-sandbox claims;
- the total deadline is propagated through cache, policy, quota, credential,
  and transport waits with honest cooperative/preemptive semantics and
  cancellation cleanup for every acquired lease;
- operation-level policy, dossier, provenance, and expiry evidence gates every
  stable capability;
- registry-bound `DataHandlingProfile` gates Sweden-owned cache, fixture,
  transform/export, retention/purge, and sensitive-field diagnostic behavior
  and is preserved in finalized provenance, without claiming control after
  values cross into caller code;
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
- the closed registered response algebra prevents `304`, reviewed empty,
  redirect, and source-error outcomes from masquerading as body success;
  cache revalidation matches a prior `Finalized<R>` entry and current cache
  permission without creating semantic completion;
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
