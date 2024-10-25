use std::fs::File;
use std::io::Write;
use rand::distributions::{Distribution, Uniform};

struct RGB {
    r: u8,
    g: u8,
    b: u8
}

struct State {
    width: usize,
    height: usize,
    img_buf: Vec<u8>,
    f_buf: Vec<usize>
}

struct Point (usize, usize);

const RED: RGB = RGB { r:255, g:0, b:0 };
const GREEN: RGB = RGB { r:0, g:255, b:0 };
const BLUE: RGB = RGB { r:0, g:0, b:255 };
const YELLOW: RGB = RGB { r:255, g:255, b:0 };
const AQUA: RGB = RGB { r: 0, g:255, b: 255 };
const FUCHSIA: RGB = RGB { r: 255, g: 0, b: 255 };
const BLACK: RGB = RGB { r:0, g:0, b:0 };
const WHITE: RGB = RGB { r:255, g:255, b:255 };

const IMG_W: usize = 300;
const IMG_H: usize = 300;
const PALETTE: [RGB; 8] = [BLACK, RED, GREEN, BLUE, YELLOW, AQUA, FUCHSIA, WHITE];

impl State {
    fn sizeof_ibuf(&self) -> usize {
        3 * self.width * self.height
    }

    fn sizeof_fbuf(&self) -> usize {
        self.width * self.height
    }

    fn get_ibuf_pt(&self, x: usize, y: usize) -> Option<usize> {
        let offset = (x * 3) + (y * self.width * 3);
        if offset < self.sizeof_ibuf() {
            Some(offset)
        } else {
            None
        }
    }

    fn get_fbuf_pt(&self, x: usize, y: usize) -> Option<usize> {
        let offset = x + (y * self.width);
        if offset < self.sizeof_fbuf() {
            Some(offset)
        } else {
            None
        }
    }
}

fn new_state(w: usize, h: usize) -> State {
    State {
        width: w,
        height: h,
        img_buf: vec!(0; 3 * w * h),
        f_buf: vec!(7; w * h),
    }
}

fn field_buf_to_img(state: &mut State) -> bool {
    for y in 0..state.height {
        for x in 0..state.width {
            if let Some(pixel) = state.get_ibuf_pt(x, y) {
                let color = state.f_buf[state.get_fbuf_pt(x, y).unwrap()];

                state.img_buf[pixel] = PALETTE[color].r;
                state.img_buf[pixel + 1] = PALETTE[color].g;
                state.img_buf[pixel + 2] = PALETTE[color].b;
            } else {
                return false
            }
        }
    }

    return true
}

fn noize_field(fbuf: &mut Vec<usize>) {
    let mut rng = rand::thread_rng();
    let uni_random = Uniform::from(1..100);

    for p in fbuf {
        let blackorwhite = uni_random.sample(&mut rng);

        *p = if blackorwhite < 65 {
            continue
        } else {
            0
        }
    }
}

fn should_be_wall(ux: usize, uy: usize, state: &State) -> bool {
    let mut acc: u32 = 0;

    // is this a corner or edge? early return.
    if ux == 0 || ux == state.width-1 || uy == 0 || uy == state.height-1 {
        return true
    }

    let testp: [Point; 8] = [
        Point (ux-1, uy-1), Point (ux, uy-1), Point (ux+1, uy-1),
        Point (ux-1, uy), Point (ux+1, uy),
        Point (ux-1, uy+1), Point (ux, uy+1), Point (ux+1, uy+1)
    ];

    for p in testp {
        if state.f_buf[state.get_fbuf_pt(p.0, p.1).unwrap()] == 0 {
            acc = acc + 1;
        }
    }

    if acc >= 4 {
        return true
    } else {
        return false
    }
}

fn iterate_landscape(state: &mut State) {
    let mut fbuf_new: Vec<usize> = Vec::new();

    for y in 0..state.height {
        for x in 0..state.width {
            if should_be_wall(x, y, &state) {
                fbuf_new.push(0);
            } else {
                fbuf_new.push(7);
            }
        }
    }

    state.f_buf = fbuf_new.clone();
    fbuf_new.clear();
}

fn main() -> std::io::Result<()> {
    let mut f = File::create("test.ppm")?;
    let mut gstate = new_state(IMG_W, IMG_H);
    let header = format!("P6 {} {} 255\n", gstate.width, gstate.height);


    println!("generating ...");

    noize_field(&mut gstate.f_buf);

    for _ in 0..69 {
        iterate_landscape(&mut gstate);
    }

    println!("done!");

    
    println!("writing ppm ...");

    if field_buf_to_img(&mut gstate) == false {
        panic!("field to ppm failed!");
    }

    f.write(header.as_bytes())?;
    f.write(&gstate.img_buf)?;

    println!("done!");

    Ok(())
}
