//! The writer: a tree to bytes in a fixed layout (V23, V26): `---`,
//! two-space indentation, block style, `[a, b]` lists, comments on their
//! own lines or two spaces after a value, blank lines only between
//! top-level keys.

use super::node::{Entry, Key, Node, Value};
use super::scalar::{self, WIDTH};
use super::text::{Comment, Dialect, Text};

/// The document: `---`, the leading comments, then the top-level entries
/// with a blank line between them. Literal text is escaped by `dialect`.
#[must_use]
pub fn document(
    dialect: &dyn Dialect,
    comments: &[Comment],
    entries: &[Entry],
) -> String {
    let mut w = Writer {
        out: String::from("---\n"),
        dialect,
    };
    w.remarks(comments, 0);
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            w.out.push('\n');
        }
        w.member(entry, 0);
    }
    w.out
}

/// The bytes written so far and the dialect literal text passes through.
struct Writer<'d> {
    out: String,
    dialect: &'d dyn Dialect,
}

/// The start of an entry's first line, the comment that ends it, and the
/// column its block lines (nested keys, items, block scalars) start at.
struct Head<'a> {
    text: String,
    tail: Option<&'a Comment>,
    indent: usize,
}

impl Head<'_> {
    fn fits(&self, rest: &str) -> bool {
        let extra = self.tail.map_or(0, |c| c.inline().len().saturating_add(2));
        let len = self
            .text
            .chars()
            .count()
            .saturating_add(rest.chars().count());
        len.saturating_add(extra) <= WIDTH
    }
}

impl Writer<'_> {
    fn remarks(&mut self, comments: &[Comment], indent: usize) {
        for line in comments.iter().flat_map(Comment::lines) {
            self.push(&format!("{}{line}", pad(indent)), None);
        }
    }

    fn member(&mut self, entry: &Entry, indent: usize) {
        self.remarks(&entry.comments, indent);
        self.entry(entry, &pad(indent), indent);
    }

    fn map(&mut self, entries: &[Entry], indent: usize) {
        for entry in entries {
            self.member(entry, indent);
        }
    }

    fn item(&mut self, entries: &[Entry], indent: usize) {
        let Some((first, rest)) = entries.split_first() else {
            return self.push(&format!("{}- {{}}", pad(indent)), None);
        };
        self.remarks(&first.comments, indent);
        let inner = indent.saturating_add(2);
        self.entry(first, &format!("{}- ", pad(indent)), inner);
        self.map(rest, inner);
    }

    /// `entry` after `lead` (its indentation, and `- ` for a sequence
    /// item), its key at column `indent`.
    fn entry(&mut self, entry: &Entry, lead: &str, indent: usize) {
        let head = Head {
            text: format!("{lead}{}:", key(&entry.key)),
            tail: entry.trailing.as_ref(),
            indent: indent.saturating_add(2),
        };
        match &entry.node {
            Node::Value(v) => self.value(&head, v),
            Node::List(items) => self.list(&head, items),
            Node::Map(m) => self.nested(&head, m),
            Node::Seq(items) => self.seq(&head, items),
            Node::Null => self.head(&head, ""),
        }
    }

    /// A nested mapping; `{}` when empty.
    fn nested(&mut self, head: &Head, entries: &[Entry]) {
        if entries.is_empty() {
            return self.head(head, " {}");
        }
        self.head(head, "");
        self.map(entries, head.indent);
    }

    /// A block sequence; `[]` when empty.
    fn seq(&mut self, head: &Head, items: &[Vec<Entry>]) {
        if items.is_empty() {
            return self.head(head, " []");
        }
        self.head(head, "");
        for entries in items {
            self.item(entries, head.indent);
        }
    }

    /// `[a, b]` when it fits on the line, else one `- item` per line.
    fn list(&mut self, head: &Head, items: &[Text]) {
        let flow: Vec<String> =
            items.iter().map(|t| self.inline(t, true)).collect();
        let one = format!(" [{}]", flow.join(", "));
        if head.fits(&one) {
            return self.head(head, &one);
        }
        self.head(head, "");
        for text in items {
            let line =
                format!("{}- {}", pad(head.indent), self.inline(text, false));
            self.push(&line, None);
        }
    }

    fn value(&mut self, head: &Head, value: &Value) {
        match value {
            Value::Int(n) => self.head(head, &format!(" {n}")),
            Value::Bool(b) => self.head(head, &format!(" {b}")),
            Value::Text(t) => self.text(head, t),
        }
    }

    /// A string: a literal block for several lines, one line when it
    /// fits, else a folded block, else one long line.
    fn text(&mut self, head: &Head, text: &Text) {
        let raw = text.render(self.dialect);
        if let Some((style, lines)) = scalar::literal(&raw) {
            self.head(head, &format!(" {style}"));
            return self.block(&lines, head.indent);
        }
        let one = format!(" {}", self.inline(text, false));
        match scalar::fold(&raw, head.indent) {
            Some(lines) if !head.fits(&one) => {
                self.head(head, " >-");
                self.block(&lines, head.indent);
            }
            _ => self.head(head, &one),
        }
    }

    fn inline(&self, text: &Text, flow: bool) -> String {
        scalar::inline(self.dialect, text, flow)
    }

    /// The lines of a block scalar at `indent`; an empty line stays bare.
    fn block<S: AsRef<str>>(&mut self, lines: &[S], indent: usize) {
        for line in lines.iter().map(AsRef::as_ref) {
            if line.is_empty() {
                self.out.push('\n');
            } else {
                self.push(&format!("{}{line}", pad(indent)), None);
            }
        }
    }

    /// The first line of `head` with `rest` after it, then its tail.
    fn head(&mut self, head: &Head, rest: &str) {
        self.push(&format!("{}{rest}", head.text), head.tail);
    }

    /// One line: `line`, then `tail` two spaces after it.
    fn push(&mut self, line: &str, tail: Option<&Comment>) {
        self.out.push_str(line);
        if let Some(comment) = tail {
            self.out.push_str("  ");
            self.out.push_str(&comment.inline());
        }
        self.out.push('\n');
    }
}

fn key(key: &Key) -> String {
    match key {
        Key::Fixed(k) => (*k).to_owned(),
        Key::User(k) => scalar::key(k),
    }
}

fn pad(indent: usize) -> String {
    " ".repeat(indent)
}

#[cfg(test)]
mod tests;
