//! `src/yaml`: the ordered YAML model and writer. Malformed YAML and an
//! injected expression are not representable: a forge crate builds a
//! tree of [`Node`]s, and [`document`] prints it with one quoting rule
//! and a fixed layout. Expression syntax comes from the forge crate
//! through a [`Dialect`]; pipelith ships none (V25).

mod node;
mod render;
mod scalar;
mod text;

#[cfg(test)]
mod tests;

pub use node::{Entry, Key, Node, Value};
pub use render::document;
pub use scalar::{WIDTH, key, quote};
pub use text::{Comment, Dialect, Expr, Part, Text};
