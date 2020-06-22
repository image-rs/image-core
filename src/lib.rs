//! This crate provides the core types and traits required to encode and decode image files.

#![deny(missing_docs)]
#![deny(unused_extern_crates)]
#![forbid(unsafe_code)]

mod colortype;
mod decoder;
mod error;
mod format;

pub use colortype::*;
pub use decoder::*;
pub use error::*;
pub use format::ImageFormat;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Empty {}
