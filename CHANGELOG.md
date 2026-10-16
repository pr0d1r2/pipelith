# Changelog

All notable changes to pipelith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes the YAML writer or the generated header emits for the same
input is at least a minor version (SPEC.md V7), and its entry names what
consumers will see change.

## [Unreleased]

### Added

- `src/files`: the generated-file set ported from actlith: `header(tool,
  config)`, `write` (only listed paths, unchanged files untouched),
  `check` (stale or missing files sorted by path, writes nothing) and
  `target` (refuses any path that could leave the root).
