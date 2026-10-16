//! The whole file: the `format` line, then `## <section>` tables checked
//! against the caller's schema; `#` comments and blank lines skipped
//! (registry C, V15, V17, V18).

use super::cells;
use super::document::{Document, Row, Section};
use super::error::{Pos, RegistryError};
use super::format;
use super::schema::{Schema, SectionSpec};
use super::unique::unique;

/// A content line: its number, the column of its first character, and
/// its text trimmed.
#[derive(Debug, Clone, Copy)]
struct Line<'a> {
    /// The 1-based line number.
    no: usize,
    /// The 1-based column of the first non-blank character.
    column: usize,
    /// The text, trimmed.
    text: &'a str,
}

/// A `## <section>` heading and the content lines under it.
struct Group<'a> {
    /// The heading line.
    head: Line<'a>,
    /// The name after `##`.
    name: &'a str,
    /// Each row, header first.
    rows: Vec<Line<'a>>,
}

/// The file name for errors and the schema to check against.
struct Ctx<'a> {
    /// The file name, as the caller gave it.
    file: &'a str,
    /// The sections the caller knows.
    schema: &'a Schema<'a>,
}

/// Parse registry `text` against `schema`. Pure: the caller reads the
/// file and passes its name, which every error carries (registry C).
///
/// # Errors
///
/// The first problem found, with its line, column and a fix: a missing
/// or newer `format`, an unknown or repeated section, a bad header, a
/// row with the wrong cell count, a repeated key, or a missing required
/// section.
pub fn parse(
    file: &str,
    text: &str,
    schema: &Schema<'_>,
) -> Result<Document, RegistryError> {
    let mut lines = content(text);
    let (at, first) = head(file, lines.next());
    let format = format::check(at, first)?;
    let ctx = Ctx { file, schema };
    let mut sections: Vec<Section> = Vec::new();
    for group in groups(file, lines)? {
        sections.push(ctx.section(&group, &sections)?);
    }
    ctx.complete(at, &sections)?;
    Ok(Document { format, sections })
}

impl<'a> Line<'a> {
    /// Line `no` with its text `raw`.
    fn new(no: usize, raw: &'a str) -> Self {
        let lead = raw.chars().take_while(|c| c.is_whitespace()).count();
        let column = lead.saturating_add(1);
        let text = raw.trim();
        Self { no, column, text }
    }

    /// Where the line's text starts.
    fn pos(self, file: &str) -> Pos<'_> {
        let (line, column) = (self.no, self.column);
        Pos { file, line, column }
    }
}

/// Where the first content line is and its text; an empty file points at
/// line 1, column 1.
fn head<'a>(file: &'a str, first: Option<Line<'a>>) -> (Pos<'a>, &'a str) {
    let top = Pos {
        file,
        line: 1,
        column: 1,
    };
    first.map_or((top, ""), |l| (l.pos(file), l.text))
}

/// Each line that is not blank or a `#` comment.
fn content(text: &str) -> impl Iterator<Item = Line<'_>> {
    (1..)
        .zip(text.lines())
        .map(|(no, raw)| Line::new(no, raw))
        .filter(|l| !l.text.is_empty() && !comment(l.text))
}

/// A `#` at the start of a line opens a comment; `##` is a heading.
fn comment(text: &str) -> bool {
    text.starts_with('#') && !text.starts_with("##")
}

/// Group content lines under their `##` heading.
fn groups<'a>(
    file: &str,
    lines: impl Iterator<Item = Line<'a>>,
) -> Result<Vec<Group<'a>>, RegistryError> {
    let mut out: Vec<Group<'a>> = Vec::new();
    for line in lines {
        if let Some(name) = line.text.strip_prefix("##") {
            let (head, name, rows) = (line, name.trim(), Vec::new());
            out.push(Group { head, name, rows });
        } else if let Some(last) = out.last_mut() {
            last.rows.push(line);
        } else {
            return Err(outside(line.pos(file), line.text));
        }
    }
    Ok(out)
}

/// A row above the first heading.
fn outside(at: Pos<'_>, text: &str) -> RegistryError {
    let what = format!("row `{text}` is outside any section");
    at.error(what, "put it under a `## <section>` heading")
}

impl<'a> Ctx<'a> {
    /// A heading's section: known, given once, header and rows checked.
    fn section(
        &self,
        group: &Group<'_>,
        seen: &[Section],
    ) -> Result<Section, RegistryError> {
        let spec = self.known(group, seen)?;
        let rows = self.rows(spec, &group.rows)?;
        unique(self.file, spec, &rows)?;
        let (name, line) = (spec.name.to_owned(), group.head.no);
        Ok(Section { name, line, rows })
    }

    /// The section a heading names, given once.
    fn known(
        &self,
        group: &Group<'_>,
        seen: &[Section],
    ) -> Result<&'a SectionSpec<'a>, RegistryError> {
        let at = group.head.pos(self.file);
        let Some(spec) = self.schema.spec(group.name) else {
            let what = format!("unknown section `## {}`", group.name);
            let fix = format!("sections are {}", self.schema.names());
            return Err(at.error(what, fix));
        };
        match seen.iter().find(|s| s.name == spec.name) {
            None => Ok(spec),
            Some(first) => Err(twice(at, spec, first.line)),
        }
    }

    /// The header row checked, then each data row split into cells.
    fn rows(
        &self,
        spec: &SectionSpec<'_>,
        lines: &[Line<'_>],
    ) -> Result<Vec<Row>, RegistryError> {
        let Some((head, body)) = lines.split_first() else {
            return Ok(Vec::new());
        };
        cells::header(head.pos(self.file), spec, head.text)?;
        let row = |l: &Line<'_>| cells::row(l.pos(self.file), spec, l.text);
        body.iter().map(row).collect()
    }

    /// Every required section is present; the error points at `at`.
    fn complete(
        &self,
        at: Pos<'_>,
        sections: &[Section],
    ) -> Result<(), RegistryError> {
        let present = |s: &&SectionSpec<'_>| {
            !s.required || sections.iter().any(|x| x.name == s.name)
        };
        let Some(spec) = self.schema.sections.iter().find(|s| !present(s))
        else {
            return Ok(());
        };
        let what = format!("required section `## {}` is missing", spec.name);
        let fix = format!(
            "add `## {}` and its header `{}`",
            spec.name,
            spec.header()
        );
        Err(at.error(what, fix))
    }
}

/// A section heading given a second time.
fn twice(at: Pos<'_>, spec: &SectionSpec<'_>, first: usize) -> RegistryError {
    let what = format!("`## {}` given twice, first on line {first}", spec.name);
    at.error(what, "merge them into one section")
}

#[cfg(test)]
mod tests;
