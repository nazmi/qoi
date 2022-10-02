mod common;
use qoi::{consts::*, pixel::Pixel, *};

fn test_chunk<const N: usize>(data: impl AsRef<[[u8; N]]>, expected: impl AsRef<[u8]>) {
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
    println!("{:x?} {:?}", encoded, encoded.len());
    assert_eq!(&encoded[QOI_HEADER_SIZE..][..expected.len()], expected);
    assert_eq!(
        encoded.len(),
        expected.len() + QOI_HEADER_SIZE + QOI_PADDING_SIZE
    );
}

#[test]
fn test_encode_rgb() {
    let v = vec![[0xab, 0xcd, 0xef]];
    let expected = [QOI_OP_RGB, 0xab, 0xcd, 0xef];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba_default() {
    let v = vec![[0, 0, 0, 0xff]];
    let expected = [QOI_OP_RUN];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba_same_alpha() {
    let v = vec![[0xab, 0xcd, 0xef, 0xff]];
    let expected = [QOI_OP_RGB, 0xab, 0xcd, 0xef];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba() {
    let v = vec![[0xab, 0xcd, 0xef, 0xab]];
    let expected = [QOI_OP_RGBA, 0xab, 0xcd, 0xef, 0xab];

    test_chunk(v, expected);
}

#[test]
fn test_encode_run_rgb_len62() {
    for n in 1..=62 {
        let pixel = [0xab, 0xcd, 0xef];
        let mut v = vec![[0, 0, 0]; n];
        v.push(pixel);

        let expected = [
            QOI_OP_RUN | (n as u8 - 1),
            QOI_OP_RGB,
            pixel[0],
            pixel[1],
            pixel[2],
        ];
        test_chunk(v, expected);
    }
}

#[test]
fn test_encode_run_rgba_len62() {
    for n in 1..=62 {
        let pixel = [0xab, 0xcd, 0xef, 0xab];
        let mut v = vec![[0, 0, 0, 0xff]; n];
        v.push(pixel);

        let expected = [
            QOI_OP_RUN | (n as u8 - 1),
            QOI_OP_RGBA,
            pixel[0],
            pixel[1],
            pixel[2],
            pixel[3],
        ];
        test_chunk(v, expected);
    }
}

#[test]
fn test_encode_run_rgb_len62_124() {
    for n in 63..=124 {
        let pixel = [0xab, 0xcd, 0xef];
        let mut v = vec![[0, 0, 0]; n];
        v.push(pixel);

        let expected = [
            QOI_OP_RUN | 61,
            QOI_OP_RUN | (n as u8 - 63),
            QOI_OP_RGB,
            pixel[0],
            pixel[1],
            pixel[2],
        ];
        test_chunk(v, expected)
    }
}

#[test]
fn test_encode_run_rgba_len62_124() {
    for n in 63..=124 {
        let pixel = [0xab, 0xcd, 0xef, 0xab];
        let mut v = vec![[0, 0, 0, 0xff]; n];
        v.push(pixel);

        let expected = [
            QOI_OP_RUN | 61,
            QOI_OP_RUN | (n as u8 - 63),
            QOI_OP_RGBA,
            pixel[0],
            pixel[1],
            pixel[2],
            pixel[3],
        ];
        test_chunk(v, expected)
    }
}

#[test]
fn test_encode_rgb_run() {
    let px = [0xab, 0xcd, 0xef];
    let v = vec![[0xef, 0xcd, 0xab], px, px, px];

    let expected = [
        QOI_OP_RGB,
        0xef,
        0xcd,
        0xab,
        QOI_OP_RGB,
        px[0],
        px[1],
        px[2],
        QOI_OP_RUN | 1,
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba_run() {
    let px = [0xab, 0xcd, 0xef, 0xab];
    let v = vec![[0xef, 0xcd, 0xab, 0xef], px, px, px];

    let expected = [
        QOI_OP_RGBA,
        0xef,
        0xcd,
        0xab,
        0xef,
        QOI_OP_RGBA,
        px[0],
        px[1],
        px[2],
        px[3],
        QOI_OP_RUN | 1,
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgb_rgba() {
    let rgb = [0xab, 0xcd, 0xef, 0xff];
    let rgba = [0x00, 0x00, 0x00, 0x00];
    let v = vec![rgb, rgba];

    let expected = [
        QOI_OP_RGB,
        rgb[0],
        rgb[1],
        rgb[2],
        QOI_OP_RGBA,
        rgba[0],
        rgba[1],
        rgba[2],
        rgba[3],
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba_rgb() {
    let rgba = [0xab, 0xcd, 0xef, 0xab];
    let rgb = [0x00, 0x00, 0x00, 0xab];
    let v = vec![rgba, rgb];

    let expected = [
        QOI_OP_RGBA,
        rgba[0],
        rgba[1],
        rgba[2],
        rgba[3],
        QOI_OP_RGB,
        rgb[0],
        rgb[1],
        rgb[2],
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgb_index() {
    let px_1 = [0xab, 0xcd, 0xef];
    let px_2 = [0x11, 0x22, 0x33];
    let v = vec![px_1, px_2, px_1];

    let expected = [
        QOI_OP_RGB,
        px_1[0],
        px_1[1],
        px_1[2],
        QOI_OP_RGB,
        px_2[0],
        px_2[1],
        px_2[2],
        QOI_OP_INDEX | Pixel::from(px_1).hash_index(),
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba_index() {
    let px_1 = [0xab, 0xcd, 0xef, 0xab];
    let px_2 = [0x11, 0x22, 0x33, 0xff];
    let v = vec![px_1, px_2, px_1];

    let expected = [
        QOI_OP_RGBA,
        px_1[0],
        px_1[1],
        px_1[2],
        px_1[3],
        QOI_OP_RGBA,
        px_2[0],
        px_2[1],
        px_2[2],
        px_2[3],
        QOI_OP_INDEX | Pixel::from(px_1).hash_index(),
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgb_diff() {
    let px_1 = [0xab, 0xcd, 0xef];
    let px_2 = [0xab, 0xcd, 0xf0];
    let v = vec![px_1, px_2];

    let expected = [
        QOI_OP_RGB,
        px_1[0],
        px_1[1],
        px_1[2],
        QOI_OP_DIFF | 2 << 4 | 2 << 2 | 3,
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba_diff() {
    let px_1 = [0xab, 0xcd, 0xef, 0xfe];
    let px_2 = [0xab, 0xcd, 0xf0, 0xfe];
    let v = vec![px_1, px_2];

    let expected = [
        QOI_OP_RGBA,
        px_1[0],
        px_1[1],
        px_1[2],
        px_1[3],
        QOI_OP_DIFF | 2 << 4 | 2 << 2 | 3,
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgb_luma() {
    let px_1 = [0xab, 0xcd, 0xde];
    let px_2 = [0xab+7, 0xcd, 0xde+7];
    let v = vec![px_1, px_2];

    let expected = [
        QOI_OP_RGB,
        px_1[0],
        px_1[1],
        px_1[2],
        QOI_OP_LUMA | 32,
        0xff,
    ];

    test_chunk(v, expected);
}

#[test]
fn test_encode_rgba_luma() {
    let px_1 = [0xab, 0xcd, 0xde, 0xfe];
    let px_2 = [0xab+7, 0xcd, 0xde+7, 0xfe];
    let v = vec![px_1, px_2];

    let expected = [
        QOI_OP_RGBA,
        px_1[0],
        px_1[1],
        px_1[2],
        px_1[3],
        QOI_OP_LUMA | 32,
        0xff,
    ];

    test_chunk(v, expected);
}

//rgb-rgba-run-diff-index-luma
#[test]
fn test_encode_long_1() {
    let rgb = [0xab, 0xcd, 0xef, 0xff];
    let rgba = [0x00, 0x00, 0x00, 0x00];
    let diff = [0x00, 0x00, 0x01, 0x00];
    let luma = [0x00+7, 0x00, 0x00+7, 0x00];

    let v = vec![rgb, rgba, rgba, rgba, rgba, diff, rgba, luma];

    let expected = [
        QOI_OP_RGB,
        rgb[0],
        rgb[1],
        rgb[2],
        QOI_OP_RGBA,
        rgba[0],
        rgba[1],
        rgba[2],
        rgba[3],
        QOI_OP_RUN | 2,
        QOI_OP_DIFF | 2 << 4 | 2 << 2 | 3,
        QOI_OP_INDEX | Pixel::from(rgba).hash_index(),
        QOI_OP_LUMA | 32,
        0xff,
    ];


    test_chunk(v, expected);
}


//run-rgba-luma-index-diff-rgb
#[test]
fn test_encode_long_2() {
    
    let rgb = [0xef, 0xcd, 0xab, 0xfe];
    let rgba = [0xab, 0xcd, 0xef, 0xfe];
    let run = [0x00, 0x00, 0x00, 0xff];
    let diff = [0xab, 0xcd, 0xef+1, 0xfe];
    let luma = [0xab+7, 0xcd, 0xef+7, 0xfe];

    let v = vec![run, run, rgba, luma, rgba, diff, rgb];

    let expected = [
        QOI_OP_RUN | 1,
        QOI_OP_RGBA,
        rgba[0],
        rgba[1],
        rgba[2],
        rgba[3],
        QOI_OP_LUMA | 32,
        0xff,
        QOI_OP_INDEX | Pixel::from(rgba).hash_index(),
        QOI_OP_DIFF | 2 << 4 | 2 << 2 | 3,
        QOI_OP_RGB,
        rgb[0],
        rgb[1],
        rgb[2],
    ];

    test_chunk(v, expected);
}