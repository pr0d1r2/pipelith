//! One test per fixture in `tests/fixtures/registry/` (registry T10),
//! parsed through an actlith-shaped schema.

use crate::registry::{
    Document, RegistryError, Schema, SectionSpec, args, list, parse, required,
};

const FILE: &str = ".actlith";

/// actlith's four sections; none required, each keyed on its first
/// column.
const ACTLITH: Schema<'static> = Schema {
    sections: &[
        spec("workflows", &["id", "kind", "systems", "cache", "args"]),
        spec("actions", &["action", "version", "sha"]),
        spec("hand-written", &["path", "reason"]),
        spec("bots", &["bot", "args"]),
    ],
};

const fn spec(
    name: &'static str,
    columns: &'static [&'static str],
) -> SectionSpec<'static> {
    SectionSpec {
        name,
        columns,
        required: false,
        key: Some(0),
    }
}

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            "../../../tests/fixtures/registry/",
            $name,
            ".actlith"
        ))
    };
}

fn parsed(text: &str) -> Document {
    parse(FILE, text, &ACTLITH).unwrap_or_default()
}

fn refused(text: &str) -> RegistryError {
    parse(FILE, text, &ACTLITH).err().unwrap_or_default()
}

/// The first cell of each row of `section`.
fn keys<'a>(doc: &'a Document, section: &str) -> Vec<&'a str> {
    let first = |r: &'a crate::registry::Row| r.cells.first();
    let rows = doc.section(section).iter().filter_map(first);
    rows.map(|c| c.text.as_str()).collect()
}

#[test]
fn full_fills_the_workflows() {
    let doc = parsed(fixture!("full"));
    assert_eq!(doc.format, 1);
    assert_eq!(keys(&doc, "workflows"), ["ci", "update-pins", "check"]);
    let rows = doc.section("workflows");
    let lines: Vec<usize> = rows.iter().map(|r| r.line).collect();
    assert_eq!(lines, [8, 10, 11]);
    let ci = rows.first().cloned().unwrap_or_default();
    assert_eq!(list(FILE, &ci, 2, "systems").map(|s| s.len()), Ok(3));
    let check = rows.last().cloned().unwrap_or_default();
    let got = args(FILE, &check, 4, "args").unwrap_or_default();
    let value = got.first().map(|a| a.value.as_str());
    assert_eq!(value, Some("hk check --all | tee log"));
}

#[test]
fn full_fills_the_other_sections() {
    let doc = parsed(fixture!("full"));
    let pin = doc.section("actions").first().cloned().unwrap_or_default();
    let version = pin.cells.get(1).map(|c| c.text.as_str());
    assert_eq!((pin.line, version), (15, Some("v4.2.2")));
    assert_eq!(doc.section("hand-written").len(), 1);
    assert_eq!(keys(&doc, "bots"), ["coderabbit", "socket"]);
    let names: Vec<&str> =
        doc.sections.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, ["workflows", "actions", "hand-written", "bots"]);
    let heads: Vec<usize> = doc.sections.iter().map(|s| s.line).collect();
    assert_eq!(heads, [5, 13, 17, 21]);
}

#[test]
fn full_cells_carry_their_columns() {
    let doc = parsed(fixture!("full"));
    let bot = doc.section("bots").first().cloned().unwrap_or_default();
    let columns: Vec<usize> = bot.cells.iter().map(|c| c.column).collect();
    assert_eq!(columns, [1, 12]);
}

#[test]
fn format_only_has_empty_sections() {
    let want = Document {
        format: 1,
        sections: vec![],
    };
    assert_eq!(parse(FILE, fixture!("format-only"), &ACTLITH), Ok(want));
}

#[test]
fn no_format_is_refused() {
    let e = refused(fixture!("no-format"));
    assert_eq!((e.line, e.what.as_str()), (1, "no `format` line"));
    assert_eq!(refused("").what, "no `format` line");
    let e = refused("# only a comment\n\n");
    assert_eq!(
        (e.line, e.column, e.what.as_str()),
        (1, 1, "no `format` line")
    );
}

#[test]
fn newer_format_is_refused() {
    let e = refused(fixture!("newer-format"));
    assert_eq!((e.file.as_str(), e.line, e.column), (FILE, 1, 1));
    assert_eq!(e.what, "`.actlith` is format 2; this build reads format 1");
}

