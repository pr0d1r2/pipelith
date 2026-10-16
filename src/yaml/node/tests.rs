use crate::yaml::{Comment, Entry, Key, Node, Value};

#[test]
fn fixed_entry_has_schema_key_and_no_comments() {
    let entry = Entry::fixed("name", Node::Null);
    assert_eq!(entry.key, Key::Fixed("name"));
    assert_eq!(entry.node, Node::Null);
    assert!(entry.comments.is_empty());
    assert_eq!(entry.trailing, None);
}

#[test]
fn user_entry_owns_the_callers_key() {
    let entry = Entry::user("on", Node::Value(Value::Int(1)));
    assert_eq!(entry.key, Key::User("on".to_owned()));
    assert_eq!(entry.node, Node::Value(Value::Int(1)));
    assert!(entry.comments.is_empty());
}

#[test]
fn comments_attach_to_the_entry() {
    let entry = Entry::fixed("k", Node::Null)
        .above(Comment::new("why"))
        .after(Comment::new("v1"));
    assert_eq!(entry.comments, [Comment::new("why")]);
    assert_eq!(entry.trailing, Some(Comment::new("v1")));
}
