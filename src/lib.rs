//! This crate provides the core types and traits required to encode and decode image files.

#![deny(missing_docs)]
#![deny(unused_extern_crates)]
#![forbid(unsafe_code)]

mod colortype;
mod decoder;
mod error;

pub use colortype::*;
pub use decoder::*;
pub use error::*;
