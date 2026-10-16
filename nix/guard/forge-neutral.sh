#!/usr/bin/env bash
#
# pipelith holds no forge-specific code (SPEC.md V2, nix V50): no GitHub
# Actions or GitLab CI keys, runner labels, expression syntax or API
# hosts in the library. Forge crates own those.
#
#   nix/guard/forge-neutral.sh
#
# Scans tracked `src/**/*.rs` except unit tests (`tests.rs`): tests replay
# actlith's bytes through a GitHub-shaped test dialect (src/yaml V25), so
# they are allowed to spell GitHub syntax. The match is textual and
# includes comments, so a doc comment that names a forge key fails too:
# name the forge crate instead.
set -euo pipefail

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "forge-neutral: not inside a git work tree -- the file list comes from git, so nothing could be checked. This is a failure, not a pass." >&2
  exit 1
fi

# The tokens are literal text, never expanded.
# shellcheck disable=SC2016
tokens=(
  '${{'
  'runs-on'
  'uses:'
  'stages:'
  'image:'
  'ubuntu-latest'
  'ubuntu-24.04-arm'
  'macos-latest'
  'api.github.com'
  'gitlab.com'
  'GITHUB_TOKEN'
  'CI_JOB_TOKEN'
)

status=0
while IFS= read -r file; do
  case "$file" in
  */tests.rs) continue ;;
  esac
  for token in "${tokens[@]}"; do
    if hits="$(grep -nF -- "$token" "$file")"; then
      while IFS= read -r hit; do
        echo "forge-neutral: ${file}:${hit%%:*}: \`${token}\` is forge-specific; it belongs in actlith or lablith (SPEC.md V2)." >&2
      done <<<"$hits"
      status=1
    fi
  done
done < <(git ls-files 'src/*.rs' 'src/**/*.rs')

exit "$status"
