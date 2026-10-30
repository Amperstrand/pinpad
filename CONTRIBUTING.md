# Contributing

This repository is an archived experiment (see the status note at the end
of the README) kept as a workflow artifact, so new features are unlikely
to be accepted.

## Ground rules

- Keep changes docs-only or fix-level; the pinpad implementations are
  historical artifacts.
- Do not hand-edit `docs/screenshots/` — the `Screenshots and Tests`
  workflow regenerates them and commits the results to `main`
  automatically (`[skip ci] Update generated screenshots` commits).
- The workflow has one canonical trigger per push to `main` (plus manual
  `workflow_dispatch`); keep it that way.

## Pull requests

Small, focused PRs with a short what/why/verification body. Note that the
workflow currently runs on pushes to `main` only — PRs get no automated
checks, so keep diffs easy to review.
