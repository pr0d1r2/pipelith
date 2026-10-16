use super::{Pos, SUPPORTED_FORMAT, check};

fn at(line: usize) -> Pos<'static> {
    Pos {
        file: "x.reg",
        line,
        column: 1,
    }
}

#[test]
fn the_supported_format_is_1() {
    assert_eq!(SUPPORTED_FORMAT, 1);
}

#[test]
fn format_1_is_read() {
    assert_eq!(check(at(1), "format 1"), Ok(1));
    assert_eq!(check(at(3), "format  1 "), Ok(1));
}

#[test]
fn no_format_line_is_refused() {
    for first in ["", "## workflows", "formats 1"] {
        let e = check(at(2), first).err().unwrap_or_default();
        assert_eq!(e.what, "no `format` line", "{first:?}");
        assert_eq!(e.fix, "start the file with `format 1`");
        assert_eq!((e.file.as_str(), e.line, e.column), ("x.reg", 2, 1));
    }
}

#[test]
fn a_newer_format_names_both_versions() {
    let e = check(at(1), "format 2").err().unwrap_or_default();
    assert_eq!(e.line, 1);
    assert_eq!(e.what, "`x.reg` is format 2; this build reads format 1");
    assert_eq!(e.fix, "upgrade to a release that reads format 2");
}

#[test]
fn a_malformed_version_is_refused() {
    let huge = "format 99999999999999999999";
    for bad in ["format x", "format 0", "format", "format -1", huge] {
        let e = check(at(1), bad).err().unwrap_or_default();
        assert!(e.what.starts_with("unknown format"), "{bad}: {}", e.what);
        assert_eq!(e.fix, "write `format 1`");
    }
}
