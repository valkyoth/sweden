# Sweden Secret Handling Policy

No foundation crate stores or accepts credentials.

Before credential support is added:

- secret types must not implement `Copy`, secret-revealing `Debug`, or
  `Display`;
- credentials must be inserted only after a closed origin is selected;
- credential headers must use protected typed late-injection slots; callers
  cannot override them or supply a raw header map;
- canonical request hashes, cache keys, fixtures, metrics, logs, errors, and
  panic messages must exclude secret material;
- query-string credentials require a separate uncredentialed representation;
- test and production credentials must be separate types or environments;
- credential-pool quota identity must be an opaque provider value coupled to
  credential selection, stable across shared-pool rotation/aliases, and never
  derived by hashing or otherwise transforming raw secret bytes;
- credential partition IDs must not be logged or accepted from callers even
  though they contain no secret bytes;
- fixture and replay tooling must fail closed on protected headers;
- every secret-bearing error and debug path needs snapshot tests;
- hosted credentials must be tenant-scoped and encrypted outside the SDK.

This policy reduces accidental disclosure. Without an admitted protected-memory
implementation, Sweden does not promise zeroization, swap exclusion, or crash
dump resistance.
