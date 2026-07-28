# Sweden Toolchain Policy

Sweden pins stable Rust `1.97.1` and supports Rust `1.90.0` through `1.97.1`.
The workspace uses edition 2024 and resolver 3.

Rules:

- `rust-toolchain.toml` pins the current stable release, never a floating
  channel.
- `workspace.package.rust-version` records the MSRV as `1.90`.
- `scripts/check_latest_tools.sh` compares the pin with the official Rust
  stable distribution manifest.
- The same script checks every pinned Cargo verification tool and
  `actions/checkout` release.
- `scripts/check_latest_crates.py` rejects external project dependencies and
  verifies exact versions if an explicit future admission changes that rule.
- Normal builds do not require nightly.
- Release gates check every installed supported stable toolchain.
- A missing supported toolchain is a release-gate failure, not a silent skip.

Compatibility command:

```bash
for toolchain in \
    1.90.0 1.91.0 1.91.1 1.92.0 1.93.0 1.93.1 \
    1.94.0 1.94.1 1.95.0 1.96.0 1.96.1 1.97.0; do
    cargo "+$toolchain" check --workspace --all-features
done
```

The full check, lint, test, documentation, and package gates run on `1.97.1`.
