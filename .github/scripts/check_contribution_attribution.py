#!/usr/bin/env python3
"""Keep branches, commits, and pull requests attribution-neutral."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ASSISTANT_NAME = re.compile(
    r"(?i)(?<![a-z0-9])(?:aider|anthropic|chatgpt|claude|codex|cursor|"
    r"deepseek|devin|gemini|github[-_ /]?copilot|gpt(?:[-_ ]?[0-9][a-z0-9.-]*)?|"
    r"llama|openai|qwen|windsurf)(?![a-z0-9])"
)
ATTRIBUTION = re.compile(
    r"(?im)(?:\b(?:ai|llm)[-_ ]?(?:assisted|authored|generated|reviewed|written)\b|"
    r"\b(?:assisted|authored|created|generated|implemented|reviewed|written)\s+"
    r"(?:by|with)\s+(?:an?\s+)?(?:ai|coding assistant|generative assistant|llm)\b|"
    r"^\s*(?:assisted-by|generated-by)\s*:)"
)


def git(*arguments: str) -> str:
    return subprocess.run(
        ["git", *arguments], check=True, capture_output=True, text=True
    ).stdout


def violations(label: str, value: str) -> list[str]:
    found = []
    if match := ASSISTANT_NAME.search(value):
        found.append(f"{label}: contains prohibited name {match.group(0)!r}")
    if match := ATTRIBUTION.search(value):
        found.append(f"{label}: contains prohibited attribution {match.group(0)!r}")
    return found


def branch_violations(label: str, branch: str) -> list[str]:
    if match := ASSISTANT_NAME.search(branch):
        return [f"{label}: branch {branch!r} contains {match.group(0)!r}"]
    return []


def range_violations(revision_range: str) -> list[str]:
    fields = git("log", "--format=%H%x00%B%x00", revision_range).split("\0")
    found = []
    for index in range(0, len(fields) - 1, 2):
        if fields[index].strip():
            found.extend(
                violations(f"commit {fields[index].strip()[:12]}", fields[index + 1])
            )
    return found


def event_violations(path: Path) -> list[str]:
    event = json.loads(path.read_text(encoding="utf-8"))
    found = []
    pull_request = event.get("pull_request")
    if isinstance(pull_request, dict):
        head = pull_request.get("head", {})
        base = pull_request.get("base", {})
        if isinstance(head.get("ref"), str):
            found.extend(branch_violations("pull request", head["ref"]))
        for field in ("title", "body"):
            if isinstance(pull_request.get(field), str):
                found.extend(violations(f"pull request {field}", pull_request[field]))
        if isinstance(base.get("sha"), str) and isinstance(head.get("sha"), str):
            found.extend(range_violations(f"{base['sha']}..{head['sha']}"))
        return found

    ref = event.get("ref")
    if isinstance(ref, str) and ref.startswith("refs/heads/"):
        found.extend(branch_violations("push", ref.removeprefix("refs/heads/")))
    for commit in event.get("commits", []):
        if isinstance(commit, dict) and isinstance(commit.get("message"), str):
            found.extend(
                violations(f"commit {str(commit.get('id', ''))[:12]}", commit["message"])
            )
    return found


def main() -> int:
    parser = argparse.ArgumentParser()
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("branches")
    message = commands.add_parser("message-file")
    message.add_argument("path", type=Path)
    event = commands.add_parser("github-event")
    event.add_argument("path", type=Path)
    arguments = parser.parse_args()

    if arguments.command == "branches":
        branch = subprocess.run(
            ["git", "symbolic-ref", "--quiet", "--short", "HEAD"],
            check=False,
            capture_output=True,
            text=True,
        ).stdout.strip()
        found = branch_violations("repository", branch) if branch else []
    elif arguments.command == "message-file":
        found = violations(
            "commit message", arguments.path.read_text(encoding="utf-8")
        )
    else:
        found = event_violations(arguments.path)
    if found:
        print("Contribution metadata must remain attribution-neutral:", file=sys.stderr)
        print("\n".join(f"  {item}" for item in found), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
