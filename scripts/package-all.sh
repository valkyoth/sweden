#!/usr/bin/env sh
set -eu

core_patch='patch.crates-io.sweden-core.path="crates/sweden-core"'

check_package_files() {
    package="$1"
    shift
    listing="$(cargo package -p "$package" --allow-dirty --no-verify --list "$@")"
    for required in Cargo.toml LICENSE-APACHE LICENSE-MIT README.md src/lib.rs; do
        if ! printf '%s\n' "$listing" | grep -Fxq "$required"; then
            echo "${package} package is missing ${required}" >&2
            exit 1
        fi
    done
}

cargo package -p sweden-core --allow-dirty --no-verify
check_package_files sweden-core

cargo package -p sweden --allow-dirty --no-verify \
    --config "$core_patch"
check_package_files sweden --config "$core_patch"
