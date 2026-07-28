#!/usr/bin/env sh
set -eu

release_version="$(
    python3 -c 'import tomllib; print(tomllib.load(open("release-crates.toml", "rb"))["release"]["version"])'
)"
facade_version="$(
    python3 -c 'import tomllib; print(tomllib.load(open("crates/sweden/Cargo.toml", "rb"))["package"]["version"])'
)"

test "$release_version" = "$facade_version"
grep -Fq "## ${release_version} - " CHANGELOG.md
test -s "release-notes/RELEASE_NOTES_${release_version}.md"
test -s "security/pentest/v${release_version}.md"
