# Sweden Release Runbook

1. Finish exactly one version's bounded deliverables.
2. Update tests, documentation, changelog, and version release notes.
3. Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and the matching
   version gate.
4. Commit the implementation and its `AWAITING PENTEST` report as the exact
   pentest baseline.
5. Stop and ask the maintainer to pentest that commit.
6. Update `security/pentest/vX.Y.Z.md` with the maintainer's result.
7. If there are findings, fix them, update the same report, and rerun the gates.
   Repeat until the report says `Status: PASS`.
8. If there are no findings, record that clearly and set `Status: PASS`.
9. Commit the pentest outcome, remediation when needed, release metadata, and
   the report.
10. Wait for GitHub Actions and CodeQL default setup.
11. If GitHub fails, fix the issue, update the same report, commit again, and
    wait for GitHub again.
12. When GitHub is green, wait for the maintainer to explicitly request tagging.
13. Only then create the requested `vX.Y.Z` tag at the approved commit.
14. Run `scripts/release_crates.py --version X.Y.Z --require-tag`; it publishes
    only crates marked for that release, in dependency order.

A clean pentest may produce a report-only outcome commit. There is no automatic
tag.

The `sweden` facade always equals and publishes with the tag. Unchanged
subcrates retain their existing versions and are skipped. At `v1.0.0`, all
workspace crates converge to `1.0.0` and publish.
