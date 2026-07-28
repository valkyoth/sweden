# GitHub Security Settings

Repository administrators must enable:

- GitHub CodeQL analysis default setup for Rust;
- private vulnerability reporting;
- Dependabot alerts;
- dependency graph;
- secret scanning and push protection when available;
- branch protection requiring the Rust CI workflow;
- review of workflow changes by `CODEOWNERS`.

CodeQL analysis default setup is active by repository policy. Do not commit an
advanced CodeQL workflow while default setup is enabled.

Commit the implementation with its `AWAITING PENTEST` report as the exact
pentest baseline. After the pentest, commit its `PASS` outcome and any
remediation, then wait for GitHub Actions and CodeQL default setup. If either
fails, fix the issue, update the same report, commit again, and wait again. Tag
only after the maintainer confirms the latest commit is green and explicitly
asks.
