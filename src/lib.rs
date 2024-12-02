// Enforce strict linting at the highest level
#![forbid(unsafe_code)] // Completely forbid unsafe code
#![deny(warnings)] // Treat warnings as errors

// Enable Clippy's pedantic lints and other useful ones
#![warn(
    clippy::pedantic,           // Enable Clippy's pedantic lints
    clippy::nursery,            // Lints that are still in development but useful
    clippy::cargo,              // Cargo-specific lints
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms,           // Catch common issues with Rust 2018 idioms
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
// Disable overly verbose or less relevant pedantic lints
#![allow(
    clippy::module_name_repetitions, // Sometimes too strict on module naming
    clippy::doc_markdown,            // Avoids warnings for common Markdown syntax
    clippy::missing_panics_doc,      // Panics documentation can be verbose
    clippy::cargo_common_metadata
)]

pub mod day01;
pub mod day02;
