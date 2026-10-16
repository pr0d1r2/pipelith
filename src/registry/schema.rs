//! The caller's schema: the sections a file may hold, the header each
//! one carries, which are required and which column is a key (registry
//! I, C). It replaces actlith's fixed `Table` enum.

/// The sections a registry file may hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Schema<'a> {
    /// Every known section, in the order an error hint lists them.
    pub sections: &'a [SectionSpec<'a>],
}

/// One `## <name>` section and the header row it starts with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionSpec<'a> {
    /// The name after `## `.
    pub name: &'a str,
    /// The header row's cells, in order; every row has one cell each.
    pub columns: &'a [&'a str],
    /// A file without this section is refused; otherwise it is empty.
    pub required: bool,
    /// The index of the column whose value is unique across the rows.
    pub key: Option<usize>,
}

impl<'a> Schema<'a> {
    /// The section a heading names, if any.
    #[must_use]
    pub(crate) fn spec(&self, _name: &str) -> Option<&'a SectionSpec<'a>> {
        self.sections.first()
    }

    /// The section names, as a fix hint lists them.
    #[must_use]
    pub(crate) fn names(&self) -> String {
        let names: Vec<&str> = self.sections.iter().map(|s| s.name).collect();
        names.concat()
    }
}

impl SectionSpec<'_> {
    /// The header row: the columns joined by `|`.
    #[must_use]
    pub fn header(&self) -> String {
        self.columns.concat()
    }
}

#[cfg(test)]
mod tests;
