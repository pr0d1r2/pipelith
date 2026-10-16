use super::unique;
use crate::registry::{Cell, Row, SectionSpec};

const BOTS: SectionSpec<'static> = SectionSpec {
    name: "bots",
    columns: &["bot", "args"],
    required: false,
    key: Some(0),
};

fn row(line: usize, key: &str) -> Row {
    let cell = |column, text: &str| Cell {
        column,
        text: text.into(),
    };
    let cells = vec![cell(3, key), cell(9, "")];
    Row { line, cells }
}

#[test]
fn distinct_keys_pass() {
    let rows = [row(3, "socket"), row(4, "coderabbit")];
    assert_eq!(unique("x.reg", &BOTS, &rows), Ok(()));
}

#[test]
fn a_duplicate_key_names_both_lines() {
    let rows = [row(3, "socket"), row(5, "a"), row(8, "socket")];
    let e = unique("x.reg", &BOTS, &rows).err().unwrap_or_default();
    assert_eq!((e.file.as_str(), e.line, e.column), ("x.reg", 8, 3));
    assert_eq!(e.what, "bot `socket` given twice, first on line 3");
    assert_eq!(e.fix, "keep one row per key");
}

#[test]
fn a_section_without_a_key_may_repeat() {
    let spec = SectionSpec { key: None, ..BOTS };
    let rows = [row(3, "socket"), row(4, "socket")];
    assert_eq!(unique("x.reg", &spec, &rows), Ok(()));
}

#[test]
fn a_key_past_the_cells_is_never_repeated() {
    let spec = SectionSpec {
        key: Some(5),
        ..BOTS
    };
    let rows = [row(3, "socket"), row(4, "socket")];
    assert_eq!(unique("x.reg", &spec, &rows), Ok(()));
}