#[test]
fn duplicate_id_names_both_lines() {
    let e = refused(fixture!("duplicate-id"));
    assert_eq!((e.line, e.column), (7, 1));
    assert_eq!(e.what, "id `ci` given twice, first on line 5");
}

#[test]
fn empty_reason_is_refused_by_its_reader() {
    let doc = parsed(fixture!("empty-reason"));
    let row = doc.section("hand-written").first().cloned();
    let row = row.unwrap_or_default();
    assert!(required(FILE, &row, 0, "path").is_ok());
    let e = required(FILE, &row, 1, "reason").err().unwrap_or_default();
    assert_eq!((e.line, e.column), (5, 31));
    assert_eq!(e.what, "`reason` is empty");
    assert_eq!(e.to_string().split(": ").next(), Some(".actlith:5:31"));
}

#[test]
fn bad_cell_count_is_refused() {
    let e = refused(fixture!("bad-cell-count"));
    assert_eq!((e.line, e.column), (5, 1));
    assert_eq!(e.what, "4 cells where `## workflows` rows have 5");
}

#[test]
fn bad_header_is_refused() {
    let e = refused(fixture!("bad-header"));
    assert_eq!(e.line, 4);
    assert_eq!(
        e.what,
        "`## workflows` header is `id|kind|systems|gate|cache`"
    );
    assert_eq!(e.fix, "start the section with `id|kind|systems|cache|args`");
}

#[test]
fn unknown_section_is_refused() {
    let e = refused(fixture!("unknown-section"));
    assert_eq!(
        (e.line, e.what.as_str()),
        (3, "unknown section `## checks`")
    );
    let names = "## workflows, ## actions, ## hand-written, ## bots";
    assert_eq!(e.fix, format!("sections are {names}"));
}

#[test]
fn duplicate_section_names_both_lines() {
    let e = refused(fixture!("duplicate-section"));
    assert_eq!(e.line, 7);
    assert_eq!(e.what, "`## bots` given twice, first on line 3");
    assert_eq!(e.fix, "merge them into one section");
}

#[test]
fn row_outside_section_is_refused() {
    let e = refused(fixture!("row-outside-section"));
    assert_eq!((e.line, e.column), (2, 1));
    assert_eq!(e.what, "row `ci|gate|-|-|-` is outside any section");
}

#[test]
fn a_section_may_be_empty_or_header_only() {
    let doc =
        parsed("format 1\n## workflows\n## actions\naction|version|sha\n");
    assert_eq!(doc.sections.len(), 2);
    assert!(doc.sections.iter().all(|s| s.rows.is_empty()));
}

#[test]
fn indented_lines_keep_their_columns() {
    let e = refused("  format 1\n  ## bots\n  bot|args\n    a|b|c\n");
    assert_eq!((e.line, e.column), (4, 5));
    let doc = parsed("format 1\n## bots\nbot|args\n  a | k=v\n");
    let row = doc.section("bots").first().cloned().unwrap_or_default();
    let columns: Vec<usize> = row.cells.iter().map(|c| c.column).collect();
    assert_eq!(columns, [3, 7]);
}

#[test]
fn a_missing_required_section_is_refused() {
    let paths = SectionSpec {
        required: true,
        ..spec("hand-written", &["path", "reason"])
    };
    let schema = Schema {
        sections: &[spec("bots", &["bot", "args"]), paths],
    };
    let e = parse(FILE, "\nformat 1\n## bots\n", &schema).err();
    let e = e.unwrap_or_default();
    assert_eq!((e.line, e.column), (2, 1));
    assert_eq!(e.what, "required section `## hand-written` is missing");
    assert_eq!(e.fix, "add `## hand-written` and its header `path|reason`");
    let text = "format 1\n## hand-written\npath|reason\n";
    assert!(parse(FILE, text, &schema).is_ok());
}

#[test]
fn odd_input_is_refused_not_panicked_on() {
    for text in ["\u{0}", "format 1\n##\n", "format 1\n## bots\n|", "\\|"] {
        assert!(parse(FILE, text, &ACTLITH).is_err(), "{text:?}");
    }
}
