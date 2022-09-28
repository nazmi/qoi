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
    println!("{:?} {:?}",encoded, encoded.len());
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

#[test]
fn test_encode_run_rgb_len62() {

    for n in 1..=62 {
        let mut v = vec![[0,0,0]; n];
        v.push([11, 22, 33]);
        test_chunk(v, [ QOI_OP_RUN | (n as u8 - 1),
                        QOI_OP_RGB, 11, 22, 33])

    }
}


#[test]
fn test_encode_run_rgba_len62() {

    for n in 1..=62 {
        let mut v = vec![[0,0,0, 0xff]; n];
        v.push([11, 22, 33, 44]);
        test_chunk(v, [ QOI_OP_RUN | (n as u8 - 1),
                        QOI_OP_RGBA, 11, 22, 33, 44])

    }
}

#[test]
fn test_encode_run_rgb_len62_124() {

    for n in 63..=124 {
        let mut v = vec![[0,0,0]; n];
        v.push([11,22,33]);
        test_chunk(v, [ QOI_OP_RUN | 61,
                        QOI_OP_RUN | (n as u8 - 63),
                        QOI_OP_RGB, 11, 22, 33])

    }
}

#[test]
fn test_encode_run_rgba_len62_124() {

    for n in 63..=124 {
        let mut v = vec![[0,0,0, 0xff]; n];
        v.push([11,22,33, 44]);
        test_chunk(v, [ QOI_OP_RUN | 61,
                        QOI_OP_RUN | (n as u8 - 63),
                        QOI_OP_RGBA, 11, 22, 33, 44])

    }
}

#[test]
fn test_encode_rgb_then_run() {
    let px = [0xab, 0xcd, 0xef];
    let v = vec![[0xef,0xcd,0xab], px, px, px];

    test_chunk(v, [ QOI_OP_RGB, 0xef, 0xcd, 0xab,
                    QOI_OP_RGB, 0xab, 0xcd, 0xef,
                    QOI_OP_RUN | 1 ]);
}

#[test]
fn test_encode_rgba_then_run() {
    let px = [0xab, 0xcd, 0xef, 0xab];
    let v = vec![[0xef,0xcd,0xab, 0xef],px,px,px];

    test_chunk(v, [ QOI_OP_RGBA, 0xef, 0xcd, 0xab, 0xef,
                    QOI_OP_RGBA, 0xab, 0xcd, 0xef, 0xab,
                    QOI_OP_RUN | 1]);
}
