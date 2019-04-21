
/// An enumeration over supported color types and bit depths
#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
pub enum ColorType {
    /// Pixel is 1-bit luminance
    L1,

    /// Pixel is 8-bit luminance
    L8,
    /// Pixel is 8-bit luminance with an alpha channel
    LA8,
    /// Pixel contains 8-bit R, G and B channels
    RGB8,
    /// Pixel is 8-bit RGB with an alpha channel
    RGBA8,

    /// Pixel is 16-bit luminance
    L16,
    /// Pixel is 16-bit luminance with an alpha channel
    LA16,
    /// Pixel is 16-bit RGB
    RGB16,
    /// Pixel is 16-bit RGBA
    RGBA16,

    /// Pixel contains 8-bit B, G and R channels
    BGR8,
    /// Pixel is 8-bit BGR with an alpha channel
    BGRA8,

    /// Pixel is of unknown color type with the specified bit depth. This can apply to pixels which
    /// are associated with an external palette. In that case, the pixel value is an index into the
    /// palette.
    Unknown(u8),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl ColorType {
    /// Returns the number of bits contained in a single pixel.
    pub fn bits_per_pixel(&self) -> u64 {
        match *self {
            ColorType::L1 => 1,
            ColorType::L8 => 8,
            ColorType::L16 | ColorType::LA8 => 16,
            ColorType::RGB8 | ColorType::BGR8 => 24,
            ColorType::RGBA8 | ColorType::BGRA8 | ColorType::LA16 => 32,
            ColorType::RGB16 => 48,
            ColorType::RGBA16 => 64,
            ColorType::Unknown(n) => n as u64,
            ColorType::__Nonexhaustive => unreachable!(),
        }
    }
}
