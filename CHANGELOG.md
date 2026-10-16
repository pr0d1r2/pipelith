# Changelog

All notable changes to pipelith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes the YAML writer or the generated header emits for the same
input is at least a minor version (SPEC.md V7), and its entry names what
consumers will see change.

## [Unreleased]

### Added

- `src/registry`: the pipe-table registry grammar. `parse` checks a
  file against a caller `Schema` (format line, sections, headers, cell
  counts, repeated keys) and returns sections of rows of cells; errors
  name file, line and column with a fix; cell readers `required`,
  `optional`, `list` and `args` type a cell.
