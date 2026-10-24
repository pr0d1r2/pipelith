# SPEC

## §G GOAL

`src/system` — nix system NAMES & the CI coverage set (∀ 4). ⊥ runner labels (forge).

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/system|nix system names, CI coverage set
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/files|generated header, write, drift check, path safety
sib|src/kind|neutral kind catalogue, row & arg validation, secret purposes, portability check
sib|src/runner|own runner labels per system
sib|src/detect|rows from tracked files, first config file

## §C CONSTRAINTS

- zero deps. 1 `const` table. moves from actlith `src/system/table` minus `runner` (stays: GitHub labels, `actlith/V15`).

## §I INTERFACES

- types: `System` (enum: `X86_64Linux` `Aarch64Linux` `Aarch64Darwin` `X86_64Darwin`), `SystemError` (`Unknown(name)`, `NoRunner(System)`: forge has ⊥ default, fix names `## runners`).
- fns: `parse_system(&str) -> Result<System, SystemError>`, `System::name()`, consts `ALL`, `DEFAULT_SYSTEMS`.

## §V INVARIANTS

V20: nix system name ↔ `System` 1 table; unknown name → error listing known ones
V21: ∀ system = CI system (owner 2026-10-24, paradigm T19); was: `x86_64-darwin` refused (⊥ hosted runner)
V22: `DEFAULT_SYSTEMS` = `ALL` = set ∀ forge runner table ! cover: label | ⊥ default → `NoRunner` unless `src/runner` row (forge test asserts total)

## §T TASKS

id|status|task|cites
T11|x|port `System`, parse & names w/ tests; `ci_system` refusal; ⊥ runner table|V20,V21,V22,`.:V10`
T36|.|`DEFAULT_SYSTEMS` = `ALL`; drop `ci_system`; `NoRunner` message names `## runners` fix|V21,V22,`.:T38`

## §B BUGS

id|date|cause|fix
