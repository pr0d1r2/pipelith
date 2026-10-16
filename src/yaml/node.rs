//! The YAML tree a forge crate lowers its pipeline model to (V23, V26):
//! scalars, flow lists, ordered mappings and sequences of mappings, with
//! comments attached to entries. Order is insertion order; the caller
//! sorts.

use super::text::{Comment, Text};

/// A scalar value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// A string, styled by the one quoting rule (V24).
    Text(Text),
    /// A whole number.
    Int(u64),
    /// `true` or `false`.
    Bool(bool),
}

/// A mapping key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    /// A key of the forge's schema, written as it is.
    Fixed(&'static str),
    /// A key from the caller (an input, a variable, a job id), quoted when
    /// it could not be read back plain.
    User(String),
}

/// A node of the tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    /// A scalar.
    Value(Value),
    /// A list of strings, `[a, b]` when it fits on its line.
    List(Vec<Text>),
    /// A mapping, in order.
    Map(Vec<Entry>),
    /// A block sequence of mappings.
    Seq(Vec<Vec<Entry>>),
    /// No value: `key:`.
    Null,
}

/// A mapping entry with the comments above it and after it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Comments on their own lines above the key.
    pub comments: Vec<Comment>,
    /// The key.
    pub key: Key,
    /// The value.
    pub node: Node,
    /// A comment two spaces after the entry's first line.
    pub trailing: Option<Comment>,
}

impl Entry {
    /// An entry with a schema key and no comments.
    #[must_use]
    pub fn fixed(key: &'static str, node: Node) -> Self {
        let _ = key;
        Self::new(Key::Fixed(""), node)
    }

    /// An entry with a caller's key and no comments.
    #[must_use]
    pub fn user(key: &str, node: Node) -> Self {
        let _ = key;
        Self::new(Key::Fixed(""), node)
    }

    /// This entry with `comment` added above it.
    #[must_use]
    pub fn above(self, comment: Comment) -> Self {
        drop(comment);
        self
    }

    /// This entry with `comment` after its first line.
    #[must_use]
    pub fn after(self, comment: Comment) -> Self {
        drop(comment);
        self
    }

    fn new(key: Key, node: Node) -> Self {
        Self {
            comments: Vec::new(),
            key,
            node,
            trailing: None,
        }
    }
}

#[cfg(test)]
mod tests;
