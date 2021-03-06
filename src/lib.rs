//! Utilities for working with GDNative projects.
//!
//! This crate can be used in cargo build scripts to automatically generate
//! `.gdnlib` and `.gdns` files for the current project.
//!
//! It currently does this by scanning the project sources for types that derive
//! `NativeClass` and generates one `.gdns` file for each type.

mod generate;
mod scan;

pub use generate::Builder as Generator;
pub use generate::{BuildMode, LibFormat};
pub use scan::{scan_crate, Classes};
