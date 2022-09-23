use crate::error::{ Error, Result };

/// The colorspace is purely informative.
/// They do not change the way data chunks are encoded.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum ColorSpace {
    Srgb = 0,
    Linear = 1,
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::Srgb
    }
}

// From<T> for U
// Into<U> for T
impl From<ColorSpace> for u8 {
    fn from(colorspace: ColorSpace) -> Self {
        colorspace as Self
    }
}

impl TryFrom<u8> for ColorSpace {
    type Error = Error;

    fn try_from(colorspace: u8) -> Result<Self> {
        match colorspace {
            0 => Ok(Self::Srgb),
            1 => Ok(Self::Linear),
            _ => Err(Error::InvalidColorSpace{ colorspace }),
        }
    }

}

/// Number of 8-bit channels in a pixel.
/// The channel is purely informative.
/// They do not change the way data chunks are encoded.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Channels {
    Rgb = 3,
    Rgba = 4,
}

impl Channels {

    /// Converts to an 8-bit integer
    /// 3 if RGB, 4 if RGBA
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self::Rgb
    }
}

// From<T> for U
// Into<U> for T
impl From<Channels> for u8 {
    fn from(channels: Channels) -> Self {
        channels as Self
    }
}

impl TryFrom<u8> for Channels {
    type Error = Error;

    fn try_from(channels: u8) -> Result<Self> {
        match channels {
            3 => Ok(Self::Rgb),
            4 => Ok(Self::Rgba),
            _ => Err(Error::InvalidChannels { channels }),
        }
    }
}