#!/usr/bin/env sh
set -eu

for source in crates/*/src/lib.rs; do
    grep -Fq '#![forbid(unsafe_code)]' "$source"
done

for crate_dir in crates/*; do
    cmp -s LICENSE-MIT "${crate_dir}/LICENSE-MIT"
    cmp -s LICENSE-APACHE "${crate_dir}/LICENSE-APACHE"
done

for crate in \
    sweden \
    sweden-core; do
    grep -Fq '#![no_std]' "crates/${crate}/src/lib.rs"
done

grep -Fq 'unknown-git = "deny"' deny.toml
grep -Fq 'unknown-registry = "deny"' deny.toml
grep -Fq 'wildcards = "deny"' deny.toml
grep -Fq 'panic = "abort"' Cargo.toml
grep -Fq 'CodeQL default setup' SECURITY.md
grep -Fq 'CodeQL analysis default setup is active' docs/github-security-settings.md
test -f docs/secret-handling-policy.md
test -s security/pentest/README.md
test -s security/pentest/TEMPLATE.md
version="$(
    python3 -c 'import tomllib; print(tomllib.load(open("release-crates.toml", "rb"))["release"]["version"])'
)"
report="security/pentest/v${version}.md"
test -s "$report"
grep -Eq '^Status: (AWAITING PENTEST|FINDINGS OPEN|PASS)$' \
    "$report"

if rg -n '^\s*(unsafe\s*\{|unsafe\s+fn|unsafe\s+impl)' crates; then
    echo "unsafe Rust is forbidden" >&2
    exit 1
fi
