# SPEC

## §G GOAL

`src/` — hub. library nodes of `pipelith`. ⊥ own logic itself.

## §F FEDERATION

dir|owns|⊥owns|tokens
registry|pipe-table grammar: format line, sections, header rows, cells, errors|what any section means|-
system|nix system names, CI coverage set, `x86_64-darwin` refusal|runner labels|-
yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait|pipeline keys, expression syntax|-
files|generated header, write, drift check, path safety|file contents, forge layout|-
kind|neutral kind catalogue, row & arg validation, secret purposes, portability check|pipeline shapes, secret names|-
detect|rows from tracked files, first config file|rendering, listing files|-

## §N NAV

rel|path|lens
up|.|-
self|src|the library: grammar, YAML writer, systems, files, kinds, detect
sib|tests|integration tests over public API, goldens copied from actlith
sib|dev|`pipelith-dev`, `publish = false`: commit rules, badges, notices, AGENTS step table

## §C CONSTRAINTS

- module = dir + `mod.rs`. `mod.rs` composes & re-exports, ⊥ implements.
- dep order (waves): `registry`, `system`, `yaml`, `files` → `kind` → `detect`. edges = `use crate::<node> as _;` lines in each `mod.rs`, 1 per line (sherd reads 1st line only).
- port from actlith @ `7ce69c6` = `pub(crate)` → `pub` + rustdoc; behaviour & bytes unchanged unless a §V here says otherwise.

## §V INVARIANTS

V12: `lib.rs` declares node modules & crate docs only, ⊥ logic.
V13: node ⊥ dep sibling node except through its `pub` surface; ⊥ cycle (wave order above).
V14: ∀ node unit-tested 1-to-1 (`<node>/<file>/tests.rs` beside `<file>.rs`, `rust-mirror`); fixtures in `tests/fixtures/<node>/`; ⊥ network, ⊥ clock in tests.

## §T TASKS

id|status|task|cites
T8|.|scaffold `src/lib.rs` + `src/<node>/mod.rs` w/ wave edges ∴ `sherd wave` orders nodes|V12,V13
T9|.|`lib.rs` declares each node as it lands; crate doc names the forge contract|V12,`.:V9`

## §B BUGS

id|date|cause|fix
