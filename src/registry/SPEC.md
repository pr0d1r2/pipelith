# SPEC

## §G GOAL

`src/registry` — pipe-table registry GRAMMAR: text + caller schema → sections of rows of cells w/ line numbers. ⊥ knows any section's meaning.

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors

## §C CONSTRAINTS

- zero deps. pure fns over text; ⊥ I/O (caller reads file, passes its name for errors).
- grammar = pklith registry style: `format N` line, `## <section>`, header row then pipe-table rows, `#` comment lines, blank lines, `-` empty cell, `\|` literal pipe.
- moves from actlith `src/registry/{format,table,cells,parse,error}`; actlith's fixed `Table` enum → caller `Schema` (D16). `rows`, `model` stay in actlith.

## §I INTERFACES

- types: `Schema { sections: &[SectionSpec] }`, `SectionSpec { name, columns: &[&str], required }`, `Document { format, sections }`, `Section { name, line, rows }`, `Row { line, cells }`, `Cell { column, text }`, `Arg { key, value }`, `RegistryError { file, line, column, what, fix }` (`Display` = `file:line:column: what (fix)`).
- consts: `SUPPORTED_FORMAT` = 1.
- fns: `parse(file, text, &Schema) -> Result<Document, RegistryError>`; `Document::section(name) -> &[Row]` (absent optional = empty); cell readers `required`, `optional` (`-` → `None`), `list` (`,`-split, trimmed, `-` = empty), `args` (`key=value`, `,`-split).

## §V INVARIANTS

V15: first non-comment non-blank line ! be `format N`; missing → error; N > `SUPPORTED_FORMAT` → error naming both versions, ⊥ best-effort parse (`actlith/V11`)
V16: ∀ error names file, line & column (1-based) + a fix hint; ⊥ panic on any input
V17: shape ! match schema: unknown section → error naming known ones; duplicate section → error; header row ≠ declared columns → error; row cell count ≠ columns → error at row; missing required section → error; missing optional section = empty (`actlith/V12`)
V18: cell text trimmed; `\|` → literal `|`, ⊥ splits; `-` alone = empty; `#` starts a comment only at line start
V19: `list` & `args` split on `,` ∴ ⊥ comma inside a value; `args` cell w/o `=` | empty key → error naming cell; order kept

## §T TASKS

id|status|task|cites
T10|.|port `format`, `cells`, `parse`, `error` w/ unit tests from actlith; replace `Table` w/ `Schema`; fixtures 1-to-1 incl. actlith `.actlith` fixtures parsed via an actlith-shaped test schema|V15,V16,V17,V18,V19,`.:V10`

## §B BUGS

id|date|cause|fix
