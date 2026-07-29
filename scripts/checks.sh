#!/usr/bin/env sh
set -eu

cargo fmt --all --check
scripts/check_shell_syntax.sh
scripts/check_doc_links.sh
scripts/check_release_plan.sh
python3 scripts/test-rfc-sources.py
scripts/check_latest_crates.py
scripts/validate-modularity-policy.sh check
scripts/validate-security-policy.sh
scripts/validate-release-metadata.sh
scripts/release_crates.py --check
python3 scripts/test-release-crates.py

if ! cmp -s README.md crates/sweden/README.md; then
    echo "README.md and crates/sweden/README.md must remain identical" >&2
    diff -u README.md crates/sweden/README.md >&2 || true
    exit 1
fi

cargo check --workspace --all-features
cargo check -p sweden --no-default-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p sweden --no-default-features
cargo doc --workspace --all-features --no-deps
scripts/package-all.sh
