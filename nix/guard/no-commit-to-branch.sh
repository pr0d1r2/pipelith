#!/usr/bin/env bash
#
# Work lands on a branch and merges by pull request, never as a direct
# commit to `main` (nix C, V42). Refuse while HEAD is the protected branch.
# A detached HEAD (a rebase, a CI checkout of a pull request) is no branch
# and passes; CI itself sets CI=true and checks the merged `main`, where a
# push already happened, so it is skipped.
#
#   nix/guard/no-commit-to-branch.sh [BRANCH]
set -euo pipefail

protected="${1:-main}"

if [ "$(git rev-parse --is-inside-work-tree 2>/dev/null)" != true ]; then
  echo "no-commit-to-branch: not inside a git work tree, so nothing could be checked. This is a failure, not a pass." >&2
  exit 1
fi

if [ -n "${CI:-}" ]; then
  exit 0
fi

if ! branch="$(git symbolic-ref --short -q HEAD)"; then
  exit 0
fi

if [ "$branch" = "$protected" ]; then
  echo "no-commit-to-branch: HEAD is ${protected}; commit on a branch and open a pull request: git switch -c <type>/<topic> (nix V42)." >&2
  exit 1
fi
