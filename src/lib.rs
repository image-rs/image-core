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

/// A marker struct for __NonExhaustive enums.
///
/// This is an empty type that can not be constructed. When an enum contains a tuple variant that
/// includes this type the optimizer can statically determined tha the branch is never taken while
/// at the same time the matching of the branch is required.
///
/// The effect is thus very similar to the actual `#[non_exhaustive]` attribute with no runtime
/// costs. Also note that we use a dirty trick to not only hide this type from the doc but make it
/// inaccessible. The visibility in this module is pub but the module itself is not and the
/// top-level crate never exports the type.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonExhaustiveMarker {
    /// Allows this crate, and this crate only, to match on the impossibility of this variant.
    pub(crate) _private: Empty,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Empty {}
