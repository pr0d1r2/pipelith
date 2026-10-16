//! The GitHub-shaped test dialect the yaml unit tests share (V25).
//! pipelith ships no dialect; this one exists to replay actlith's bytes,
//! so it spells GitHub's syntax exactly as actlith's `text::Expr` did.

use super::Dialect;

/// GitHub Actions expressions as actlith wrote them at `7ce69c6`.
pub(crate) struct Github;

impl Dialect for Github {
    fn expr(&self, inner: &str) -> Option<String> {
        let broken = inner.contains("${{") || inner.contains("}}");
        (!broken).then(|| format!("${{{{ {inner} }}}}"))
    }

    fn escape(&self, lit: &str) -> String {
        lit.replace("${{", "${{ '${{' }}")
    }
}

#[test]
fn test_dialect_wraps_and_refuses_delimiters() {
    assert_eq!(Github.expr("a").as_deref(), Some("${{ a }}"));
    assert_eq!(Github.expr("a }}"), None);
    assert_eq!(Github.expr("${{ a"), None);
}

#[test]
fn test_dialect_escapes_an_opening() {
    assert_eq!(Github.escape("x ${{ y"), "x ${{ '${{' }} y");
    assert_eq!(Github.escape("plain"), "plain");
}
