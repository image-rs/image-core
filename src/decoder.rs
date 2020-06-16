use crate::ImageResult;
use crate::{ColorType, ExtendedColorType};
use std::convert::TryFrom;
use std::io::Read;

/// Represents the progress of an image operation.
///
/// Note that this is not necessarily accurate and no change to the values passed to the progress
/// function during decoding will be considered breaking. A decoder could in theory report the
/// progress `(0, 0)` if progress is unknown, without violating the interface contract of the type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    /// A measure of completed decoding.
    pub current: u64,
    /// A measure of all necessary decoding work.
    pub total: u64,
}

impl Progress {
    /// A measure of completed decoding.
    pub fn current(self) -> u64 {
        self.current
    }

    /// A measure of all necessary decoding work.
    ///
    /// This is in general greater or equal than `current`.
    pub fn total(self) -> u64 {
        self.total
    }

    /// Calculate a measure for remaining decoding work.
    pub fn remaining(self) -> u64 {
        self.total.max(self.current) - self.current
    }
}

/// The trait that all decoders implement
pub trait ImageDecoder<'a>: Sized {
    /// The type of reader produced by `into_reader`.
    type Reader: Read + 'a;

    /// Returns a tuple containing the width and height of the image
    fn dimensions(&self) -> (u32, u32);

    /// Returns the color type of the image data produced by this decoder
    fn color_type(&self) -> ColorType;

    /// Retuns the color type of the image file before decoding
    fn original_color_type(&self) -> ExtendedColorType {
        self.color_type().into()
    }

    /// Returns a reader that can be used to obtain the bytes of the image. For the best
    /// performance, always try to read at least `scanline_bytes` from the reader at a time. Reading
    /// fewer bytes will cause the reader to perform internal buffering.
    fn into_reader(self) -> ImageResult<Self::Reader>;

    /// Returns the total number of bytes in the decoded image.
    ///
    /// This is the size of the buffer that must be passed to `read_image` or
    /// `read_image_with_progress`. The returned value may exceed usize::MAX, in
    /// which case it isn't actually possible to construct a buffer to decode all the image data
    /// into.
    fn total_bytes(&self) -> u64 {
        let dimensions = self.dimensions();
        u64::from(dimensions.0)
            * u64::from(dimensions.1)
            * u64::from(self.color_type().bytes_per_pixel())
    }

    /// Returns the minimum number of bytes that can be efficiently read from this decoder. This may
    /// be as few as 1 or as many as `total_bytes()`.
    fn scanline_bytes(&self) -> u64 {
        self.total_bytes()
    }

    /// Returns all the bytes in the image.
    ///
    /// This function takes a slice of bytes and writes the pixel data of the image into it.
    /// Although not required, for certain color types callers may want to pass buffers which are
    /// aligned to 2 or 4 byte boundaries to the slice can be cast to a [u16] or [u32]. To accommodate
    /// such casts, the returned contents will always be in native endian.
    ///
    /// # Panics
    ///
    /// This function panics if buf.len() != self.total_bytes().
    ///
    /// # Examples
    ///
    /// ```no_build
    /// use zerocopy::{AsBytes, FromBytes};
    /// fn read_16bit_image(decoder: impl ImageDecoder) -> Vec<16> {
    ///     let mut buf: Vec<u16> = vec![0; decoder.total_bytes()/2];
    ///     decoder.read_image(buf.as_bytes());
    ///     buf
    /// }
    fn read_image(self, buf: &mut [u8]) -> ImageResult<()> {
        self.read_image_with_progress(buf, |_| {})
    }

    /// Same as `read_image` but periodically calls the provided callback to give updates on loading
    /// progress.
    fn read_image_with_progress<F: Fn(Progress)>(
        self,
        buf: &mut [u8],
        progress_callback: F,
    ) -> ImageResult<()> {
        assert_eq!(u64::try_from(buf.len()), Ok(self.total_bytes()));

        let total_bytes = self.total_bytes() as usize;
        let scanline_bytes = self.scanline_bytes() as usize;
        let target_read_size = if scanline_bytes < 4096 {
            (4096 / scanline_bytes) * scanline_bytes
        } else {
            scanline_bytes
        };

        let mut reader = self.into_reader()?;

        let mut bytes_read = 0;
        while bytes_read < total_bytes {
            let read_size = target_read_size.min(total_bytes - bytes_read);
            reader.read_exact(&mut buf[bytes_read..][..read_size])?;
            bytes_read += read_size;

            progress_callback(Progress {
                current: bytes_read as u64,
                total: total_bytes as u64,
            });
        }

        Ok(())
    }
}

/// ImageDecoderExt trait
pub trait ImageDecoderExt<'a>: ImageDecoder<'a> + Sized {
    /// Read a rectangular section of the image.
    fn read_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        buf: &mut [u8],
    ) -> ImageResult<()> {
        self.read_rect_with_progress(x, y, width, height, buf, |_| {})
    }

    /// Read a rectangular section of the image, periodically reporting progress.
    fn read_rect_with_progress<F: Fn(Progress)>(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        buf: &mut [u8],
        progress_callback: F,
    ) -> ImageResult<()>;
}
