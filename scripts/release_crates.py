#!/usr/bin/env python3
"""Publish Sweden workspace crates in crates.io dependency order."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - release host guard.
    print("Python 3.11+ is required because this script uses tomllib.", file=sys.stderr)
    raise


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PLAN = ROOT / "release-crates.toml"
CHANGE_KINDS = ("code", "bugfix", "dependency", "metadata", "unchanged")
FACADE = "sweden"

# Add crates here in dependency order when their implementation begins.
PUBLISH_ORDER = (
    "sweden-core",
    "sweden",
)


def run(
    command: list[str],
    *,
    dry_run: bool,
    extra_env: dict[str, str] | None = None,
) -> None:
    print(f"+ {' '.join(command)}", flush=True)
    if dry_run:
        return
    environment = None
    if extra_env is not None:
        environment = os.environ.copy()
        environment.update(extra_env)
    subprocess.run(command, cwd=ROOT, check=True, env=environment)


def capture(command: list[str]) -> str:
    return subprocess.check_output(command, cwd=ROOT, text=True).strip()


def try_capture(command: list[str]) -> str | None:
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
    )
    if result.returncode != 0:
        return None
    return result.stdout.strip()


def load_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def parse_version(version: str) -> tuple[int, int, int]:
    parts = version.split(".")
    if len(parts) != 3:
        raise RuntimeError(f"version must be MAJOR.MINOR.PATCH: {version}")
    try:
        parsed = tuple(int(part) for part in parts)
    except ValueError as exc:
        raise RuntimeError(f"version must be numeric: {version}") from exc
    if any(part < 0 for part in parsed):
        raise RuntimeError(f"version components must be non-negative: {version}")
    return parsed  # type: ignore[return-value]


def cargo_metadata() -> dict:
    raw = capture(["cargo", "metadata", "--format-version", "1", "--no-deps"])
    return json.loads(raw)


def workspace_packages(metadata: dict) -> dict[str, dict]:
    workspace_ids = set(metadata["workspace_members"])
    return {
        package["name"]: package
        for package in metadata["packages"]
        if package["id"] in workspace_ids
    }


def validate_plan_entry(
    package_name: str,
    entry: dict,
    release: str,
) -> None:
    previous = entry.get("previous_version")
    version = entry.get("version")
    change = entry.get("change")
    publish = entry.get("publish")
    reason = entry.get("reason")
    strings = (previous, version, change, reason)
    if not all(isinstance(value, str) for value in strings):
        raise RuntimeError(f"{package_name} has incomplete release plan metadata")
    if change not in CHANGE_KINDS:
        raise RuntimeError(f"{package_name} has invalid change kind {change!r}")
    if not isinstance(publish, bool):
        raise RuntimeError(f"{package_name} publish must be true or false")

    previous_parts = parse_version(previous)
    planned_parts = parse_version(version)
    release_parts = parse_version(release)

    if package_name == FACADE:
        if planned_parts != release_parts:
            raise RuntimeError(f"{FACADE} version must equal release tag {release}")
        if change == "unchanged" or not publish:
            raise RuntimeError(f"{FACADE} must be published for every release tag")
        return

    if release_parts == (1, 0, 0):
        if planned_parts != release_parts or not publish or change == "unchanged":
            raise RuntimeError(
                f"{package_name} must converge to 1.0.0 and publish for v1.0.0"
            )
        return

    if change == "code":
        expected = (previous_parts[0], previous_parts[1] + 1, 0)
        if planned_parts != expected:
            expected_text = ".".join(str(part) for part in expected)
            raise RuntimeError(
                f"{package_name} has code changes, so its independent "
                f"version must be {expected_text}"
            )
        if not publish:
            raise RuntimeError(f"{package_name} has code changes but publish is false")
    elif change == "bugfix":
        expected = (previous_parts[0], previous_parts[1], previous_parts[2] + 1)
        if planned_parts != expected:
            expected_text = ".".join(str(part) for part in expected)
            raise RuntimeError(
                f"{package_name} has an API-compatible bug fix, so its "
                f"independent version must be {expected_text}"
            )
        if not publish:
            raise RuntimeError(f"{package_name} has a bug fix but publish is false")
    elif change == "metadata":
        if planned_parts != release_parts:
            raise RuntimeError(
                f"{package_name} has metadata changes, so version must be {release}"
            )
        if not publish:
            raise RuntimeError(
                f"{package_name} has metadata changes but publish is false"
            )
    elif change == "dependency":
        same_line = planned_parts[:2] == previous_parts[:2]
        patch_bump = planned_parts[2] > previous_parts[2]
        if not same_line or not patch_bump:
            raise RuntimeError(
                f"{package_name} dependency-only bumps must stay on the existing "
                "minor line and increase only the patch number"
            )
        if not publish:
            raise RuntimeError(
                f"{package_name} has dependency-only changes but publish is false"
            )
    else:
        if planned_parts != previous_parts:
            raise RuntimeError(
                f"{package_name} is unchanged but version differs from "
                "previous_version"
            )
        if publish:
            raise RuntimeError(f"{package_name} is unchanged but publish is true")


def release_plan(plan_path: Path) -> dict:
    plan = load_toml(plan_path)
    release = plan.get("release", {})
    crates = plan.get("crates", {})
    version = release.get("version")
    if not isinstance(version, str):
        raise RuntimeError("release-crates.toml is missing [release].version")
    if release.get("policy") != "independent":
        raise RuntimeError("release-crates.toml policy must be 'independent'")
    if set(crates) != set(PUBLISH_ORDER):
        raise RuntimeError(
            "release-crates.toml crates are not in sync with PUBLISH_ORDER: "
            f"expected {tuple(sorted(PUBLISH_ORDER))}, actual {tuple(sorted(crates))}"
        )
    parse_version(version)
    for package_name, entry in crates.items():
        validate_plan_entry(package_name, entry, version)
    return {"version": version, "crates": crates}


def require_clean_tree(*, allow_dirty: bool) -> None:
    if allow_dirty:
        return
    status = capture(["git", "status", "--porcelain"])
    if status:
        print("Refusing to publish from a dirty worktree:", file=sys.stderr)
        print(status, file=sys.stderr)
        print("Commit or stash changes, or pass --allow-dirty.", file=sys.stderr)
        sys.exit(1)


def verify_publish_order(packages: dict[str, dict], plan: dict) -> None:
    expected_names = tuple(sorted(PUBLISH_ORDER))
    actual_names = tuple(sorted(packages))
    if actual_names != expected_names:
        raise RuntimeError(
            "release_crates.py PUBLISH_ORDER is not in sync with workspace "
            f"packages: expected {expected_names}, actual {actual_names}"
        )

    seen: set[str] = set()
    for package_name in PUBLISH_ORDER:
        package = packages[package_name]
        planned_version = plan["crates"][package_name]["version"]
        if package["version"] != planned_version:
            raise RuntimeError(
                f"{package_name} is version {package['version']}, "
                f"expected {planned_version}"
            )
        for dependency in package["dependencies"]:
            dependency_name = dependency["name"]
            if dependency_name in packages and dependency_name not in seen:
                raise RuntimeError(
                    f"{package_name} depends on {dependency_name}, but "
                    f"{dependency_name} appears later in PUBLISH_ORDER"
                )
        seen.add(package_name)


def check_release_tag(version: str, *, require_tag: bool) -> bool:
    tag = f"v{version}"
    head = try_capture(["git", "rev-parse", "HEAD"])
    tagged_commit = try_capture(["git", "rev-list", "-n", "1", tag])
    if head is None or tagged_commit is None:
        message = f"release tag {tag!r} was not found"
        if require_tag:
            print(f"Refusing to publish: {message}.", file=sys.stderr)
            sys.exit(1)
        print(f"Warning: {message}.", file=sys.stderr)
        return False
    if head != tagged_commit:
        message = f"HEAD is not tagged as {tag} (HEAD {head}, {tag} {tagged_commit})"
        if require_tag:
            print(f"Refusing to publish: {message}.", file=sys.stderr)
            sys.exit(1)
        print(f"Warning: {message}.", file=sys.stderr)
        return False
    print(f"Release tag {tag} points at HEAD.")
    return True


def release_gate(version: str) -> Path | None:
    major, minor, patch = parse_version(version)
    names = [f"release_{major}_{minor}_{patch}_gate.sh"]
    if patch == 0:
        names.append(f"release_{major}_{minor}_gate.sh")
    for name in names:
        candidate = ROOT / "scripts" / name
        if candidate.exists():
            return candidate
    return None


def run_preflight(args: argparse.Namespace, *, release_tag_at_head: bool) -> None:
    if args.skip_checks:
        print("Skipping preflight checks by request.")
        return
    gate = release_gate(args.version)
    gate_env = None
    if release_tag_at_head:
        gate_env = {"SWEDEN_RELEASE_PUBLISH_TAG": f"v{args.version}"}
    if gate is None:
        run(["scripts/checks.sh"], dry_run=args.dry_run)
    else:
        run(
            [str(gate.relative_to(ROOT))],
            dry_run=args.dry_run,
            extra_env=gate_env,
        )
    run(["cargo", "deny", "check"], dry_run=args.dry_run)
    run(["cargo", "audit"], dry_run=args.dry_run)


def publish_plan(plan: dict) -> tuple[str, ...]:
    return tuple(
        package for package in PUBLISH_ORDER if plan["crates"][package]["publish"]
    )


def selected_steps(start_at: str, steps: tuple[str, ...]) -> tuple[str, ...]:
    if not steps:
        return ()
    try:
        index = steps.index(start_at)
    except ValueError as exc:
        raise RuntimeError(f"unknown package for --start-at: {start_at}") from exc
    return steps[index:]


def wait_for_index(package: str, version: str, *, dry_run: bool) -> None:
    print()
    print(f"Published {package} {version}.")
    print(f"Wait until crates.io shows: https://crates.io/crates/{package}/{version}")
    print("Then press Enter to continue with dependent crates.")
    if dry_run:
        print("[dry-run] skipping wait")
        return
    input()
    time.sleep(5)


def publish(package: str, args: argparse.Namespace) -> None:
    command = ["cargo", "publish", "-p", package]
    if args.allow_dirty:
        command.append("--allow-dirty")
    if args.no_verify:
        command.append("--no-verify")
    run(command, dry_run=args.dry_run)


def confirm_no_verify(args: argparse.Namespace) -> int:
    if not args.no_verify or args.dry_run:
        return 0
    print(
        "\nWARNING: --no-verify bypasses cargo package verification.\n"
        "Type 'no-verify confirmed' to continue:",
        file=sys.stderr,
    )
    if input().strip() != "no-verify confirmed":
        print("Aborted.", file=sys.stderr)
        return 1
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Publish Sweden workspace crates in crates.io order."
    )
    parser.add_argument("--version", default=None)
    parser.add_argument("--plan", default=str(DEFAULT_PLAN))
    parser.add_argument("--start-at", default=None, choices=PUBLISH_ORDER)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--allow-dirty", action="store_true")
    parser.add_argument("--skip-checks", action="store_true")
    parser.add_argument("--no-verify", action="store_true")
    parser.add_argument("--require-tag", action="store_true")
    parser.add_argument("--yes", action="store_true")
    args = parser.parse_args()

    raw_plan_path = Path(args.plan)
    plan_path = (
        raw_plan_path
        if raw_plan_path.is_absolute()
        else (ROOT / raw_plan_path).resolve()
    )
    plan = release_plan(plan_path)
    if args.version is None:
        args.version = plan["version"]
    elif args.version != plan["version"]:
        print(
            f"Refusing to publish: --version {args.version} does not match "
            f"{plan_path.name} release {plan['version']}.",
            file=sys.stderr,
        )
        return 1

    packages = workspace_packages(cargo_metadata())
    verify_publish_order(packages, plan)
    if args.check:
        print("release_crates.py publish order is up to date.")
        print(f"release_crates.py release plan is {args.version}.")
        return 0

    require_clean_tree(allow_dirty=args.allow_dirty or args.dry_run)
    release_tag_at_head = check_release_tag(
        args.version, require_tag=args.require_tag
    )
    planned_publish = publish_plan(plan)
    start_at = args.start_at or (planned_publish[0] if planned_publish else "")
    steps = selected_steps(start_at, planned_publish)

    print(f"Workspace root: {ROOT}")
    print(f"Release version: {args.version}")
    print("Publish sequence:")
    for package in steps:
        entry = plan["crates"][package]
        print(f"  - {package} {entry['version']} ({entry['change']})")
    if not steps:
        print("  - no crates selected for publishing")
    print()

    if not args.yes:
        answer = input("Type the release version to start publishing: ").strip()
        if answer != args.version:
            print("Version confirmation did not match; aborting.", file=sys.stderr)
            return 1
    if confirm_no_verify(args) != 0:
        return 1

    run_preflight(args, release_tag_at_head=release_tag_at_head)
    for index, package in enumerate(steps):
        publish(package, args)
        version = plan["crates"][package]["version"]
        if index != len(steps) - 1:
            wait_for_index(package, version, dry_run=args.dry_run)

    print()
    print("Release publish sequence completed.")
    print(f"Recommended follow-up: cargo info {FACADE}@{args.version}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
