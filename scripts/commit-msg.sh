#!/usr/bin/env bash
# The commit message rule (nix §C, T21): subject `type(scope): claim`, at
# most 72 characters, and a body that says why. Taken from pklith's shell
# version; it moves into `pipelith-dev` as tested Rust once that crate
# exists (dev T18), as pklith's did.
#
#   scripts/commit-msg.sh [MESSAGE_FILE]
#
# The hook passes the file git gave it (MERGE_MSG on a merge); by hand it
# defaults to git's COMMIT_EDITMSG.
set -euo pipefail

f="${1:-$(git rev-parse --git-path COMMIT_EDITMSG)}"
# A missing file means nothing was checked, which is a failure, not a pass.
[ -f "$f" ] || {
  echo "commit-msg: no message file at $f, so nothing was checked." >&2
  exit 1
}

# Judge only what git keeps: drop everything from the `git commit -v`
# scissors line down (pklith found a diff there counted as a body), then
# comment lines.
msg="$(sed '/^# -\{24\} >8 -\{24\}$/,$d' "$f" | grep -v '^#' || true)"
subject="$(printf '%s\n' "$msg" | sed -n 1p)"

# git or the operator writes these; they are not held to the style.
case "$subject" in "Merge "* | "Revert "* | "fixup! "* | "squash! "*) exit 0 ;; esac

types='feat|fix|docs|test|chore|refactor|perf|build|ci'
if ! printf '%s\n' "$subject" | grep -Eq "^($types)(\([a-z0-9/._-]+\))?!?: [^ ]"; then
  echo "commit-msg: subject must be 'type(scope): claim', type one of ${types//|/ }. Got: $subject" >&2
  exit 1
fi

if [ "${#subject}" -gt 72 ]; then
  echo "commit-msg: subject is ${#subject} characters, the limit is 72. Say less, or move detail to the body." >&2
  exit 1
fi

body="$(printf '%s\n' "$msg" | sed -n '2,$p' | grep -v '^Co-Authored-By:' | grep -c '[^[:space:]]' || true)"
if [ "$body" -eq 0 ]; then
  echo "commit-msg: the body is empty. Say WHY: the diff already shows what changed." >&2
  exit 1
fi
