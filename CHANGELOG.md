# Changelog

All notable changes to pipelith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes the YAML writer or the generated header emits for the same
input is at least a minor version (SPEC.md V7), and its entry names what
consumers will see change.

## [Unreleased]

### Added

- `src/runner`: the `## runners` section, header `system|labels`, where
  a repository names its own runner for a nix system, for example
  `x86_64-darwin|self-hosted, macOS, X64`. Its labels replace the
  forge's default runner for that system in every `## workflows` row.
  A forge puts `runner::RUNNERS` in its schema, reads the section with
  `Runners::from_document` and calls `Runners::pick(system,
  has_default)` for each system of each checked row. The answer is
  `Runner::Default`, `Runner::Labels(..)`, or `SystemError::NoRunner`
  when there is neither a row nor a forge default. An unknown system,
  an empty `labels` cell, or a label other than letters, digits and
  `._-` (or one starting with `-`) is refused, located.
- `Runner::trusted_only()` is true for every `Runner::Labels`, with no
  opt-out: a forge runs such a job only for a push, a schedule, a
  manual run or a change request from the same repository, never for
  a change request from a fork, whose code would otherwise run on the
  repository's own machine.

### Changed

- `x86_64-darwin` is a CI system (paradigm T19, owner 2026-10-24).
  `system::DEFAULT_SYSTEMS` is now all four systems, so a `gate` or
  `build` row with `-` in `systems` runs on Intel macOS too, and a row
  may list `x86_64-darwin`. Every consumer's generated pipeline gains
  that system once it takes this version, and each forge's runner
  table has to cover it, by a default label or by refusing with
  `NoRunner`.
- `system::ci_system` is removed: every system is a CI system.
  `SystemError::NoRunner` now means "this forge has no default runner
  and `## runners` names none", and its message names that fix.
- `tests/golden/ci.yaml` differs from what actlith rendered at
  `7ce69c6` by one thing: the gate matrix lists `x86_64-darwin`, with
  GitHub's Intel macOS label in the test's runner table.
- These are breaking API changes, so the next release is `0.2.0`.

## [0.1.1] - 2026-10-17

### Added

- `src/detect`: `text_with(rows, line)` writes the header, then
  `line(row)` and a newline for each row, as given; `seed_init_text(root,
  config, text)` writes that text with `seed_init`'s rule (created new,
  never over an existing file). A forge can now propose a row that stays
  off until the user opts in, such as a note line and a commented-out
  `update-pins` row, with its own note text. `text` and `seed_init` are
  these with `row_line`, and their bytes do not change.

## [0.1.0] - 2026-10-17

### Added

- `tests/golden/ci.yaml`: the `ci.yml` actlith rendered at `7ce69c6`
  from its own `.actlith` `ci` row, byte for byte. The end-to-end test
  rebuilds it through the public API (`registry`, `kind`, `yaml`,
  `files`); a later change to these bytes needs its own entry here.
- `src/kind`: `Row::check` refuses what actlith refused only while
  rendering: a `build` row without a cache, a `package` that is not a
  flake attribute name, a `cron` that is not five fields, a `token`
  that is not a secret name, a bump kind on more than one system. A
  row that passes is never refused later by a forge.
- `src/kind`: `Row::check` refuses a `build` (`update-pins`), `script`
  or `branch-prefix` (`upstream`) arg that is not a plain word (letters,
  digits and `._/#+:@-`, not empty, not starting with `-`), as actlith
  did only while rendering, with actlith's message.
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

[Unreleased]: https://github.com/pr0d1r2/pipelith/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/pr0d1r2/pipelith/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/pr0d1r2/pipelith/releases/tag/v0.1.0
