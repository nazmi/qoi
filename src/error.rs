use std::fmt::{self , Display};

pub enum Error {
    InvalidImageDimension{width: u32, height: u32},
    InvalidColorSpace { colorspace: u8},
    InvalidChannels { channels: u8},
}

pub type Result<T> = core::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidImageDimension { width, height } => {
                write!(f, "Invalid image dimensions of {} x {}", width, height)
            },
            Self::InvalidColorSpace { colorspace } => {
                write!(f, "Invalid color space: {}. Must be 0 or 1.", colorspace)
            },
            Self::InvalidChannels { channels } => {
                write!(f, "Invalid number of channels: {}. Must be 3 or 4.", channels)
            },
        }
    }
}