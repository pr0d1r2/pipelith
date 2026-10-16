# Agents

Instructions for coding agents working in this repository.

## Read first

- `SPEC.md` is the law, federated into node specs under `src/`,
  `tests/`, `dev/` and `nix/`. `SPEC-PROMPT.md` is the historical input
  the spec was made from.
- pipelith is extracted from actlith at commit `7ce69c6`: when a choice
  is open, do what actlith does, minus anything GitHub-specific.
- Nothing forge-specific lands here: no GitHub Actions or GitLab CI keys,
  runner labels, action pins or expression syntax.

## Working rules

- Work in TDD: a failing test first, then the code, then refactor.
- One topic per commit; commit through the hooks (never `--no-verify`).
- Commit subjects are `type(scope): claim` (at most 72 characters, scope
  in lower case) with a body that says why.
- Enter the dev shell (`direnv allow` or `nix develop`) before committing:
  the hooks refuse without `hk`.
- Spec edits go through `/spec`; `/build` only flips task status.
- A new check is added to `.pklith` and laid with `pklith lay`, one commit
  per check, after proving it rejects a planted violation.
- Never push, publish, or open issues or pull requests on another
  repository without the owner's explicit approval.
