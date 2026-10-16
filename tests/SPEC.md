# SPEC

## §G GOAL

`tests/` — integration tests over the public API; byte-identity proof vs actlith.

## §N NAV

rel|path|lens
up|.|-
self|tests|integration tests over public API, goldens copied from actlith
sib|src|the library: grammar, YAML writer, systems, files, kinds, detect

## §C CONSTRAINTS

- `tests/*.rs` use only `pipelith::` public items. fixtures `tests/fixtures/<node>/`, goldens `tests/golden/`.

## §V INVARIANTS

V38: golden bytes copied from actlith @ `7ce69c6` change only w/ CHANGELOG entry (`.:V7`); `UPDATE_GOLDEN=1` rewrites, ⊥ default
V39: public API usable as actlith will: parse `.actlith` fixture via `registry` + `kind`, render a workflow-shaped model via `yaml` + test dialect + `files::header`, compare to golden

## §T TASKS

id|status|task|cites
T17|.|end-to-end test: actlith `.actlith` fixture → rows → test-built model → bytes == actlith golden `ci.yml`|V38,V39,`.:V3`

## §B BUGS

id|date|cause|fix
