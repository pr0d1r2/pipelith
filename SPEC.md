# SPEC

## §G GOAL

`pipelith` (lib, ⊥ bin) — forge-NEUTRAL core of fleet CI materializers: pipe-table registry grammar, YAML writer that cannot emit malformed YAML, generated-file set w/ drift check, neutral catalogue of CI kinds. `actlith` renders it for GitHub Actions, `lablith` for GitLab CI.

WHY: project starts private on GitLab, moves to GitHub on release ∴ CI rows move w/ it. actlith holds every shared part mixed w/ GitHub code; copy into lablith = the drift actlith exists to end (R4).

GOALS: 1 impl of grammar, writer & drift check used by both forges · 1 neutral kind catalogue (kind names, columns, arg schemas, nix systems mean same on both) · row valid vs catalogue renders on both ∴ release changes file name, ⊥ rows · extraction changes ⊥ byte of any workflow actlith renders.

NAME: pipe(line) + lith; pipe tables = the grammar it owns. fleet family `pklith` `actlith` `lablith` `microlith` `xenolith` (V8).

SOURCES: `SPEC-PROMPT.md` (decisions D1–D18), actlith @ `7ce69c6` (extraction source, D2). this spec wins where they differ.

## §N NAV

rel|path|lens
up|-|-
self|.|-
