//! mail-read-emlx: Apple Mail integration via AppleScript
//!
//! This crate provides reliable Apple Mail integration using AppleScript.
//! All functionality works without special permissions or Full Disk Access.

pub mod applescript;
pub mod error;

pub use applescript::*;
pub use error::*;
