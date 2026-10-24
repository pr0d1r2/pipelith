//! `src/runner`: the `## runners` section, where a repository names its
//! own runner's labels for a nix system, over the forge's default.
//!
//! A forge crate puts [`RUNNERS`] in its registry schema, reads the
//! section with [`Runners::from_document`], and asks [`Runners::pick`]
//! for every system of every checked row while it validates, passing
//! whether its own runner table has a default for that system. A
//! [`Runner::Labels`] answer is the repository's own machine, so the
//! forge runs that job only for trusted events ([`Runner::trusted_only`]).
//! Composition only (src C): the section lives in `section`.

mod section;

pub use section::{COLUMNS, RUNNERS, Runner, Runners, SECTION};
