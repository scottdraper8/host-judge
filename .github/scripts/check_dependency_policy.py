#!/usr/bin/env python3
"""Validate Host Judge's immutable Rust dependency and workflow policy."""

from __future__ import annotations

import re
import subprocess
import sys
import tomllib
from pathlib import Path

PINNED_TOOLCHAIN = re.compile(r"[0-9]+\.[0-9]+\.[0-9]+")
PINNED_REVISION = re.compile(r"[0-9a-f]{40}")
ACTION_REFERENCE = re.compile(r"^\s*uses:\s*[^\s@]+@([^\s#]+)", re.MULTILINE)
PRECOMMIT_REPOSITORY = re.compile(
    r"^\s*-\s+repo:\s*(?!local\s*$)\S+.*?^\s+rev:\s*([^\s#]+)",
    re.MULTILINE | re.DOTALL,
)


def dependency_failures(root: Path) -> list[str]:
    with (root / "Cargo.toml").open("rb") as stream:
        manifest = tomllib.load(stream)
    failures = []
    for section in ("dependencies", "dev-dependencies", "build-dependencies"):
        for name, specification in manifest.get(section, {}).items():
            version = (
                specification
                if isinstance(specification, str)
                else specification.get("version")
                if isinstance(specification, dict)
                else None
            )
            if isinstance(specification, dict) and "git" in specification:
                failures.append(f"Cargo.toml: {section}.{name} must not use Git")
            if not isinstance(version, str) or not version.startswith("="):
                failures.append(
                    f"Cargo.toml: {section}.{name} must use an exact '=VERSION' pin"
                )
    return failures


def immutable_reference_failures(root: Path) -> list[str]:
    failures = []
    for workflow in sorted((root / ".github" / "workflows").glob("*.y*ml")):
        for revision in ACTION_REFERENCE.findall(workflow.read_text(encoding="utf-8")):
            if PINNED_REVISION.fullmatch(revision) is None:
                failures.append(f"{workflow}: action revision {revision!r} is mutable")
    precommit = root / ".pre-commit-config.yaml"
    for revision in PRECOMMIT_REPOSITORY.findall(
        precommit.read_text(encoding="utf-8")
    ):
        if PINNED_REVISION.fullmatch(revision) is None:
            failures.append(f"{precommit}: hook revision {revision!r} is mutable")
    return failures


def workflow_hardening_failures(root: Path) -> list[str]:
    failures = []
    for workflow in sorted((root / ".github" / "workflows").glob("*.y*ml")):
        text = workflow.read_text(encoding="utf-8")
        jobs = len(re.findall(r"^    runs-on:\s*", text, re.MULTILINE))
        hardened = text.count("uses: step-security/harden-runner@")
        sudo_disabled = text.count("disable-sudo: true")
        if hardened != jobs:
            failures.append(f"{workflow}: every job must use Harden Runner")
        if sudo_disabled != jobs:
            failures.append(f"{workflow}: every job must disable sudo")
        if re.search(r"^permissions:\s*(?:read-all|write-all)\s*$", text, re.MULTILINE):
            failures.append(f"{workflow}: broad workflow permissions are prohibited")
        for position in (
            match.end()
            for match in re.finditer(r"^    runs-on:.*$", text, re.MULTILINE)
        ):
            remaining = text[position:]
            steps = list(re.finditer(r"^      - name:.*$", remaining, re.MULTILINE))
            if not steps:
                failures.append(f"{workflow}: job has no steps")
                continue
            end = steps[1].start() if len(steps) > 1 else len(remaining)
            first_block = remaining[steps[0].start() : end]
            if "uses: step-security/harden-runner@" not in first_block:
                failures.append(f"{workflow}: Harden Runner must be the first step")
    return failures


def policy_failures(root: Path) -> list[str]:
    failures = []
    with (root / "rust-toolchain.toml").open("rb") as stream:
        toolchain = tomllib.load(stream).get("toolchain", {})
    if PINNED_TOOLCHAIN.fullmatch(str(toolchain.get("channel", ""))) is None:
        failures.append("rust-toolchain.toml must pin a full numeric version")
    if not {"rustfmt", "clippy"}.issubset(set(toolchain.get("components", []))):
        failures.append("rust-toolchain.toml must install rustfmt and clippy")

    dependabot = (root / ".github" / "dependabot.yml").read_text(encoding="utf-8")
    for ecosystem in ("cargo", "github-actions"):
        if f"package-ecosystem: {ecosystem}" not in dependabot:
            failures.append(f"Dependabot is missing {ecosystem}")
    if dependabot.count("open-pull-requests-limit: 0") != 2:
        failures.append("routine Dependabot pull requests must be disabled")
    if "versioning-strategy: increase" not in dependabot:
        failures.append("Cargo security updates must preserve exact pins")
    return failures


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    failures = (
        dependency_failures(root)
        + immutable_reference_failures(root)
        + workflow_hardening_failures(root)
        + policy_failures(root)
    )
    if not (root / "Cargo.lock").is_file():
        failures.append("Cargo.lock is required")
    metadata = subprocess.run(
        ["cargo", "metadata", "--locked", "--no-deps", "--format-version=1"],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    if metadata.returncode:
        failures.append(f"Cargo lock validation failed:\n{metadata.stderr.strip()}")
    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
