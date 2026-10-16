# Changelog

All notable changes to pipelith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes the YAML writer or the generated header emits for the same
input is at least a minor version (SPEC.md V7), and its entry names what
consumers will see change.

## [Unreleased]

### Added

- `src/detect`: proposed `## workflows` rows from tracked paths and
  `flake.lock` text (`detect`: `ci|gate` for a tracked `flake.nix`,
  then `update-pins` for a `nixpkgs-lock` node; `has_node`); `text`
  and `row_line` write them as a `format 1` file that parses back to
  the same rows; `seed_init(root, config, rows)` writes the caller's
  config file once and never overwrites it (`SeedError::code`: 1
  exists, 2 unwritable).
- `src/files`: the generated-file set ported from actlith: `header(tool,
  config)`, `write` (only listed paths, unchanged files untouched),
  `check` (stale or missing files sorted by path, writes nothing) and
  `target` (refuses any path that could leave the root).
- `src/kind`: the neutral kind catalogue (`gate`, `build`,
  `update-pins`, `upstream`) with actlith's arg schemas, default
  systems and secret purposes; the `## workflows` section and
  `RawRow::from_cells`; `Row::check` (systems, cachix cache, args,
  timeout) with errors naming the row's line and key; forge `Extras`
  and `portable`, one finding per forge-only kind or arg key.
- `src/registry`: the pipe-table registry grammar. `parse` checks a
  file against a caller `Schema` (format line, sections, headers, cell
  counts, repeated keys) and returns sections of rows of cells; errors
  name file, line and column with a fix; cell readers `required`,
  `optional`, `list` and `args` type a cell.
- `pipelith::yaml`: the ordered YAML model and writer moved from actlith
  (`Node`, `Entry`, `Text`, `Comment`, `document`), byte-identical to
  actlith's output for the same tree. Expression syntax comes from a
  `Dialect` the forge crate supplies. Two fixes against actlith: an
  expression whose rendered text would read as a mapping or a comment
  in a plain scalar is quoted, and adjacent literal parts are escaped as
  one run.
