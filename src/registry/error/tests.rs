use super::{Pos, RegistryError};

fn at(line: usize, column: usize) -> Pos<'static> {
    Pos {
        file: "x.reg",
        line,
        column,
    }
}

#[test]
fn an_error_names_file_line_column_what_and_fix() {
    let e = at(3, 7).error("bad", "do better");
    let want = RegistryError {
        file: "x.reg".into(),
        line: 3,
        column: 7,
        what: "bad".into(),
        fix: "do better".into(),
    };
    assert_eq!(e, want);
    assert_eq!(e.to_string(), "x.reg:3:7: bad (do better)");
}

#[test]
fn an_error_is_a_std_error() {
    let e: Box<dyn std::error::Error> = Box::new(at(1, 1).error("w", "f"));
    assert_eq!(e.to_string(), "x.reg:1:1: w (f)");
}
