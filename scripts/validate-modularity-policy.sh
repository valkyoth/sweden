#!/usr/bin/env sh
set -eu

mode="${1:-check}"
if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-modularity-policy.sh check" >&2
    exit 2
fi

violations="$(
    find crates -type f -name '*.rs' -exec wc -l {} \; |
        awk '$1 > 500 { print }'
)"
if [ -n "$violations" ]; then
    echo "Rust files exceed 500 lines:" >&2
    echo "$violations" >&2
    exit 1
fi

for crate in \
    sweden \
    sweden-core; do
    test -f "crates/${crate}/Cargo.toml"
    test -f "crates/${crate}/README.md"
    test -f "crates/${crate}/src/lib.rs"
done
