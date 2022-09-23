use crate::types::{ Channels, ColorSpace };
use crate::error::{ Error, Result };
use crate::consts::{
    QOI_HEADER_SIZE,
    QOI_MAGIC,
    QOI_PIXELS_MAX
};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    pub width: u32,
    pub height: u32,
    pub channels: Channels,
    pub colorspace: ColorSpace,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            channels: Channels::default(),
            colorspace: ColorSpace::default(),
        }
    }
}

impl Header {
    pub fn try_new(width: u32, height:u32 ,channels: Channels, colorspace:ColorSpace) -> Result<Self> {
        
        let px_len = (width as usize).saturating_mul(height as usize);
        if  px_len == 0 || px_len > QOI_PIXELS_MAX {
            return Err(Error::InvalidImageDimension{ width: width, height: height });
        }

        Ok(Self { 
            width, 
            height, 
            channels, 
            colorspace 
        })

    }

    pub fn encode(&self) -> [u8; QOI_HEADER_SIZE] {
        let mut out = [0; QOI_HEADER_SIZE];
        out[..4].copy_from_slice(&QOI_MAGIC.to_be_bytes());
        out[4..8].copy_from_slice(&self.width.to_be_bytes());
        out[8..12].copy_from_slice(&self.height.to_be_bytes());
        out[12] = self.channels.into();
        out[13] = self.colorspace.into();
        out
    }

    pub const fn n_pixels(&self) -> usize {
        (self.width as usize).saturating_mul(self.height as usize)
    }

    pub const fn n_bytes(&self) -> usize {
        self.n_bytes() * self.channels.as_u8() as usize
    }
}

