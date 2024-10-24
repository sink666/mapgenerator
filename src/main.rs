use std::fs::File;
use std::io::Write;

struct RGB {
    r: u8,
    g: u8,
    b: u8
}

struct Image {
    width: u32,
    height: u32,
    buffer: Vec<u8>
}

impl Image {
    fn sizeof_buf(&self) -> u32 {
        3 * self.width * self.height
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (x * 3) + (y * self.width * 3);
        if offset < self.sizeof_buf() {
            Some(offset as usize)
        } else {
            None
        }
    }
}

fn new_ppm(w: u32, h: u32) -> Image {
    Image {
        width: w,
        height: h,
        buffer: vec!(0; (3 * w * h) as usize),
    }
}

fn fill(color: RGB, image: &mut Image) {
    for x in 0..image.height {
        for y in 0..image.width {
            let offset = image.get_offset(x, y).unwrap();

            image.buffer[offset] = color.r;
            image.buffer[offset + 1] = color.g;
            image.buffer[offset + 2] = color.b;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut f = File::create("test.ppm")?;
    let mut img = new_ppm(100, 100);

    let header = format!("P6 {} {} 255\n", img.width, img.height);
    // let mut buffer: Vec<u8> = vec!(0; (3 * img_width * img_height) as usize);

    let red = RGB { r:255, g:0, b:0 };
    fill(red, &mut img);
    
    println!("writing ppm ...");

    f.write(header.as_bytes())?;
    f.write(&img.buffer)?;

    println!("done!");

    Ok(())
}
