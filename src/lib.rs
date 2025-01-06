//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
//! [![Crates.io Version](https://img.shields.io/crates/v/vrt.svg)](https://crates.io/crates/vrt)
//! [![Github CI](https://github.com/littleairmada/vrt-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/littleairmada/vrt-rs/actions)
//! [![Minimum rustc version](https://img.shields.io/badge/rustc-1.76.0+-lightgray.svg)](#rust-version-requirements)
//!
//! # A VRT parser/encoder library for Rust
//!
//! A VITA Radio Transport (VITA 49.0-2015) parser and encoder, implemented with the [nom](https://github.com/Geal/nom)
//! parser combinator framework.
//!

#![no_std]
#![deny(
    missing_docs,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unreachable_pub
)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, missing_debug_implementations)]
// pragmas for doc
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

mod error;
mod types;

pub use error::Error;
pub use types::*;

pub use nom_derive::Parse;
