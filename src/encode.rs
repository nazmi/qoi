
use crate::pixel::Pixel;
use crate::header::Header;
use crate::consts::*;

pub fn encode<const N: usize>(data: &[u8], header: Header) -> Option<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&header.encode());

    let mut run: u8 = 0;
    let mut index = [Pixel::new(); 256];
    let mut px_prev = Pixel::<N>::new().with_a(0xff);
    let mut hash_prev = px_prev.hash_index();
    let mut px = Pixel::<N>::new().with_a(0xff);
    let px_end = data.len() / 4;

    for (i, chunk) in data.chunks_exact(4).enumerate() {
        px.read(chunk);
        if px == px_prev {
            run += 1;
            if run == 62 || i == px_end - 1 {
                buf.push(QOI_OP_RUN | (run - 1) );
                run = 0;
            }
        }else{

            if run > 0 {
                buf.push(QOI_OP_RUN | (run - 1) );
                run = 0;
            }

            let index = px.hash_index();
            
        }
    }
    
    Some(buf)
}

