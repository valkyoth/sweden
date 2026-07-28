#!/usr/bin/env sh
set -eu

workflow_dir="${GITHUB_WORKFLOW_DIR:-.github/workflows}"
ci_file="${CI_WORKFLOW_FILE:-${workflow_dir}/ci.yml}"
rust_toolchain_file="${RUST_TOOLCHAIN_FILE:-rust-toolchain.toml}"
rust_manifest_url="${RUST_STABLE_MANIFEST_URL:-https://static.rust-lang.org/dist/channel-rust-stable.toml}"

pinned_rust="$(
    sed -n 's/^channel = "\([0-9][0-9.]*\)"$/\1/p' "$rust_toolchain_file" |
        head -n 1
)"
latest_rust="$(
    curl -fsSL "$rust_manifest_url" |
        sed -n '/^\[pkg\.rust\]$/,/^\[/ {
            s/^version = "\([0-9][0-9.]*\) .*/\1/p
        }' |
        head -n 1
)"

test -n "$pinned_rust"
test -n "$latest_rust"
if [ "$pinned_rust" != "$latest_rust" ]; then
    echo "Rust is stale: pinned ${pinned_rust}, latest ${latest_rust}" >&2
    exit 1
fi

for tool in cargo-deny cargo-audit; do
    pinned="$(
        sed -n "s/.*cargo install --locked ${tool} --version \([0-9][^ ]*\).*/\1/p" \
            "$ci_file" |
            head -n 1
    )"
    latest="$(cargo info "$tool" | sed -n 's/^version: //p' | head -n 1)"
    test -n "$pinned"
    test -n "$latest"
    if [ "$pinned" != "$latest" ]; then
        echo "${tool} is stale: pinned ${pinned}, latest ${latest}" >&2
        exit 1
    fi
done

latest_checkout_tag="$(
    git ls-remote --tags --refs https://github.com/actions/checkout.git \
        'refs/tags/v*' |
        sed 's#.*refs/tags/##' |
        awk '/^v[0-9]+(\.[0-9]+)*$/' |
        sort -V |
        tail -n 1
)"
latest_checkout_sha="$(
    git ls-remote --tags --refs https://github.com/actions/checkout.git \
        "refs/tags/${latest_checkout_tag}" |
        awk '{ print $1 }'
)"
test -n "$latest_checkout_tag"
test -n "$latest_checkout_sha"

uses_count=0
for workflow in "$workflow_dir"/*.yml "$workflow_dir"/*.yaml; do
    [ -f "$workflow" ] || continue
    while IFS= read -r line; do
        uses_count=$((uses_count + 1))
        pin="$(printf '%s\n' "$line" | sed -n 's/.*actions\/checkout@\([0-9a-f]\{40\}\).*/\1/p')"
        tag="$(printf '%s\n' "$line" | sed -n 's/.*# \(v[0-9][0-9.]*\).*/\1/p')"
        if [ "$pin" != "$latest_checkout_sha" ] || [ "$tag" != "$latest_checkout_tag" ]; then
            echo "stale actions/checkout pin in ${workflow}" >&2
            exit 1
        fi
    done <<EOF
$(sed -n '/uses: actions\/checkout@/p' "$workflow")
EOF
done

test "$uses_count" -gt 0
echo "Rust, Cargo verification tools, and actions/checkout pins are current"
