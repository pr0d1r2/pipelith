use super::splice;

#[test]
fn only_the_named_block_is_replaced() {
    let doc = "# t\n\n<!-- BEGIN x: note -->\nold\n<!-- END x -->\n\nbody\n";
    assert_eq!(
        splice(doc, "x", "NEW\n").as_deref(),
        Some("# t\n\nNEW\n\nbody\n")
    );
}

#[test]
fn a_block_at_the_very_end_is_replaced() {
    let end = "<!-- BEGIN x -->\nold\n<!-- END x -->";
    assert_eq!(splice(end, "x", "NEW\n").as_deref(), Some("NEW\n"));
}

#[test]
fn a_missing_or_unclosed_block_is_none() {
    let doc = "<!-- BEGIN x -->\n<!-- END x -->\n";
    assert_eq!(splice(doc, "y", "x"), None);
    assert_eq!(splice("<!-- BEGIN x -->\nno end", "x", "x"), None);
}

/// Splicing twice is splicing once, so `--check` can be equality.
#[test]
fn splicing_is_idempotent() {
    let doc = "a\n<!-- BEGIN x -->\n<!-- END x -->\nb\n";
    let block = "<!-- BEGIN x: g -->\nv\n<!-- END x -->\n";
    let once = splice(doc, "x", block).unwrap_or_default();
    assert_eq!(splice(&once, "x", block), Some(once));
}
