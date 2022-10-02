use std::error::Error as OtherError;
use std::fmt::{self, Display};
use std::io;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    InvalidImageLength {
        size: usize,
        width: u32,
        height: u32,
        n_channels: usize,
    },
    InvalidImageDimension {
        width: u32,
        height: u32,
    },
    InvalidColorSpace {
        colorspace: u8,
    },
    InvalidChannels {
        channels: u8,
    },
    Other {
        message: String,
    },
}

impl Error {
    fn new(msg: String) -> Self {
        Self::Other {
            message: msg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidImageLength {
                size,
                width,
                height,
                n_channels,
            } => {
                write!(
                    f,
                    "Invalid image data of {} x {} x {} != {}",
                    width, height, n_channels, size
                )
            }
            Self::InvalidImageDimension { width, height } => {
                write!(f, "Invalid image dimensions of {} x {}", width, height)
            }
            Self::InvalidColorSpace { colorspace } => {
                write!(f, "Invalid color space: {}. Must be 0 or 1.", colorspace)
            }
            Self::InvalidChannels { channels } => {
                write!(
                    f,
                    "Invalid number of channels: {}. Must be 3 or 4.",
                    channels
                )
            }
            Self::Other { message } => {
                write!(f, "Other Error: {}.", message)
            }
        }
    }
}

impl OtherError for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::new(err.to_string())
    }
}
