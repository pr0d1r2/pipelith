# Changelog

All notable changes to pipelith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes the YAML writer or the generated header emits for the same
input is at least a minor version (SPEC.md V7), and its entry names what
consumers will see change.

## [Unreleased]

### Added

- `pipelith::yaml`: the ordered YAML model and writer moved from actlith
  (`Node`, `Entry`, `Text`, `Comment`, `document`), byte-identical to
  actlith's output for the same tree. Expression syntax comes from a
  `Dialect` the forge crate supplies. Two fixes against actlith: an
  expression whose rendered text would read as a mapping or a comment
  in a plain scalar is quoted, and adjacent literal parts are escaped as
  one run.
