#!/usr/bin/env bash
#
# `core.worktree` in `.git/config` is shared by every worktree, so a
# scratch `git init` that writes it points all of them at one checkout
# (actlith B5, nix V47). Refuse it whenever it is set.
#
#   nix/guard/no-core-worktree.sh
set -euo pipefail

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "no-core-worktree: not inside a git work tree, so nothing could be checked. This is a failure, not a pass." >&2
  exit 1
fi

if value="$(git config --local --get core.worktree)"; then
  echo "no-core-worktree: .git/config sets core.worktree=${value}; every worktree now acts on that checkout. Remove it: git config --local --unset core.worktree (nix V47)." >&2
  exit 1
fi
