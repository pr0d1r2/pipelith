//! `src/kind`: the neutral kind catalogue, row and argument validation,
//! secret purposes and the portability check.
//!
//! A forge crate puts [`WORKFLOWS`] in its registry schema, types each
//! row with [`RawRow::from_cells`], finds its kind with [`resolve`],
//! checks it with [`Row::check`] and builds its pipeline from the
//! checked [`Row`]; [`portable()`] says whether the row would render on
//! the other forge too. Composition only (src C): the catalogue lives in
//! `catalogue`, the section in `raw`, the checks in `args`, `cache` and
//! `row`, the forge's additions in `portable`.

mod args;
mod cache;
mod catalogue;
mod error;
mod portable;
mod raw;
mod row;

pub use args::{Args, parse_args};
pub use cache::Cache;
pub use catalogue::{
    ArgDefault, ArgSpec, BUILD, CATALOGUE, DEFAULT_TIMEOUT, GATE, KindSpec,
    MAX_TIMEOUT, Purpose, TIMEOUT, UPDATE_PINS, UPSTREAM, lookup,
};
pub use error::KindError;
pub use portable::{Extras, Finding, portable, resolve};
pub use raw::{COLUMNS, RawRow, SECTION, WORKFLOWS};
pub use row::Row;
