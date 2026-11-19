# Repository operating rules

## General

- Establish behavior from current source and reproducible runtime evidence.
  Correctness and completeness outrank speed or convenience.
- Continue within the authorized scope until the acceptance criteria are met or
  a concrete blocker prevents safe progress.
- Prefer the smallest correct design. Remove obsolete code instead of retaining
  compatibility paths that the current design does not require.
- Keep documentation current-state and forward-facing. Do not use maintained
  documentation as an investigation log or changelog.
- Preserve unrelated dirty work.
- Keep contribution metadata project-focused and attribution-neutral. Branch
  names, commit messages and trailers, pull-request titles and descriptions,
  and equivalent metadata must not name, advertise, credit, or attribute work
  to a generative assistant or model.

## Quality and security

- Do not skip repository hooks or suppress lint, audit, or security findings.
- Keep direct Cargo dependencies exact-pinned and commit `Cargo.lock`.
- Use `--locked` for Cargo commands that resolve or build dependencies.
- Pin GitHub Actions and remote pre-commit hooks to full commit hashes.
- Keep workflows least-privileged and start every job with Harden Runner.
- Treat downloaded host lists and generated workflow artifacts as untrusted
  input. Preserve the read-only analysis and constrained publisher boundary.
- Preserve host-list uniqueness, whitelist, freshness, and removal-candidate
  semantics when changing the analyzer.
