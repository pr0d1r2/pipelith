//! One generated file: where it goes and what it holds.

/// A generated file: its path under the root and its bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    /// Relative, `/`-separated path under the root, e.g. `ci/build.yml`.
    pub path: String,
    /// The whole file, header first.
    pub bytes: String,
}
