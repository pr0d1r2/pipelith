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

## §C CONSTRAINTS

- zero deps. writer only, ⊥ reader. style yamllint-clean: `---` start, 2-space indent, 2 spaces before inline comments, ≤120 cols (fold long scalars `>-`), literal blocks `|` `|-` `|+`.
- moves from actlith `src/yaml/{node,scalar,text,render}`; `workflow`, `job`, `step` stay. `Expr` was GitHub syntax (R6) → `Dialect`.

## §I INTERFACES

- types: `Node` (`Map(Vec<Entry>)`, `Seq(Vec<Node>)`, `Scalar(Text)`, `Value` plain kinds), `Entry` (key, node, comments, inline comment), `Text` (`Part::Lit` | `Part::Expr`), `Comment`, trait `Dialect` (`expr(inner) -> Option<String>` validates & renders, `escape(lit) -> String` neutralizes literal text that would read as an expression).
- fns: `document(&dyn Dialect, &[Comment], &[Entry]) -> String`; scalar helpers `quote`, `key`; const `WIDTH` = 120.

## §V INVARIANTS

V23: output built from typed model only; ⊥ string substitution into YAML text (`actlith/V5`)
V24: 1 quoting rule: scalar plain only if safe chars, safe start, ⊥ reserved word (`true` `false` `yes` `no` `on` `off` `y` `n` `null` `~`, any case), ⊥ trailing space; else single-quoted; control chars → double-quoted w/ escapes. keys same rule
V25: expression syntax only via `Dialect`; ∀ literal part passes `Dialect::escape` ∴ literal text ⊥ becomes an expression. pipelith ships ⊥ dialect (forge crates do); tests use a GitHub-shaped test dialect
V26: fixed layout: map order = insertion order (caller sorts); comments attached to entries; same model → same bytes (`.:V1`)

## §T TASKS

id|status|task|cites
T12|.|`Dialect` trait + port `text` (Expr → dialect parts), `scalar`, `node`, `render` w/ unit tests; golden-bytes tests incl. pklith `ci.yml` shape replayed w/ test dialect|V23,V24,V25,V26,`.:V3`,`.:V10`

## §B BUGS

id|date|cause|fix
