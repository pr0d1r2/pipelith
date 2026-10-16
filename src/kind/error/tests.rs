//! A kind error names the row's line, what and the fix (T14, V31).

use crate::kind::KindError;
use crate::registry::RegistryError;

#[test]
fn a_kind_error_names_its_line_and_fix() {
    let error = KindError::new(7, "arg `x` is unknown", "remove `x`");
    assert_eq!(error.line, 7);
    assert_eq!(error.to_string(), "line 7: arg `x` is unknown (remove `x`)");
}

#[test]
fn a_located_error_is_a_registry_error_at_the_row() {
    let error = KindError::new(7, "arg `x` is unknown", "remove `x`");
    let want = RegistryError {
        file: ".lablith".to_owned(),
        line: 7,
        column: 1,
        what: "arg `x` is unknown".to_owned(),
        fix: "remove `x`".to_owned(),
    };
    assert_eq!(error.located(".lablith"), want);
}
