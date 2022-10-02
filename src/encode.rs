use crate::consts::*;
use crate::error::{Error, Result};
use crate::header::Header;
use crate::pixel::Pixel;
use crate::types::{Channels, ColorSpace};

#[allow(unused_assignments)]
fn encode_with_n<const N: usize>(buf:&mut [u8], data: &[u8]) -> Result<usize> {

    let mut buf_index = 0;
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
                buf[buf_index] = QOI_OP_RUN | (run - 1);
                buf_index += 1;
                run = 0;
            }
        } else {
            if run > 0 {
                buf[buf_index] = QOI_OP_RUN | (run - 1);
                buf_index += 1;
                run = 0;
            }

            let px_rgba = px.from_a(0xff);
            hash_prev = px_rgba.hash_index();
            let px_index = &mut index[hash_prev as usize];

            if *px_index == px_rgba {
                buf[buf_index] = QOI_OP_INDEX | hash_prev;
                buf_index += 1;
            } else {
                *px_index = px_rgba;
                let out = px.encode(px_prev)?;
                for item in out {
                    buf[buf_index] = item;
                    buf_index += 1;
                }
            }
            px_prev = px;
        }
        
    }
    
    
    for i in 0..8 {
        buf[buf_index] = QOI_PADDING[i];
        buf_index += 1;
    }
    
    
    Ok(buf_index + QOI_HEADER_SIZE)
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

    pub fn encode(&self, out: &mut [u8]) -> Result<usize> {
        match self.header.channels {
            Channels::Rgb => encode_with_n::<3>(out, self.data),
            Channels::Rgba => encode_with_n::<4>(out, self.data),
        }
    }

    pub fn encode_to_vec(&self) -> Result<Vec<u8>> {
        let mut out = vec![0_u8; self.header.buf_max_len()];
        let (head, tail) = out.split_at_mut(QOI_HEADER_SIZE);
        head.copy_from_slice(&self.header.encode());
        
        let size = self.encode(tail)?;
        out.truncate(size);
        
        Ok(out)


    }
}
