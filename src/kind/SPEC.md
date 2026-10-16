# SPEC

## §G GOAL

`src/kind` — NEUTRAL kind catalogue: names, row columns, arg schemas, default systems, secret purposes; row validation & portability check. ⊥ builds any pipeline.

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/kind|neutral kind catalogue, row & arg validation, secret purposes, portability check
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/system|nix system names, CI coverage set, `x86_64-darwin` refusal
sib|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/files|generated header, write, drift check, path safety
sib|src/detect|rows from tracked files, first config file

## §C CONSTRAINTS

- zero deps. catalogue = `const` data; forge crate adds its shape per kind.
- moves from actlith `src/kind/{row,args,contract,cache}` + `secret` purpose half + `## workflows` row model of `src/registry/model`; kind schemas copied from actlith `gate`, `build`, `bump` nodes.
- section `workflows`, columns `id|kind|systems|cache|args` (`.:T7`).

## §I INTERFACES

- types: `KindSpec { name, schema: &[ArgSpec], default_systems, secrets: &[Purpose] }`, `ArgSpec { key, required, default: ArgDefault }`, `ArgDefault::{None, Value(&str), Forge}` (forge fills: e.g. `token`; absent from `Args`), `Purpose::{CachePush, OpenChange}`, `RawRow { line, id, kind, systems, cache, args }` (cells), `Row { line, id, systems: Vec<System>, cache: Option<Cache>, args }` (checked), `Cache` (`name()`, `key()`, `nix_config()`), `Args`, `KindError { line, what, fix }` (`located(file)` → `RegistryError`), `Extras<'a> { kinds: &[KindSpec], keys: &[ArgSpec] }` (forge-only; key valid on ∀ kind), `Finding::{Kind, Key}`.
- consts: `SECTION` = `workflows`, `COLUMNS`, `WORKFLOWS` (`SectionSpec`, key col 0), `CATALOGUE` = `GATE` `BUILD` `UPDATE_PINS` `UPSTREAM`, `TIMEOUT` spec, `DEFAULT_TIMEOUT`, `MAX_TIMEOUT`.
- fns: `RawRow::from_cells(file, &registry::Row) -> Result<RawRow, RegistryError>` (id = file stem), `lookup(name)`, `resolve(&RawRow, &Extras) -> Result<&KindSpec, KindError>`, `Row::check(&KindSpec, &RawRow, &Extras) -> Result<Row, KindError>` (+ timeout range, V75-V77), `Cache::parse(line, name)`, `parse_args(&KindSpec, &RawRow, &Extras) -> Result<Args, KindError>`, `Args::{get, number, line, timeout}`, `KindSpec::purposes(&Row)`, `portable(&RawRow, &Extras) -> Vec<Finding>`.
- schemas: `gate` command?=`nix develop --command hk check --all --check`, timeout · `build` package!, timeout · `update-pins` cron?=`50 6 * * *`, build?=`.#default`, token?=forge, timeout · `upstream` script!, cron?=`20 6 * * *`, branch-prefix?=`upstream/`, token?=forge, timeout. timeout?=`30`, 1..=360.

## §V INVARIANTS

V31: unknown | repeated | missing required arg key → error naming row line & key; ⊥ silent default; forge-only key valid only when in `Extras` (`actlith/V10`)
V32: ∀ kind declares ∀ secret PURPOSE it needs; forge maps purpose → secret name & injection; fork change request needs ⊥ secret (`actlith/V14`)
V33: `portable` → 1 finding per forge-only kind | key in row; empty ⇔ row renders on both forges (D4, D17)
V34: catalogue values (names, keys, defaults) == actlith @ `7ce69c6` except `ArgDefault::Forge` where actlith default named a GitHub secret (`.:V3`)

V52: systems `-` → kind default; ∀ system via `system::parse_system` + `ci_system` (`x86_64-darwin` refused); duplicate → error
V53: cache = cachix name w/ public key from built-in table (v1 `pr0d1r2`); other name refused w/ fix hint, ⊥ guessed key. cachix serves both forges ∴ neutral

V75: `Row::check` refuses what actlith refused only at build: `build` w/o cache; `package` ∉ flake attr name (`[A-Za-z_][A-Za-z0-9_-]*`); `cron` ≠ 5 fields of `[A-Za-z0-9*/-]`; `token` ∉ secret name (`[A-Za-z_][A-Za-z0-9_]*`) ∴ passing row ⊥ refused by a forge (`.:V4`); messages = actlith's
V76: kind w/ `Purpose::OpenChange` (bump) runs 1 job ∴ exactly 1 system; else error naming count
V77: `build` (update-pins), `script` & `branch-prefix` (upstream) ∉ `[A-Za-z0-9._/#+:@-]+` | leading `-` → refused by `Row::check` (actlith `plain()` at build); message = actlith's

## §T TASKS

id|status|task|cites
T14|x|`Row`, `ArgSpec`, `parse_args`, `Args` port w/ tests; `Purpose`; `CATALOGUE` w/ 4 kinds|V31,V32,V34,V52,V53,`.:V5`,`.:V10`
T15|x|`Extras` & `portable` w/ tests: forge-only kind, forge-only key, clean row|V33,`.:V4`
T32|x|move actlith build-time row rules here: `build` ! cache, `package` = flake attr name, `cron` = 5 fields, `token` = secret name, bump kinds exactly 1 system ∴ portable row ⊥ refused later by a forge|`.:V4`,V75,V76
T33|x|port actlith `plain()` (`src/bump/shared.rs`): `build`, `script`, `branch-prefix` values = letters, digits, `._/#+:@-`, ⊥ empty, ⊥ leading `-`; refused in `Row::check`|`.:V4`,V77

## §B BUGS

id|date|cause|fix
