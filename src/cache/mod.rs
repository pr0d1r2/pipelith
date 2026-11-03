//! `src/cache`: the `## caches` section, where a repository names any
//! nix binary cache a `## workflows` row's `cache` cell may use, by kind
//! (`cachix`, `https`), with the built-in cachix table as the fallback.
//!
//! A forge crate puts [`CACHES`] in its registry schema, reads the
//! section with [`Caches::from_document`] and passes it to
//! `kind::Row::check_with`, so a row naming a missing cache is refused
//! while it is validated. The [`BinaryCache`] that returns says what to
//! read ([`BinaryCache::nix_settings`], [`BinaryCache::read_secrets`])
//! and how to push ([`BinaryCache::push`]); a cell may name a secret
//! instead of a literal ([`CacheValue::Secret`]), which the forge
//! injects its own way. `kind::Cache`, the built-in cachix cache of
//! 0.2.0, is unchanged and never holds a `## caches` row. Composition only (src C): the values
//! live in `value`, a cache and the built-in table in `entry`, the
//! section in `section`.

mod entry;
mod section;
mod value;

pub(crate) use entry::builtin_key;
pub use entry::{BinaryCache, CacheKind, Push, builtin, builtin_names};
pub use section::{CACHES, COLUMNS, Caches, SECTION};
pub use value::{CacheValue, SECRET_PREFIX};
