//! Value types of the model: the dialect, an expression, a text made of
//! literal and expression parts, and a comment (V25). None of them can
//! carry a broken expression: an expression is checked by the dialect
//! when built, and every literal part is escaped by the dialect when
//! written, so literal text never reads as an expression.

/// A forge's expression syntax, supplied by the forge crate (V25).
/// pipelith ships none; the writer only calls through this trait.
pub trait Dialect {
    /// The expression `inner` as written in a pipeline file, or `None`
    /// when `inner` would end or nest the expression early (an injected
    /// expression). `inner` arrives trimmed, non-empty and free of
    /// control characters.
    fn expr(&self, inner: &str) -> Option<String>;

    /// `lit` written so the forge reads it back as those characters:
    /// any text that would open an expression is neutralized.
    fn escape(&self, lit: &str) -> String;
}

/// An expression, checked and rendered by a [`Dialect`] when built.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    inner: String,
    text: String,
}

impl Expr {
    /// The expression `inner`, trimmed, as `dialect` writes it. `None`
    /// when it is empty, holds a control character, or `dialect`
    /// refuses it.
    pub fn new(dialect: &dyn Dialect, inner: &str) -> Option<Self> {
        let _ = (dialect, inner);
        None
    }

    /// The expression as it is written in a pipeline file.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The expression without its delimiters, as it was given (trimmed).
    #[must_use]
    pub fn inner(&self) -> &str {
        &self.inner
    }
}

/// One part of a [`Text`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Part {
    /// Characters taken as they are; escaped by the dialect on write.
    Lit(String),
    /// An expression the forge evaluates.
    Expr(Expr),
}

/// A string value: literal parts and expression parts, in order. Two
/// literal parts are never adjacent: a literal after a literal joins
/// it, so the dialect sees (and escapes) the whole run at once.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Text(Vec<Part>);

impl Text {
    /// A text of one literal part.
    #[must_use]
    pub fn lit(s: &str) -> Self {
        let _ = s;
        Self::default()
    }

    /// A text of one expression.
    #[must_use]
    pub fn expr(e: Expr) -> Self {
        drop(e);
        Self::default()
    }

    /// This text followed by the literal `s`.
    #[must_use]
    pub fn then_lit(self, s: &str) -> Self {
        let _ = s;
        self
    }

    /// This text followed by the expression `e`.
    #[must_use]
    pub fn then_expr(self, e: Expr) -> Self {
        drop(e);
        self
    }

    /// The parts, in order.
    #[must_use]
    pub fn parts(&self) -> &[Part] {
        &self.0
    }

    /// The text as written in a pipeline file, before any YAML quoting:
    /// literal parts escaped by `dialect`, expressions as built.
    #[must_use]
    pub fn render(&self, dialect: &dyn Dialect) -> String {
        let _ = dialect;
        String::new()
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::lit(s)
    }
}

/// A comment of one or more lines. A control character other than a line
/// break becomes a space, and trailing spaces are dropped, so a comment
/// never ends its line early or leaves trailing whitespace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment(Vec<String>);

impl Comment {
    /// A comment of the lines of `text`.
    #[must_use]
    pub fn new(text: &str) -> Self {
        let _ = text;
        Self(Vec::new())
    }

    /// The comment's lines, each as written: `# text`, or `#` when empty.
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        self.0.clone()
    }

    /// The comment on one line, its lines joined by a space: the form a
    /// comment takes after a value.
    #[must_use]
    pub fn inline(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests;
