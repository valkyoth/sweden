#!/usr/bin/env python3
"""Enforce Sweden's first-party-only Cargo dependency policy."""

from __future__ import annotations

import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXACT = re.compile(r"^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$")


def dependency_tables(value: object, name: str = ""):
    if not isinstance(value, dict):
        return
    if name in {"dependencies", "dev-dependencies", "build-dependencies"}:
        yield value
        return
    for child_name, child in value.items():
        yield from dependency_tables(child, child_name)


def main() -> int:
    checked = 0
    for manifest in sorted(ROOT.rglob("Cargo.toml")):
        if any(part in {".git", "target"} for part in manifest.parts):
            continue
        with manifest.open("rb") as source:
            document = tomllib.load(source)
        for table in dependency_tables(document):
            for name, specification in table.items():
                checked += 1
                if not name.startswith("sweden"):
                    raise SystemExit(
                        f"{manifest}: third-party dependency is forbidden: {name}"
                    )
                if not isinstance(specification, dict):
                    raise SystemExit(
                        f"{manifest}: {name} must use workspace/path metadata"
                    )
                if specification.get("workspace") is True:
                    continue
                path = specification.get("path")
                version = specification.get("version")
                if not isinstance(path, str) or not isinstance(version, str):
                    raise SystemExit(
                        f"{manifest}: {name} needs path and exact version"
                    )
                if EXACT.fullmatch(version) is None:
                    raise SystemExit(
                        f"{manifest}: {name} version is not exact: {version}"
                    )
    print(f"all {checked} Cargo dependencies are first-party Sweden crates")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
