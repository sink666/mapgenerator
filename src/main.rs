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

const RED: RGB = RGB { r:255, g:0, b:0 };
const GREEN: RGB = RGB { r:0, g:255, b:0 };
const BLUE: RGB = RGB { r:0, g:0, b:255 };
const YELLOW: RGB = RGB { r:255, g:255, b:0 };
const AQUA: RGB = RGB { r: 0, g:255, b: 255 };
const FUCHSIA: RGB = RGB { r: 255, g: 0, b: 255 };
const BLACK: RGB = RGB { r:0, g:0, b:0 };
const WHITE: RGB = RGB { r:255, g:255, b:255 };

const IMG_W: u32 = 200;
const IMG_H: u32 = 200;
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

fn new_state(w: u32, h: u32) -> State {
    State {
        width: w as usize,
        height: h as usize,
        img_buf: vec!(0; (3 * w * h) as usize),
        f_buf: vec!(7; (w * h) as usize),
    }
}

fn field_buf_to_img(state: &mut State) -> bool {
    for y in 0..state.height {
        for x in 0..state.width {
            if let Some(pixel) = state.get_ibuf_pt(x, y) {
                let color = state.f_buf[x + (y * state.width)];

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
    let uni_random = Uniform::from(0..101);

    for p in fbuf {
        let blackorwhite = uni_random.sample(&mut rng);

        *p = if blackorwhite < 50 {
            continue
        } else {
            0
        }
    }
}

fn fill_edges(state: &mut State) {
    for y in 0..state.height {
        state.f_buf[0 + (y * state.width)] = 0;
        state.f_buf[state.width - 1 + (y * state.width)] = 0;
    }

    for x in 0..state.width {
        state.f_buf[x + 0] = 0;
        state.f_buf[x + ((state.height - 1) * state.width)] = 0;
    }
}

// fn should_be_wall(ux: usize, uy: usize, state: &State) -> bool {
//     // is this a wall already? return true
//     // are 5 points adjacent to this one (3x3) a wall? return true

//     if state.f_buf[state.get_fbuf_pt(ux, uy).unwrap()] == 0 {
//         return true;
//     }

//     false
// }

fn main() -> std::io::Result<()> {
    let mut f = File::create("test.ppm")?;
    let mut gstate = new_state(IMG_W, IMG_H);
    let header = format!("P6 {} {} 255\n", gstate.width, gstate.height);


    println!("generating ...");

    noize_field(&mut gstate.f_buf);
    fill_edges(&mut gstate);

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
