<p align="center">
  <b>Security-first, no_std-first Rust crates for Swedish public APIs and public data.</b><br>
  One reviewed crate per source, explicit resource budgets, and small auditable releases.
</p>

<div align="center">
  <a href="https://crates.io/crates/sweden-core">Crates.io</a>
  |
  <a href="https://docs.rs/sweden-core">Docs.rs</a>
  |
  <a href="https://github.com/valkyoth/sweden/blob/main/docs/RELEASE_PLAN.md">Release Plan</a>
  |
  <a href="https://github.com/valkyoth/sweden/blob/main/SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/sweden">
    <img src="https://raw.githubusercontent.com/valkyoth/sweden/main/.github/images/sweden.webp" alt="Sweden Rust crate overview">
  </a>
</p>

# sweden-core

`sweden-core` is the dependency-free, `no_std` foundation shared by every
Sweden agency crate. Version `0.2.0` provides borrowed canonical source,
operation, schema, policy, and upstream identifiers; closed reviewed source
spellings; non-zero schema/policy/upstream versions; descriptive foundation
metadata; HTTP methods; and explicit response budgets.

It does not perform networking, parse upstream payloads, store credentials, or
claim that any agency integration is complete. Identifiers and reviewed source
spellings are descriptive only. They cannot create registry membership,
operation authority, or permission to execute.

Canonical identifiers start with a lowercase ASCII letter. Remaining bytes are
lowercase ASCII letters, digits, `-`, or `.`, with no adjacent or trailing
separator. Constructors reject empty, overlong, non-ASCII, uppercase, or
otherwise non-canonical input without allocation or silent normalization.
Source IDs accept at most 63 bytes, schema and policy IDs 95 bytes, and
operation and upstream IDs 127 bytes.

Licensed under `MIT OR Apache-2.0`.
