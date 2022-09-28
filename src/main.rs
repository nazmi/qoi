use qoi::*;

use std::io::Write;
use std::fs::{File};
use std::path::{Path, PathBuf};


fn find_png(root: impl AsRef<Path>) -> Vec<PathBuf> {
    let root = root.as_ref();
    let files = std::fs::read_dir(root).unwrap();
    let mut out = vec![];

    for file in files {
        let f = file.unwrap().path();
        if f.extension().unwrap() == "png" {
            out.push(f)
        }
    }

    out

}


fn main() -> Result<()> {

    let images = find_png("assets");

    for image in images {

        
        println!("{}",image.display());
        let decoder = png::Decoder::new(File::open(&image).unwrap());

        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0_u8; reader.output_buffer_size()];
        
        let info = reader.next_frame(&mut buf).unwrap();
        let data = &buf[..info.buffer_size()];
        let width = info.width;
        let height = info.height;
        let encoder = Encoder::new(data, width, height)?;
        
        let out = encoder.encode_to_vec()?;
        let out_file = image.as_path().with_extension("qoi");
        let mut f = File::create(out_file).expect("aa");
        f.write_all(&out).expect("aaa");

    }

    Ok(())
    
}