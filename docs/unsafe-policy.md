# Sweden Unsafe Policy

Every first-party crate uses:

```rust
#![forbid(unsafe_code)]
```

Unsafe Rust is not admitted in the workspace.

If future platform FFI or a measured performance boundary appears to require
unsafe code, implementation stops for an explicit user and security decision.
Any exception would require a dedicated crate, documented invariants, a safe
fallback, Miri or sanitizer evidence, fuzzing of the safe wrapper, independent
review, and an exact-version pentest.

Unsafe code may never be mixed into source policy, request validation,
credentials, codecs, agency semantics, or the root facade.
