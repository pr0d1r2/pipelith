//! What a parse yields: the format, then sections of rows of cells, each
//! with the place it was written (registry I).

/// A parsed registry file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    /// The version on the `format` line.
    pub format: u32,
    /// The sections present, in file order.
    pub sections: Vec<Section>,
}

/// One `## <name>` section as written.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Section {
    /// The name after `## `.
    pub name: String,
    /// The 1-based line of the heading.
    pub line: usize,
    /// The data rows, header row excluded.
    pub rows: Vec<Row>,
}

/// One data row: one cell per schema column.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Row {
    /// The 1-based line the row is on.
    pub line: usize,
    /// The cells, in column order.
    pub cells: Vec<Cell>,
}

/// One cell: trimmed, `\|` made `|`, `-` made empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cell {
    /// The 1-based column, in characters, where the cell's text starts.
    pub column: usize,
    /// The cell's text.
    pub text: String,
}

impl Document {
    /// The rows of the section `name`; a section not in the file has none.
    #[must_use]
    pub fn section(&self, _name: &str) -> &[Row] {
        self.sections.first().map_or(&[], |s| s.rows.as_slice())
    }
}

#[cfg(test)]
mod tests;
