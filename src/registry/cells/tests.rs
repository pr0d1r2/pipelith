use super::{Arg, Pos, args, header, list, optional, required, row, split};
use crate::registry::{Cell, Row, SectionSpec};

const BOTS: SectionSpec<'static> = SectionSpec {
    name: "bots",
    columns: &["bot", "args"],
    required: false,
    key: Some(0),
};

const PATHS: SectionSpec<'static> = SectionSpec {
    name: "hand-written",
    columns: &["path", "reason"],
    required: false,
    key: Some(0),
};

fn at(line: usize, column: usize) -> Pos<'static> {
    Pos {
        file: "x.reg",
        line,
        column,
    }
}

fn texts(row: &str) -> Vec<String> {
    split(row).into_iter().map(|(_, t)| t).collect()
}

/// A one-cell row at line 9 whose cell starts at column 4.
fn one(text: &str) -> Row {
    let cells = vec![Cell {
        column: 4,
        text: text.into(),
    }];
    Row { line: 9, cells }
}

fn arg(key: &str, value: &str) -> Arg {
    Arg {
        key: key.into(),
        value: value.into(),
    }
}

#[test]
fn a_row_splits_on_pipes_trimmed() {
    assert_eq!(texts(" a | b|c "), ["a", "b", "c"]);
    assert_eq!(texts("a"), ["a"]);
    assert_eq!(texts("a||"), ["a", "", ""]);
}

#[test]
fn each_cell_knows_where_its_text_starts() {
    let offsets: Vec<usize> = split("ab|  cd|-|").iter().map(|c| c.0).collect();
    assert_eq!(offsets, [0, 5, 8, 10]);
    let offsets: Vec<usize> = split(r"a\|b| c").iter().map(|c| c.0).collect();
    assert_eq!(offsets, [0, 6]);
}

#[test]
fn a_dash_is_an_empty_cell() {
    assert_eq!(texts("-|a-b| - "), ["", "a-b", ""]);
}

#[test]
fn an_escaped_pipe_is_literal() {
    assert_eq!(texts(r"a \| b|c\d"), ["a | b", r"c\d"]);
    assert_eq!(texts(r"x\"), [r"x\"]);
}

#[test]
fn a_hash_inside_a_cell_is_text() {
    assert_eq!(texts("a # b|#c"), ["a # b", "#c"]);
}

#[test]
fn the_cell_count_must_match_the_header() {
    let ok = row(at(4, 3), &PATHS, "p | r");
    let cell = |column, text: &str| Cell {
        column,
        text: text.into(),
    };
    let cells = vec![cell(3, "p"), cell(7, "r")];
    assert_eq!(ok, Ok(Row { line: 4, cells }));
    let e = row(at(4, 1), &PATHS, "p").err().unwrap_or_default();
    assert_eq!((e.file.as_str(), e.line, e.column), ("x.reg", 4, 1));
    assert_eq!(e.what, "1 cell where `## hand-written` rows have 2");
    assert!(e.fix.starts_with("path|reason: one cell"), "{}", e.fix);
    let e = row(at(4, 1), &PATHS, "p|r|x").err().unwrap_or_default();
    assert_eq!(e.what, "3 cells where `## hand-written` rows have 2");
}

#[test]
fn the_header_row_is_the_spec_one() {
    assert_eq!(header(at(2, 1), &BOTS, "bot | args"), Ok(()));
    let e = header(at(2, 1), &BOTS, "bot|arg").err();
    let e = e.unwrap_or_default();
    assert_eq!((e.line, e.column), (2, 1));
    assert_eq!(e.what, "`## bots` header is `bot|arg`");
    assert_eq!(e.fix, "start the section with `bot|args`");
}

#[test]
fn a_required_cell_is_not_empty() {
    assert_eq!(
        required("x.reg", &one("gate"), 0, "kind"),
        Ok("gate".into())
    );
    let e = required("x.reg", &one(""), 0, "kind").err();
    let e = e.unwrap_or_default();
    assert_eq!((e.file.as_str(), e.line, e.column), ("x.reg", 9, 4));
    assert_eq!(e.what, "`kind` is empty");
    assert_eq!(e.fix, "fill `kind`; `-` is only for optional cells");
}

#[test]
fn a_missing_required_cell_points_at_the_row() {
    let e = required("x.reg", &one("a"), 3, "kind").err();
    let e = e.unwrap_or_default();
    assert_eq!(
        (e.line, e.column, e.what.as_str()),
        (9, 1, "`kind` is empty")
    );
}

#[test]
fn an_optional_cell_is_none_when_empty() {
    assert_eq!(optional(&one("c"), 0), Some("c".into()));
    assert_eq!(optional(&one(""), 0), None);
    assert_eq!(optional(&one("c"), 1), None);
}

#[test]
fn a_list_splits_on_commas() {
    let got = list("x.reg", &one("x86_64-linux, aarch64-linux"), 0, "s");
    assert_eq!(got, Ok(vec!["x86_64-linux".into(), "aarch64-linux".into()]));
    assert_eq!(list("x.reg", &one(""), 0, "s"), Ok(vec![]));
    assert_eq!(list("x.reg", &one("a"), 1, "s"), Ok(vec![]));
}

#[test]
fn a_list_has_no_empty_item() {
    let e = list("x.reg", &one("a,,b"), 0, "systems").err();
    let e = e.unwrap_or_default();
    assert_eq!((e.line, e.column), (9, 4));
    assert_eq!(e.what, "`systems` has an empty item");
}

#[test]
fn args_are_key_value_pairs_in_order() {
    let cell = one("package=pipelith, cron=0 6 * * *, x=a=b");
    let got = args("x.reg", &cell, 0, "args");
    let want = vec![
        arg("package", "pipelith"),
        arg("cron", "0 6 * * *"),
        arg("x", "a=b"),
    ];
    assert_eq!(got, Ok(want));
    assert_eq!(args("x.reg", &one(""), 0, "args"), Ok(vec![]));
}

#[test]
fn an_arg_without_key_or_value_is_refused() {
    for bad in ["package", "=x", "package=", " = "] {
        let e = args("x.reg", &one(bad), 0, "args").err();
        let e = e.unwrap_or_default();
        assert_eq!((e.line, e.column), (9, 4), "{bad}");
        assert!(e.what.contains("is not `key=value`"), "{}", e.what);
    }
}

#[test]
fn an_arg_key_is_given_once() {
    let e = args("x.reg", &one("a=1, a=2"), 0, "args").err();
    let e = e.unwrap_or_default();
    assert_eq!((e.line, e.what.as_str()), (9, "arg `a` given twice"));
    assert_eq!(e.fix, "give each key once");
}

#[test]
fn a_list_error_in_args_is_kept() {
    let e = args("x.reg", &one("a=1,,b=2"), 0, "args").err();
    assert_eq!(e.unwrap_or_default().what, "`args` has an empty item");
}
