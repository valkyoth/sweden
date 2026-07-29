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
- credential binding must contain no secret bytes: only an opaque provider
  token, non-serializable provider-session epoch, quota/access partitions,
  generation, and expiry;
- binding tokens must be non-`Copy`, non-`Clone`, non-serializable, and
  consumed by one `SecretLease` materialization or terminal cached return;
  restart, epoch mismatch, generation reset/wrap, or replay forces reselection;
- credential-pool quota identity must remain stable across shared-pool
  rotation/aliases, while access identity remains stable only when entitlement
  is unchanged; neither may derive from raw secret bytes;
- quota/access partition IDs and binding tokens must not be logged or accepted
  from callers even though they contain no secret bytes;
- after cache miss/quota admission/final policy check, a one-use `SecretLease`
  must match the binding, be injected immediately, and never be cached or
  retained for retry;
- deadline expiry or cancellation during materialization/injection must drop
  the secret lease, release uncommitted authority at most once, and never
  transfer secret material to a cache-fill waiter;
- cache lookup/fill waits carry no secret lease; before returning protected
  cached data, the provider must revalidate non-secret access and the same
  `AccessPartitionId`, otherwise the candidate is discarded or access denied;
- provider access rebinds must consume one finite non-secret restart ledger;
  exhaustion discards the candidate and cannot recover an earlier binding,
  partition, token, or cached value within that execution;
- fixture and replay tooling must fail closed on protected headers;
- every secret-bearing error and debug path needs snapshot tests;
- hosted credentials must be tenant-scoped and encrypted outside the SDK.

This policy reduces accidental disclosure. Without an admitted protected-memory
implementation, Sweden does not promise zeroization, swap exclusion, or crash
dump resistance.
