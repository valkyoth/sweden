#!/usr/bin/env bash
set -euo pipefail

test -s rfc/SOURCES
test -s rfc/SHA256SUMS

while read -r number url role; do
    [[ -n "${number:-}" ]] || continue
    [[ "$number" != \#* ]] || continue
    expected_url="https://www.rfc-editor.org/rfc/rfc${number}.txt"
    [[ "$url" == "$expected_url" && -n "${role:-}" ]] || {
        echo "invalid RFC source entry for ${number}" >&2
        exit 1
    }

    destination="rfc/rfc${number}.txt"
    expected_digest="$(
        sed -n "s/^\\([0-9a-f]\\{64\\}\\)  rfc${number}\\.txt$/\\1/p" \
            rfc/SHA256SUMS
    )"
    [[ -n "$expected_digest" ]] || {
        echo "missing checksum for RFC ${number}" >&2
        exit 1
    }
    [[ -e "$destination" ]] && continue

    temporary="${destination}.tmp"
    rm -f "$temporary"
    curl --fail --location --silent --show-error --proto '=https' --tlsv1.2 \
        --connect-timeout 10 --max-time 60 "$url" --output "$temporary"
    [[ -s "$temporary" ]]
    actual_digest="$(sha256sum "$temporary" | awk '{ print $1 }')"
    if [[ "$actual_digest" != "$expected_digest" ]]; then
        echo "checksum mismatch for RFC ${number}" >&2
        rm -f "$temporary"
        exit 1
    fi
    mv "$temporary" "$destination"
done < rfc/SOURCES

scripts/lock-rfcs.sh
