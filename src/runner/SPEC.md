# SPEC

## §G GOAL

`src/runner` — `## runners`: a repo's own runner labels per nix system, over the forge's default; trust guard for them. ⊥ default labels (forge).

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/runner|own runner labels per system
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/system|nix system names, CI coverage set
sib|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/files|generated header, write, drift check, path safety
sib|src/kind|neutral kind catalogue, row & arg validation, secret purposes, portability check
sib|src/detect|rows from tracked files, first config file

## §C CONSTRAINTS

- deps: std, `registry`, `system`. section shape & checks neutral; label values = the repo's own runner's, typed by the user.
- self-hosted runner on public repo runs fork code w/ the machine's access (GitHub advises against) ∴ guard ⊥ optional.

## §I INTERFACES

- section `## runners`, header `system|labels`, key `system`, optional: `x86_64-darwin|self-hosted, macOS, X64` → that system's jobs in ∀ `## workflows` row run on a runner w/ ∀ those labels.
- consts: `SECTION` = `runners`, `COLUMNS`, `RUNNERS` (`SectionSpec`, key col 0). forge crate puts `RUNNERS` in its schema.
- types: `Runners` (checked rows), `Runner<'a>::{Default, Labels(&'a [String])}`.
- fns: `Runners::from_document(file, &Document) -> Result<Runners, RegistryError>`, `Runners::pick(System, has_default: bool) -> Result<Runner, SystemError>` (`NoRunner` ⇔ ⊥ row ∧ ⊥ default), `Runner::trusted_only()`.
- forge: per row system, `pick` w/ its table's answer, at validation (∴ ⊥ refused at render); `Default` → its label; `Labels` → those labels + guard.

## §V INVARIANTS

V80: row → `Runner::Labels` for its system, over forge default; ⊥ row → `Default` | `NoRunner`; label = `[A-Za-z0-9._-]+`, ⊥ leading `-`; ≥ 1 label; unknown | repeated system → error w/ line & column
V81: `Labels` ⇒ `trusted_only`: forge runs job only for push, schedule, manual run, change request from same repo; ⊥ fork change request; ⊥ opt-out; forge test asserts

## §T TASKS

id|status|task|cites
T37|.|`RUNNERS`, `Runners`, `Runner`, `trusted_only` w/ tests; `lib.rs` declares node|V80,V81,`.:V9`,`.:T38`

## §B BUGS

id|date|cause|fix
