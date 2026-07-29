#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/check_latest_tools.sh
cargo deny check
cargo audit

for toolchain in \
    1.90.0 \
    1.91.0 \
    1.91.1 \
    1.92.0 \
    1.93.0 \
    1.93.1 \
    1.94.0 \
    1.94.1 \
    1.95.0 \
    1.96.0 \
    1.96.1 \
    1.97.0 \
    1.97.1; do
    rustup run "$toolchain" cargo check --workspace --all-features
done

for target in \
    x86_64-unknown-freebsd \
    x86_64-unknown-netbsd \
    aarch64-linux-android \
    aarch64-apple-ios; do
    rustup run 1.97.1 cargo check -p sweden-core --target "$target"
    rustup run 1.97.1 cargo check -p sweden --target "$target" --all-features
done

pentest_status="$(
    sed -n 's/^Status: //p' security/pentest/v0.2.0.md |
        head -n 1
)"
case "$pentest_status" in
"PASS")
    echo "v0.2.0 release gate passed with pentest PASS; commit and wait for GitHub"
    ;;
"FINDINGS OPEN")
    echo "v0.2.0 remediation gate passed; wait for the maintainer retest"
    ;;
*)
    echo "v0.2.0 implementation stop reached; run the maintainer pentest"
    ;;
esac
