// pub fn grayscale_to_rgb(buf: &[u8]) -> Vec<u8> {
//     let mut out = Vec::with_capacity(buf.len() * 3);
//     for &px in buf {
//         for _ in 0..3 {
//             out.push(px);
//         }
//     }
//     out
// }

// pub fn grayscale_alpha_to_rgba(buf: &[u8]) -> Vec<u8> {
//     let mut out = Vec::with_capacity(buf.len() * 4);

//     for px in buf.chunks_exact(2) {
//         for _ in 0..3 {
//             out.push(px[0]);
//         }
//         out.push(px[1])
//     }

//     out
// }