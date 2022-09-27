use crate::consts::*;
use crate::error::{Error, Result};
use crate::header::Header;
use crate::pixel::Pixel;
use crate::types::{Channels, ColorSpace};

#[allow(unused_assignments)]
fn encode_into<const N: usize>(data: &[u8], header: Header) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&header.encode());
    buf.reserve_exact(header.buf_max_len());

    let mut run: u8 = 0;
    let mut index = [Pixel::new(); 256];
    let mut px_prev = Pixel::new().with_a(0xff);
    let mut hash_prev = px_prev.hash_index();
    let mut px = Pixel::<N>::new().with_a(0xff);
    let px_end = data.len() / N;


    for (i, chunk) in data.chunks_exact(N).enumerate() {
        px.read(chunk);

        if px == px_prev {
            run += 1;
            if run == 62 || i == px_end - 1 {
                buf.push(QOI_OP_RUN | (run - 1));
                run = 0;
            }
        } else {
            if run > 0 {
                buf.push(QOI_OP_RUN | (run - 1));
                run = 0;
            }

            let px_rgba = px.from(0xff);
            hash_prev = px_rgba.hash_index();
            let px_index = &mut index[hash_prev as usize];

            if *px_index == px_rgba {
                buf.push(QOI_OP_INDEX | hash_prev);
            } else {
                *px_index = px_rgba;
                let out = px.encode(px_prev)?;
                buf.extend(out.into_iter());
            }

            px_prev = px;
        }
    }
    
    buf.extend_from_slice(&QOI_PADDING);
    buf.truncate(buf.len());

    Ok(buf)
}

pub struct Encoder<'a> {
    data: &'a [u8],
    header: Header,
}

impl<'a> Encoder<'a> {
    pub fn new(data: &'a [u8], width: u32, height: u32) -> Result<Self> {
        let mut header =
            Header::try_new(width, height, Channels::default(), ColorSpace::default())?;

        let size = data.len();
        let n_channels = size / header.n_pixels();
        if header.n_pixels() * n_channels != size {
            return Err(Error::InvalidImageLength {
                size,
                width,
                height,
                n_channels,
            });
        }

        header.channels = Channels::try_from(n_channels.min(0xff) as u8)?;
        Ok(Self { data, header })
    }

    pub fn encode(self) -> Result<Vec<u8>> {
        match self.header.channels {
            Channels::Rgb => encode_into::<3>(self.data, self.header),
            Channels::Rgba => encode_into::<4>(self.data, self.header),
        }
    }
}
