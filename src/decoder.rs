use crate::ColorType;
use std::error;
use std::io::{self, Read};


/// Represents the progress of an image operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    current: u64,
    total: u64,
}

/// Trait to decode an image file into dimensions, color type, and image data.
pub trait ImageDecoder<'a>: Sized {
    /// The type of reader produced by `into_reader`.
    type Reader: Read + 'a;

    /// Type of errors
    type Error: error::Error + From<io::Error>;

    /// Returns a tuple containing the width and height of the image.
    fn dimensions(&self) -> (u64, u64);

    /// Returns the color type of the image.
    fn colortype(&self) -> ColorType;

    /// Returns a reader that can be used to obtain the bytes of the image. For the best
    /// performance, always try to read at least `scanline_bytes` from the reader at a time. Reading
    /// fewer bytes will cause the reader to perform internal buffering.
    fn into_reader(self) -> Result<Self::Reader, Self::Error>;

    /// Returns the total number of bytes in the image.
    fn total_bytes(&self) -> u64 {
        let bits = self.dimensions().0 * self.dimensions().1 * self.colortype().bits_per_pixel();
        (bits + 7) / 8
    }

    /// Returns the minimum number of bytes that can be efficiently read from this decoder. This may
    /// be as few as 1 or as many as `total_bytes()`.
    fn scanline_bytes(&self) -> u64 {
        self.total_bytes()
    }

    /// Returns all the bytes in the image.
    fn read_image(self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.read_image_with_progress(buf, |_| {})
    }

    /// Same as `read_image` but periodically calls the provided callback to give updates on loading
    /// progress.
    fn read_image_with_progress<F: Fn(Progress)>(
        self,
        buf: &mut [u8],
        progress_callback: F,
    ) -> Result<(), Self::Error> {
        let total_bytes = self.total_bytes();
        assert_eq!(buf.len() as u64, total_bytes);

        let total_bytes = total_bytes as usize;
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

/// Decode sections of an image without loading the fully loading it first.
///
/// This is separate from the main `ImageDecoder` trait since some decoders may require a `io::Seek`
/// bound on their underlying reader to implement this functionality but not need it for normal
/// decoding operations.
pub trait ImageDecoderExt<'a>: ImageDecoder<'a> + Sized {
    /// Read a rectangular section of the image.
    fn read_rect(
        &mut self,
        x: u64,
        y: u64,
        width: u64,
        height: u64,
        buf: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.read_rect_with_progress(x, y, width, height, buf, |_|{})
    }

    /// Read a rectangular section of the image, periodically reporting progress.
    fn read_rect_with_progress<F: Fn(Progress)>(
        &mut self,
        x: u64,
        y: u64,
        width: u64,
        height: u64,
        buf: &mut [u8],
        progress_callback: F,
    ) -> Result<(), Self::Error>;
}
