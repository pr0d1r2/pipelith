use super::{Cell, Document, Row, Section};

fn row(line: usize, text: &str) -> Row {
    let cells = vec![Cell {
        column: 1,
        text: text.into(),
    }];
    Row { line, cells }
}

#[test]
fn a_section_is_found_by_its_name() {
    let rows = vec![row(4, "a"), row(5, "b")];
    let bots = Section {
        name: "bots".into(),
        line: 3,
        rows: rows.clone(),
    };
    let doc = Document {
        format: 1,
        sections: vec![bots],
    };
    assert_eq!(doc.section("bots"), rows.as_slice());
}

#[test]
fn an_absent_section_has_no_rows() {
    let doc = Document::default();
    assert!(doc.section("bots").is_empty());
}
