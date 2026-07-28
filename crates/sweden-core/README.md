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
Sweden agency crate. Version `0.1.0` provides only reviewed identifiers,
foundation status metadata, HTTP methods, and explicit response budgets.

It does not perform networking, parse upstream payloads, store credentials, or
claim that any agency integration is complete.

Licensed under `MIT OR Apache-2.0`.
