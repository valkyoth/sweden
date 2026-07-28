#!/usr/bin/env sh
set -eu

plan="docs/RELEASE_PLAN.md"
test -s "$plan"

versions="$(sed -n 's/^## \(v[^ ]*\) -.*/\1/p' "$plan")"
test -n "$versions"

for version in $versions; do
    marker="${version} implementation stop reached. Run the maintainer pentest and update the repository report."
    if ! grep -Fq "$marker" "$plan"; then
        echo "missing pentest stop for ${version}" >&2
        exit 1
    fi
done

for heading in "Goal:" "Deliverables:" "Verification:" "Exit criteria:"; do
    count="$(grep -Fc "$heading" "$plan")"
    version_count="$(printf '%s\n' "$versions" | wc -l | tr -d ' ')"
    if [ "$count" -lt "$version_count" ]; then
        echo "release plan has fewer ${heading} sections than versions" >&2
        exit 1
    fi
done
