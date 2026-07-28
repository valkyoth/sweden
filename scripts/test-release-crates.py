#!/usr/bin/env python3
"""Regression tests for Sweden's per-crate release helper."""

from __future__ import annotations

import importlib.util
from pathlib import Path
from types import SimpleNamespace


ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "release_crates.py"


def load_release_crates():
    spec = importlib.util.spec_from_file_location("release_crates", SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load release_crates.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


release_crates = load_release_crates()


def entry(
    previous: str,
    version: str,
    change: str,
    publish: bool,
) -> dict:
    return {
        "previous_version": previous,
        "version": version,
        "change": change,
        "publish": publish,
        "reason": "test",
    }


def package(name: str, version: str, deps: tuple[str, ...] = ()) -> dict:
    return {
        "name": name,
        "version": version,
        "dependencies": [{"name": dependency} for dependency in deps],
    }


def base_plan() -> dict:
    return {
        "version": "0.4.0",
        "crates": {
            "sweden-core": entry("0.3.0", "0.3.0", "unchanged", False),
            "sweden": entry("0.3.0", "0.4.0", "code", True),
        },
    }


def base_packages() -> dict[str, dict]:
    return {
        "sweden-core": package("sweden-core", "0.3.0"),
        "sweden": package("sweden", "0.4.0", ("sweden-core",)),
    }


def assert_fails(expected: str, function, *args) -> None:
    try:
        function(*args)
    except RuntimeError as exc:
        if expected not in str(exc):
            raise AssertionError(f"expected {expected!r} in {exc!r}") from exc
        return
    raise AssertionError("expected failure")


def test_current_shape_accepts_independent_core() -> None:
    release_crates.verify_publish_order(base_packages(), base_plan())


def test_facade_must_equal_release_tag() -> None:
    facade = entry("0.3.0", "0.3.1", "code", True)
    assert_fails(
        "must equal release tag 0.4.0",
        release_crates.validate_plan_entry,
        "sweden",
        facade,
        "0.4.0",
    )


def test_facade_must_publish_for_every_tag() -> None:
    facade = entry("0.3.0", "0.4.0", "unchanged", False)
    assert_fails(
        "must be published for every release tag",
        release_crates.validate_plan_entry,
        "sweden",
        facade,
        "0.4.0",
    )


def test_support_code_uses_next_independent_minor() -> None:
    core = entry("0.7.2", "0.8.0", "code", True)
    release_crates.validate_plan_entry("sweden-core", core, "0.19.0")


def test_support_code_rejects_facade_release_counter() -> None:
    core = entry("0.7.2", "0.19.0", "code", True)
    assert_fails(
        "independent version must be 0.8.0",
        release_crates.validate_plan_entry,
        "sweden-core",
        core,
        "0.19.0",
    )


def test_support_bugfix_uses_next_patch() -> None:
    core = entry("0.7.2", "0.7.3", "bugfix", True)
    release_crates.validate_plan_entry("sweden-core", core, "0.19.0")


def test_dependency_change_stays_on_minor_line() -> None:
    core = entry("0.7.2", "0.8.0", "dependency", True)
    assert_fails(
        "dependency-only bumps",
        release_crates.validate_plan_entry,
        "sweden-core",
        core,
        "0.19.0",
    )


def test_unchanged_support_crate_is_not_published() -> None:
    core = entry("0.7.2", "0.7.2", "unchanged", True)
    assert_fails(
        "unchanged but publish is true",
        release_crates.validate_plan_entry,
        "sweden-core",
        core,
        "0.19.0",
    )


def test_publish_plan_skips_unchanged_support_crate() -> None:
    assert release_crates.publish_plan(base_plan()) == ("sweden",)


def test_v1_converges_every_crate() -> None:
    core = entry("0.12.3", "1.0.0", "metadata", True)
    release_crates.validate_plan_entry("sweden-core", core, "1.0.0")


def test_v1_rejects_unchanged_support_crate() -> None:
    core = entry("0.12.3", "0.12.3", "unchanged", False)
    assert_fails(
        "must converge to 1.0.0",
        release_crates.validate_plan_entry,
        "sweden-core",
        core,
        "1.0.0",
    )


def test_dependency_order_is_enforced() -> None:
    packages = base_packages()
    packages["sweden-core"]["dependencies"] = [{"name": "sweden"}]
    assert_fails(
        "appears later in PUBLISH_ORDER",
        release_crates.verify_publish_order,
        packages,
        base_plan(),
    )


def test_milestone_gate_name_is_supported() -> None:
    gate = release_crates.release_gate("0.1.0")
    assert gate is not None
    assert gate.name == "release_0_1_gate.sh"


def test_post_tag_preflight_passes_guarded_context() -> None:
    calls = []
    original_run = release_crates.run
    release_crates.run = lambda command, **kwargs: calls.append((command, kwargs))
    try:
        args = SimpleNamespace(version="0.1.0", skip_checks=False, dry_run=False)
        release_crates.run_preflight(args, release_tag_at_head=True)
    finally:
        release_crates.run = original_run
    assert calls[0] == (
        ["scripts/release_0_1_gate.sh"],
        {
            "dry_run": False,
            "extra_env": {"SWEDEN_RELEASE_PUBLISH_TAG": "v0.1.0"},
        },
    )


def run_tests() -> None:
    tests = (
        test_current_shape_accepts_independent_core,
        test_facade_must_equal_release_tag,
        test_facade_must_publish_for_every_tag,
        test_support_code_uses_next_independent_minor,
        test_support_code_rejects_facade_release_counter,
        test_support_bugfix_uses_next_patch,
        test_dependency_change_stays_on_minor_line,
        test_unchanged_support_crate_is_not_published,
        test_publish_plan_skips_unchanged_support_crate,
        test_v1_converges_every_crate,
        test_v1_rejects_unchanged_support_crate,
        test_dependency_order_is_enforced,
        test_milestone_gate_name_is_supported,
        test_post_tag_preflight_passes_guarded_context,
    )
    for test in tests:
        test()


if __name__ == "__main__":
    run_tests()
