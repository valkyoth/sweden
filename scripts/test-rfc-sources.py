#!/usr/bin/env python3
"""Regression tests for the locked Sweden RFC source baseline."""

from __future__ import annotations

import hashlib
import os
import re
import shutil
import stat
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RFC = ROOT / "rfc"
SOURCE_PATTERN = re.compile(
    r"^(\d+) https://www\.rfc-editor\.org/rfc/rfc(\d+)\.txt ([a-z0-9-]+)$"
)
CHECKSUM_PATTERN = re.compile(r"^([0-9a-f]{64})  (rfc(\d+)\.txt)$")
REQUIRED = {2119, 3339, 3986, 6585, 8174, 8259, 9110, 9111, 9112}


def parse_sources() -> dict[int, str]:
    sources: dict[int, str] = {}
    for line in (RFC / "SOURCES").read_text(encoding="ascii").splitlines():
        if not line or line.startswith("#"):
            continue
        match = SOURCE_PATTERN.fullmatch(line)
        assert match is not None, f"invalid source line: {line!r}"
        number, repeated, role = match.groups()
        assert number == repeated
        key = int(number)
        assert key not in sources
        sources[key] = role
    return sources


def parse_checksums() -> dict[int, str]:
    checksums: dict[int, str] = {}
    for line in (RFC / "SHA256SUMS").read_text(encoding="ascii").splitlines():
        match = CHECKSUM_PATTERN.fullmatch(line)
        assert match is not None, f"invalid checksum line: {line!r}"
        digest, filename, number = match.groups()
        path = RFC / filename
        assert path.is_file() and path.stat().st_size > 0
        assert hashlib.sha256(path.read_bytes()).hexdigest() == digest
        key = int(number)
        assert key not in checksums
        checksums[key] = digest
    return checksums


def main() -> None:
    sources = parse_sources()
    checksums = parse_checksums()
    assert set(sources) == REQUIRED
    assert set(checksums) == REQUIRED
    subprocess.run(["scripts/verify-rfcs.sh"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory() as directory:
        checkout = Path(directory) / "rfc"
        shutil.copytree(RFC, checkout)
        protected = [checkout / "SHA256SUMS", *checkout.glob("rfc*.txt")]
        for path in protected:
            path.chmod(path.stat().st_mode | stat.S_IWUSR)

        environment = os.environ.copy()
        environment["RFC_ROOT"] = str(checkout)
        subprocess.run(
            ["scripts/verify-rfcs.sh"],
            cwd=ROOT,
            env=environment,
            check=True,
        )

        first_rfc = checkout / f"rfc{min(REQUIRED)}.txt"
        first_rfc.write_bytes(first_rfc.read_bytes() + b"\n")
        tampered = subprocess.run(
            ["scripts/verify-rfcs.sh"],
            cwd=ROOT,
            env=environment,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        assert tampered.returncode != 0

    print(f"RFC source baseline tests passed ({len(REQUIRED)} documents)")


if __name__ == "__main__":
    main()
