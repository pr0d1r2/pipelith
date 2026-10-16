# SPEC

## §G GOAL

`src/` — hub. library nodes of `pipelith`. ⊥ own logic itself.

## §F FEDERATION

dir|owns|⊥owns|tokens

## §N NAV

rel|path|lens
up|.|-
self|src|the library: grammar, YAML writer, systems, files, kinds, detect

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
