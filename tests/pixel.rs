mod common;


use qoi::{*, consts::*};

fn test_chunk<const N:usize>(data: impl AsRef<[[u8; N]]>, expected: impl AsRef<[u8]>) {

    let data = data.as_ref();
    let mut pixels = vec![];
    for chunk in data {
        for i in 0..N {
            pixels.push(chunk[i]);
        }
    }
    let expected = expected.as_ref();
    let encoder = Encoder::new(pixels.as_slice(), data.len() as u32, 1).unwrap();
    
    let encoded = encoder.encode_to_vec().unwrap();
   // println!("{:?}",encoded);
    assert_eq!(&encoded[QOI_HEADER_SIZE..][..expected.len()], expected);
    assert_eq!(encoded.len(), expected.len() + QOI_HEADER_SIZE + QOI_PADDING_SIZE);

}

#[test]
fn test_encode_rgb_only() {
    test_chunk([[0x91, 0xa0, 0xa3]], [QOI_OP_RGB, 0x91, 0xa0, 0xa3]);
}

#[test]
fn test_encode_rgba_first() {
    test_chunk([[0, 0, 0, 0xff]], [QOI_OP_RUN]);
}

#[test]
fn test_encode_rgba_default() {
    test_chunk([[0x91, 0xa0, 0xa3, 0xff]], [QOI_OP_RGB, 0x91, 0xa0, 0xa3]);
}




#[test]
fn test_encode_rgba() {
    test_chunk([[0x91, 0xa0, 0xa3,0xfa]], [QOI_OP_RGBA, 0x91, 0xa0, 0xa3,0xfa]);
}