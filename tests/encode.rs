mod common;
use qoi::*;
use std::fs;
use std::path::{Path, PathBuf};

fn find_png(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let files = fs::read_dir(root)?;
    let mut out = vec![];

    for file in files {
        let f = file?.path();
        if f.extension().unwrap_or_default() == "png" {
            out.push(f)
        }
    }
    Ok(out)
}

#[test]
fn encode_images() -> Result<()> {

    let images = find_png("assets")?;

    for image in images {

        let decoder = png::Decoder::new(fs::File::open(&image)?);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0_u8; reader.output_buffer_size()];
        
        let info = reader.next_frame(&mut buf).unwrap();
        let data = &buf[..info.buffer_size()];
        let width = info.width;
        let height = info.height;
        let encoder = Encoder::new(data, width, height)?;
        
        let encoded = encoder.encode_to_vec()?;
        let qoi_file = image.with_extension("qoi");
        let expected = fs::read(qoi_file)?;

        assert_eq!(encoded.len(), expected.len());
        assert_eq!(encoded, expected);

    }

    Ok(())
    
}