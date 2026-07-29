#!/usr/bin/env bash
set -euo pipefail

rfc_root="${RFC_ROOT:-rfc}"

test -s "$rfc_root/README.md"
test -s "$rfc_root/SOURCES"
test -s "$rfc_root/SHA256SUMS"

expected="$(
    sed -n 's/^[0-9a-f]\{64\}  \(rfc[0-9][0-9]*\.txt\)$/\1/p' \
        "$rfc_root/SHA256SUMS" |
        sort
)"
actual="$(
    find "$rfc_root" -maxdepth 1 -type f -name 'rfc*.txt' \
        -exec basename {} \; |
        sort
)"
sources="$(
    sed -n \
        's/^\([0-9][0-9]*\) https:\/\/www\.rfc-editor\.org\/rfc\/rfc[0-9][0-9]*\.txt [a-z0-9-][a-z0-9-]*$/rfc\1.txt/p' \
        "$rfc_root/SOURCES" |
        sort
)"

if [[ -z "$expected" || "$expected" != "$actual" || "$expected" != "$sources" ]]; then
    echo "RFC sources, checksums, and local files differ" >&2
    diff <(printf '%s\n' "$sources") <(printf '%s\n' "$expected") || true
    diff <(printf '%s\n' "$expected") <(printf '%s\n' "$actual") || true
    exit 1
fi

(
    cd "$rfc_root"
    sha256sum --check --strict SHA256SUMS
)
