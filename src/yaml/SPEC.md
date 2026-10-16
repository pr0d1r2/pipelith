# SPEC

## §G GOAL

`src/yaml` — generic ORDERED YAML model & writer. malformed YAML | injected expression ⊥ representable. ⊥ knows any pipeline key.

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/system|nix system names, CI coverage set, `x86_64-darwin` refusal
sib|src/files|generated header, write, drift check, path safety
sib|src/kind|neutral kind catalogue, row & arg validation, secret purposes, portability check
sib|src/detect|rows from tracked files, first config file

## §C CONSTRAINTS

- zero deps. writer only, ⊥ reader. yamllint-clean: `---` start, 2-space indent, 2 spaces before inline comment, ≤120 cols (`>-` fold), blocks `|` `|-` `|+`.
- from actlith `src/yaml/{node,scalar,text,render}`; GitHub model stays. `Expr` → `Dialect` (R6).

## §I INTERFACES

- types: `Node` (`Value` `List` `Map` `Seq` of maps `Null`), `Value` (`Text` `Int` `Bool`), `Entry` (`Key`, node, comments, trailing), `Text` (`Part::Lit` | `Part::Expr`), `Expr::new(&dyn Dialect, inner)`, `Comment`, trait `Dialect` (`expr(inner) -> Option<String>` validates & renders, `escape(lit) -> String` ∴ literal ⊥ reads as expression).
- fns: `document(&dyn Dialect, &[Comment], &[Entry]) -> String` from `---`; `quote`, `key`; `WIDTH` = 120.

## §V INVARIANTS

V23: output built from typed model only; ⊥ string substitution into YAML text (`actlith/V5`)
V24: 1 quoting rule: scalar plain only if safe chars, safe start, ⊥ reserved word (`true` `false` `yes` `no` `on` `off` `y` `n` `null` `~`, any case), ⊥ trailing space; else single-quoted; control chars → double-quoted w/ escapes. `Key::User` same rule; `Key::Fixed` = schema key, written as is
V25: expression syntax only via `Dialect`; ∀ literal part passes `Dialect::escape` ∴ literal text ⊥ becomes an expression. pipelith ships ⊥ dialect (forge crates do); tests use a GitHub-shaped test dialect
V26: fixed layout: map order = insertion order (caller sorts); comments attached to entries; same model → same bytes (`.:V1`)
V56: expr part plain only if ⊥ flow & rendered ⊥ leading indicator; scalar ⊥ `:`+space, space+`#`, trailing `:`

## §T TASKS

id|status|task|cites
T12|x|`Dialect` trait + port `text` (Expr → dialect parts), `scalar`, `node`, `render` w/ unit tests; golden-bytes tests incl. pklith `ci.yml` shape replayed w/ test dialect|V23,V24,V25,V26,`.:V3`,`.:V10`

## §B BUGS

id|date|cause|fix
B5|2026-10-17|expr judged on inner: `#a`, `a:` went plain|V56
B6|2026-10-17|literals escaped apart: `$`+`{{` opened expr|V25
