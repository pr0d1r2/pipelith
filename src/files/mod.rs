//! `src/files`: the generated-file set: header, write and drift check.
//!
//! Composition only (src C): the file type lives in `file`, the header
//! line in `header`, where a path lands under the root in `target`, the
//! write in `write` and the compare in `stale`. The only node that touches
//! disk, and only at the paths its caller lists, under the root.

mod file;
mod header;
mod stale;
mod target;
mod write;

pub use file::File;
pub use header::header;
pub use stale::{Stale, check};
pub use target::target;
pub use write::write;
