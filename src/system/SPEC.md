# SPEC

## §G GOAL

`src/system` — nix system NAMES & the CI coverage set. ⊥ runner labels (forge).

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/system|nix system names, CI coverage set, `x86_64-darwin` refusal
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/files|generated header, write, drift check, path safety

## §C CONSTRAINTS

- zero deps. 1 `const` table. moves from actlith `src/system/table` minus `runner` (stays: GitHub labels, `actlith/V15`).

## §I INTERFACES

- types: `System` (enum: `X86_64Linux` `Aarch64Linux` `Aarch64Darwin` `X86_64Darwin`), `SystemError` (`Unknown(name)`, `NoRunner(System)`).
- fns: `parse_system(&str) -> Result<System, SystemError>`, `System::name()`, `ci_system(System) -> Result<System, SystemError>` (refuses `x86_64-darwin`), consts `ALL`, `DEFAULT_SYSTEMS`.

## §V INVARIANTS

V20: nix system name ↔ `System` 1 table; unknown name → error listing known ones
V21: `x86_64-darwin` ⊥ CI system on any forge (⊥ hosted runner) → refused w/ hint: hand-verify on an Intel Mac
V22: `DEFAULT_SYSTEMS` = `x86_64-linux` `aarch64-linux` `aarch64-darwin` = set ∀ forge runner table ! map (forge test asserts total)

## §T TASKS

id|status|task|cites
T11|.|port `System`, parse & names w/ tests; `ci_system` refusal; ⊥ runner table|V20,V21,V22,`.:V10`

## §B BUGS

id|date|cause|fix
