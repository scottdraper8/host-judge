#!/usr/bin/env python3
"""Decide whether a pull request needs the full validation and security checks."""

import subprocess
import sys


def checks_needed(paths: list[bytes]) -> bool:
    harmless_files = {b"README.md", b"LICENSE", b".gitignore"}
    local_directories = (b".unstale/", b".claude/", b".agents/", b".opencode/")
    return not paths or any(
        path not in harmless_files and not path.startswith(local_directories)
        for path in paths
    )


def main() -> None:
    if len(sys.argv) != 3:
        raise SystemExit("usage: checks_needed.py BASE HEAD")
    result = subprocess.run(
        ["git", "diff", "--no-renames", "--name-only", "-z", sys.argv[1], sys.argv[2], "--"],
        check=True,
        stdout=subprocess.PIPE,
    )
    paths = result.stdout.split(b"\0")
    if paths[-1] == b"":
        paths.pop()
    print(str(checks_needed(paths)).lower())


if __name__ == "__main__":
    main()
