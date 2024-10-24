use std::fs::File;
use std::io::Write;

// struct Image {
//     img_width: u32,
//     img_height: u32,
//     buffer: Vec<u8>
// }

fn main() -> std::io::Result<()> {
    let mut f = File::create("test.ppm")?;

    let img_width: u32 = 100;
    let img_height: u32 = 100;
    let header = format!("P6 {img_width} {img_height} 255\n");
    let mut buffer: Vec<u8> = vec!(0; (img_width * img_height) as usize);

    println!("writing ppm ...");
    f.write(header.as_bytes())?;
    f.write(&buffer)?;

    println!("done!");

    Ok(())
}
