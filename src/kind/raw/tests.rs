//! The `## workflows` section and its rows typed from cells (T14).

use crate::kind::{COLUMNS, RawRow, SECTION, WORKFLOWS};
use crate::registry::{Arg, RegistryError, Schema, parse};

const FILE: &str = ".lablith";

/// A registry file with `rows` under the workflows header.
fn text(rows: &str) -> String {
    format!("format 1\n\n## workflows\nid|kind|systems|cache|args\n{rows}")
}

/// The rows of `rows`, typed; the first refusal as text.
fn typed(rows: &str) -> Result<Vec<RawRow>, String> {
    let schema = Schema {
        sections: &[WORKFLOWS],
    };
    let doc = parse(FILE, &text(rows), &schema).map_err(|e| e.to_string())?;
    let cells = doc.section(SECTION);
    let rows = cells.iter().map(|r| RawRow::from_cells(FILE, r));
    rows.collect::<Result<_, RegistryError>>()
        .map_err(|e| e.to_string())
}

fn refused(rows: &str) -> String {
    typed(rows).err().unwrap_or_default()
}

#[test]
fn the_section_is_workflows() {
    assert_eq!(SECTION, "workflows");
    assert_eq!(COLUMNS, ["id", "kind", "systems", "cache", "args"]);
    assert_eq!(WORKFLOWS.name, SECTION);
    assert_eq!(WORKFLOWS.columns, COLUMNS);
}

#[test]
fn the_section_is_optional_and_keyed_on_id() {
    assert_eq!(WORKFLOWS.header(), "id|kind|systems|cache|args");
    assert_eq!((WORKFLOWS.required, WORKFLOWS.key), (false, Some(0)));
}

#[test]
fn a_full_row_is_typed() -> Result<(), String> {
    let rows =
        typed("ci|gate|aarch64-darwin, x86_64-linux|pr0d1r2|timeout=45")?;
    let want = RawRow {
        line: 5,
        id: "ci".to_owned(),
        kind: "gate".to_owned(),
        systems: vec!["aarch64-darwin".to_owned(), "x86_64-linux".to_owned()],
        cache: Some("pr0d1r2".to_owned()),
        args: vec![Arg {
            key: "timeout".to_owned(),
            value: "45".to_owned(),
        }],
    };
    assert_eq!(rows, [want]);
    Ok(())
}

#[test]
fn a_dash_row_is_empty() -> Result<(), String> {
    let rows = typed("ci.v2_x-y|gate|-|-|-")?;
    let want = RawRow {
        line: 5,
        id: "ci.v2_x-y".to_owned(),
        kind: "gate".to_owned(),
        ..RawRow::default()
    };
    assert_eq!(rows, [want]);
    Ok(())
}

#[test]
fn an_id_that_is_not_a_file_name_is_refused() {
    for id in ["a/b", ".hidden", "a b"] {
        assert_eq!(
            refused(&format!("{id}|gate|-|-|-")),
            format!(
                ".lablith:5:1: id `{id}` is not a file name (use letters, \
                 digits, `.`, `_`, `-`; no leading `.`)"
            )
        );
    }
}

#[test]
fn an_empty_id_or_kind_is_refused() {
    let id = refused("-|gate|-|-|-");
    assert!(id.starts_with(".lablith:5:1: `id` is empty"), "{id}");
    let kind = refused("ci|-|-|-|-");
    assert!(kind.starts_with(".lablith:5:4: `kind` is empty"), "{kind}");
}

#[test]
fn malformed_systems_and_args_are_refused() {
    let systems = refused("ci|gate|a,,b|-|-");
    assert!(systems.contains("`systems` has an empty item"), "{systems}");
    let args = refused("ci|gate|-|-|timeout");
    assert!(args.contains("arg `timeout` is not `key=value`"), "{args}");
}

#[test]
fn a_repeated_id_is_refused_by_the_registry() {
    let error = refused("ci|gate|-|-|-\nci|build|-|-|-");
    assert!(error.starts_with(".lablith:6:1: "), "{error}");
}
