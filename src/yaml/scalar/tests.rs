use super::{WIDTH, fold, inline, key, literal, quote};
use crate::yaml::tests::Github;
use crate::yaml::{Dialect, Expr, Text};

fn lit(s: &str) -> String {
    inline(&Github, &Text::lit(s), false)
}

fn expr(inner: &str) -> Result<Text, &'static str> {
    Expr::new(&Github, inner).map(Text::expr).ok_or("expr")
}

/// A dialect whose expressions start with a YAML indicator, end with a
/// colon, and whose escape turns a safe literal into an unsafe one.
struct Odd;

impl Dialect for Odd {
    fn expr(&self, inner: &str) -> Option<String> {
        Some(format!("*{inner}:"))
    }

    fn escape(&self, lit: &str) -> String {
        lit.replace('x', "#x")
    }
}

#[test]
fn safe_strings_stay_plain() {
    assert_eq!(lit("ubuntu-24.04-arm"), "ubuntu-24.04-arm");
    assert_eq!(lit("nix develop --command hk"), "nix develop --command hk");
    assert_eq!(lit("actions/checkout@3d3c"), "actions/checkout@3d3c");
    assert_eq!(lit("/abs"), "/abs");
    assert_eq!(lit("_x"), "_x");
}

#[test]
fn indicators_and_specials_are_single_quoted() {
    assert_eq!(lit("(-source$|x)"), "'(-source$|x)'");
    assert_eq!(lit("50 6 * * *"), "'50 6 * * *'");
    assert_eq!(lit(".#default"), "'.#default'");
    assert_eq!(lit("a: b"), "'a: b'");
    assert_eq!(lit("it's"), "'it''s'");
    assert_eq!(lit("-x"), "'-x'");
}

#[test]
fn numbers_and_reserved_words_are_quoted() {
    assert_eq!(lit("30"), "'30'");
    assert_eq!(lit("true"), "'true'");
    assert_eq!(lit("On"), "'On'");
    assert_eq!(lit("null"), "'null'");
}

#[test]
fn empty_and_edge_spaces_are_quoted() {
    assert_eq!(lit(""), "''");
    assert_eq!(lit("a "), "'a '");
    assert_eq!(lit(" a"), "' a'");
}

#[test]
fn control_chars_are_double_quoted_with_escapes() {
    assert_eq!(quote("a\nb\t\"\\\r"), r#""a\nb\t\"\\\r""#);
    assert_eq!(quote("\u{7}"), r#""\u0007""#);
}

#[test]
fn expressions_are_plain_in_block_quoted_in_flow() -> Result<(), &'static str> {
    let os = Expr::new(&Github, "matrix.os").ok_or("expr")?;
    let t = Text::lit("ci-").then_expr(os.clone());
    assert_eq!(inline(&Github, &t, false), "ci-${{ matrix.os }}");
    let e = Text::expr(os);
    assert_eq!(inline(&Github, &e, false), "${{ matrix.os }}");
    assert_eq!(inline(&Github, &e, true), "'${{ matrix.os }}'");
    Ok(())
}

#[test]
fn expression_with_colon_space_or_hash_is_quoted() -> Result<(), &'static str> {
    let colon = expr("format('a: b')")?;
    assert_eq!(inline(&Github, &colon, false), "'${{ format(''a: b'') }}'");
    let hash = expr("a || ' #'")?;
    assert_eq!(inline(&Github, &hash, false), "'${{ a || '' #'' }}'");
    Ok(())
}

#[test]
fn expression_whose_rendering_breaks_plain_is_quoted()
-> Result<(), &'static str> {
    assert_eq!(inline(&Github, &expr("a:")?, false), "'${{ a: }}'");
    assert_eq!(inline(&Github, &expr("#a")?, false), "'${{ #a }}'");
    let odd = Expr::new(&Odd, "a").map(Text::expr).ok_or("expr")?;
    assert_eq!(inline(&Odd, &odd, false), "'*a:'");
    Ok(())
}

#[test]
fn literal_is_judged_after_the_dialect_escapes_it() {
    assert_eq!(inline(&Odd, &Text::lit("ax"), false), "'a#x'");
    assert_eq!(inline(&Odd, &Text::lit("ab"), false), "ab");
}

#[test]
fn empty_text_is_quoted() {
    assert_eq!(inline(&Github, &Text::default(), false), "''");
}

#[test]
fn keys_plain_or_quoted() {
    assert_eq!(key("persist-credentials"), "persist-credentials");
    assert_eq!(key("on"), "'on'");
    assert_eq!(key("a b:"), "'a b:'");
    assert_eq!(key("a\nb"), r#""a\nb""#);
}

#[test]
fn literal_block_chomping() {
    assert_eq!(literal("a\nb\n"), Some(("|", vec!["a", "b"])));
    assert_eq!(literal("a\nb"), Some(("|-", vec!["a", "b"])));
    assert_eq!(literal("a\n\n"), Some(("|+", vec!["a", ""])));
    assert_eq!(literal("\na\n"), Some(("|", vec!["", "a"])));
}

#[test]
fn literal_block_refused_when_it_would_misread() {
    assert_eq!(literal("one line"), None);
    assert_eq!(literal(" a\nb\n"), None);
    assert_eq!(literal("\n\n"), None);
    assert_eq!(literal("a\tb\nc\n"), None);
}

#[test]
fn fold_breaks_after_logical_operators() {
    let raw = "${{ a == 'b' && c != d || e }}";
    let lines = fold(raw, 12);
    let want = ["${{ a == 'b' &&", "c != d ||", "e }}"];
    assert_eq!(lines, Some(want.map(String::from).to_vec()));
}

#[test]
fn fold_wraps_before_width() {
    let word = "w".repeat(60);
    let raw = format!("{word} {word} {word}");
    let lines = fold(&raw, 4).unwrap_or_default();
    assert_eq!(lines.len(), 3);
    assert!(lines.iter().all(|l| l.len().saturating_add(4) <= WIDTH));
}

#[test]
fn fold_ending_on_operator_leaves_no_empty_line() {
    assert_eq!(fold("a &&", 0), Some(vec!["a &&".to_owned()]));
}

#[test]
fn fold_refused_when_it_would_change_the_string() {
    assert_eq!(fold("word", 0), None);
    assert_eq!(fold("a  b", 0), None);
    assert_eq!(fold(" a b", 0), None);
    assert_eq!(fold("a b\nc", 0), None);
}
