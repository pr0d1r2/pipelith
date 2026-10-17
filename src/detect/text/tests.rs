//! Row text round-trips through `src/registry` and `src/kind` (T16,
//! V37).

use super::{FIRST_ROW, header, row_line, text, text_with};
use crate::kind::{RawRow, SECTION, WORKFLOWS};
use crate::registry::{Arg, RegistryError, Schema, parse};

const FILE: &str = ".pipelith";

fn bare(line: usize, id: &str, kind: &str) -> RawRow {
    RawRow {
        line,
        id: id.to_owned(),
        kind: kind.to_owned(),
        ..RawRow::default()
    }
}

fn arg(key: &str, value: &str) -> Arg {
    Arg {
        key: key.to_owned(),
        value: value.to_owned(),
    }
}

fn full() -> RawRow {
    RawRow {
        systems: vec!["x86_64-linux".to_owned(), "aarch64-linux".to_owned()],
        cache: Some("pr0d1r2".to_owned()),
        args: vec![arg("command", "hk check | tee log"), arg("timeout", "30")],
        ..bare(FIRST_ROW, "ci", "gate")
    }
}

/// `text` parsed by the registry, each row typed by the kind node.
fn parsed(text: &str) -> Result<Vec<RawRow>, RegistryError> {
    let schema = Schema {
        sections: &[WORKFLOWS],
    };
    let doc = parse(FILE, text, &schema)?;
    let rows = doc.section(SECTION).iter();
    rows.map(|r| RawRow::from_cells(FILE, r)).collect()
}

#[test]
fn the_header_is_format_then_the_workflows_table() {
    let want = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";
    assert_eq!(header(), want);
    assert_eq!(header().lines().count(), 4);
    assert_eq!(FIRST_ROW, 5);
}

#[test]
fn an_empty_row_is_all_dashes() {
    assert_eq!(row_line(&bare(1, "ci", "gate")), "ci|gate|-|-|-");
}

#[test]
fn a_full_row_lists_and_escapes() {
    assert_eq!(
        row_line(&full()),
        "ci|gate|x86_64-linux, aarch64-linux|pr0d1r2|\
         command=hk check \\| tee log, timeout=30"
    );
}

#[test]
fn text_is_the_header_then_rows() {
    let rows = [bare(5, "ci", "gate"), bare(6, "update-pins", "update-pins")];
    let want = include_str!("../../../tests/fixtures/detect/both.actlith");
    assert_eq!(text(&rows), want);
    assert_eq!(text(&[]), header());
}

#[test]
fn text_parses_back_to_the_rows() {
    let rows = vec![full(), bare(6, "update-pins", "update-pins")];
    assert_eq!(parsed(&text(&rows)), Ok(rows));
    assert_eq!(parsed(&text(&[])), Ok(Vec::new()));
}

#[test]
fn text_with_row_line_is_text() {
    let rows = [full(), bare(6, "update-pins", "update-pins")];
    assert_eq!(text_with(&rows, row_line), text(&rows));
    assert_eq!(text_with(&[], row_line), header());
}

/// `update-pins` behind a note and commented out, as a forge proposes
/// it off by default; every other row as [`row_line`] writes it.
fn off_by_default(row: &RawRow) -> String {
    if row.kind == "update-pins" {
        return format!("# note\n# {}", row_line(row));
    }
    row_line(row)
}

#[test]
fn a_line_writer_can_comment_a_row_out() {
    let rows = [bare(5, "ci", "gate"), bare(6, "update-pins", "update-pins")];
    let written = text_with(&rows, off_by_default);
    let want = format!(
        "{}ci|gate|-|-|-\n# note\n# update-pins|update-pins|-|-|-\n",
        header()
    );
    assert_eq!(written, want);
    assert_eq!(parsed(&written), Ok(vec![bare(5, "ci", "gate")]));
}
