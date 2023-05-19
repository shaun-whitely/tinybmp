use std::fmt::Display;

#[derive(Debug)]
pub enum ConversionError {
    TooManyColors,
    TooWide(u32),
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ConversionError::TooManyColors =>
                write!(f, "source image has more than 4 colors"),
            ConversionError::TooWide(width) =>
                write!(f, "source image is more than 255 pixels wide: {}", width),
        }
    }
}

impl std::error::Error for ConversionError {}

#[derive(Debug)]
pub enum ReadError {
    UnrecognizedFormat,
    UnsupportedColorDepth(u8),
    IoError(std::io::Error),
}

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::UnrecognizedFormat =>
                write!(f, "the image is in an unrecognized format"),
            ReadError::UnsupportedColorDepth(depth) =>
                write!(f, "the image has an unsupported color depth: {}", depth),
            ReadError::IoError(error) =>
                write!(f, "an I/O error occurred whilst reading the image: {}", error),
        }
    }
}

impl std::error::Error for ReadError {}

impl From<std::io::Error> for ReadError {
    fn from(value: std::io::Error) -> Self {
        ReadError::IoError(value)
    }
}
