# Sweden Dependency Policy

Project crates use no third-party runtime, development, or build dependencies.
First-party path dependencies must also carry exact crates.io versions so every
package remains independently publishable.

A proposed third-party crate is not a routine manifest edit. Work stops until
the user explicitly authorizes an admission milestone covering:

- why first-party or standard-library implementation is unsafe or impractical;
- latest stable version and Rust `1.90.0` compatibility;
- license and source provenance;
- maintainer activity, advisories, unsafe code, build scripts, native code,
  network behavior, and default features;
- `std`, allocation, platform, and binary-size effects;
- alternatives and removal plan;
- boundary tests and fuzzing;
- updated threat model, SBOM, and pentest scope.

Unknown registries, unknown Git sources, wildcard versions, and unpinned Git
revisions are denied.
