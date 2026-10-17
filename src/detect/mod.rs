//! `src/detect`: proposed rows from tracked files and the first config
//! file.
//!
//! A forge crate lists its tracked paths and reads `flake.lock`, then
//! [`detect()`] proposes `## workflows` rows, [`text()`] writes them as a
//! registry file and [`seed_init`] writes that file once under the root,
//! never over an existing one. A forge that writes some rows its own
//! way (a note and a commented-out row, off until the user opts in)
//! passes its line writer to [`text_with`] and the result to
//! [`seed_init_text`]. Composition only (src C): the lock scan
//! lives in `lock`, the rows in `rows`, the text in `text`, the write in
//! `seed`. Pure except the seed writes (detect C).

mod lock;
mod rows;
mod seed;
mod text;

pub use lock::has_node;
pub use rows::detect;
pub use seed::{SeedError, seed_init, seed_init_text};
pub use text::{FIRST_ROW, header, row_line, text, text_with};
