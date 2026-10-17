# SPEC

## §G GOAL

`src/detect` — proposed rows from tracked files; first config file. ⊥ nix eval.

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/detect|rows from tracked files, first config file
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/system|nix system names, CI coverage set, `x86_64-darwin` refusal
sib|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/files|generated header, write, drift check, path safety
sib|src/kind|neutral kind catalogue, row & arg validation, secret purposes, portability check

## §C CONSTRAINTS

- std only. input = tracked path list + `flake.lock` text; caller lists files (node stays pure except `seed_init`).
- moves from actlith `src/detect/{rows,lock,text,seed}`; config name & tool from caller.

## §I INTERFACES

- fns: `detect(&[path], flake_lock: Option<&str>) -> Vec<kind::RawRow>` (ids `ci` → `gate`, `update-pins` → `update-pins`); `text(&[RawRow]) -> String` (`format 1`, `## workflows`, header, 1 line per row); `text_with(rows, line)`; `row_line(&RawRow)`; `seed_init(root, config, &[RawRow]) -> Result<PathBuf, SeedError>`; `seed_init_text(root, config, text)`; `SeedError::code()` 1 exists | 2 I/O.

## §V INVARIANTS

V35: `flake.nix` tracked → `gate` row `ci`; `flake.lock` has node `nixpkgs-lock` → `update-pins` row; rows in that fixed order (`.:V1`)
V36: `seed_init` ⊥ overwrites existing config; error naming it (`actlith/V32`)
V37: `text(rows)` parses back via `registry` + `kind` to same rows (round trip); `-` & `\|` escaped
V79: `text`, `seed_init` = `text_with`, `seed_init_text` w/ `row_line`; `line` output written as given (note + commented row = off)

## §T TASKS

id|status|task|cites
T16|x|port `detect`, `has_node`, `text`, `row_line`, `seed_init` w/ tests; round-trip test|V35,V36,V37,`.:V10`
T35|.|`text_with`, `seed_init_text`|V79,`.:V9`

## §B BUGS

id|date|cause|fix
