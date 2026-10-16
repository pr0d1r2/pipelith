//! `src/kind`: the neutral kind catalogue, row and argument validation,
//! secret purposes and the portability check.
//!
//! Scaffold: the catalogue lands in wave 2 (src/kind T14, T15).
#![allow(
    unused_imports,
    reason = "build-order edges for sherd wave until the real imports land"
)]

// Build-order edges for `sherd wave` (src C), one per line.
use crate::registry as _;
use crate::system as _;
