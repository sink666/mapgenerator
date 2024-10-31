use std::fs::File;
use std::io::Write;
use rand::distributions::{Distribution, Uniform};

struct RGB {
    r: u8,
    g: u8,
    b: u8
}

#[derive(Clone, PartialEq)]
enum Colors {
    Black,
    White
}

struct State {
    width: usize,
    height: usize,
    header: String,
    img_buf: Vec<u8>,
    f_buf: Vec<Colors>
}

struct Point (usize, usize);

const BLACK: RGB = RGB { r:0, g:0, b:0 };
const WHITE: RGB = RGB { r:255, g:255, b:255 };

impl State {
    fn get_ibuf_pt(&self, x: usize, y: usize) -> Option<usize> {
        let offset = (x * 3) + (y * self.width * 3);
        if offset < (3 * self.width * self.height) {
            Some(offset)
        } else {
            None
        }
    }

    fn get_fbuf_pt(&self, x: usize, y: usize) -> Option<usize> {
        let offset = x + (y * self.width);
        if offset < (self.width * self.height) {
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
        header: format!("P6 {} {} 255\n", w, h),
        img_buf: vec!(0; 3 * w * h),
        f_buf: vec!(Colors::White; w * h),
    }
}

fn field_buf_to_img(state: &mut State) -> bool {
    for y in 0..state.height {
        for x in 0..state.width {
            if let Some(pixel) = state.get_ibuf_pt(x, y) {
                let c = match state.f_buf[state.get_fbuf_pt(x, y).unwrap()] {
                    Colors::Black => { BLACK },
                    Colors::White => { WHITE }
                };

                state.img_buf[pixel] = c.r;
                state.img_buf[pixel + 1] = c.g;
                state.img_buf[pixel + 2] = c.b;
            } else {
                return false
            }
        }
    }

    return true
}

fn noize_field(fbuf: &mut Vec<Colors>) {
    let mut rng = rand::thread_rng();
    let uni_random = Uniform::from(1..100);

    for p in fbuf {
        let blackorwhite = uni_random.sample(&mut rng);

        *p = if blackorwhite < 45 {
            continue
        } else {
            Colors::Black
        }
    }
}

fn should_be_wall(ux: usize, uy: usize, state: &State) -> bool {
    let mut acc: u32 = 0;

    // is this a corner or edge? early return.
    if ux == 0 || ux == state.width-1 || uy == 0 || uy == state.height-1 {
        return false
    }

    let testp: [Point; 9] = [
        Point (ux-1, uy-1), Point (ux, uy-1), Point (ux+1, uy-1),
        Point (ux-1, uy), Point(ux, uy), Point (ux+1, uy),
        Point (ux-1, uy+1), Point (ux, uy+1), Point (ux+1, uy+1)
    ];

    for p in testp {
        if state.f_buf[state.get_fbuf_pt(p.0, p.1).unwrap()] == Colors::Black {
            acc = acc + 1;
        }
    }

    if acc >= 5 {
        return true
    } else {
        return false
    }
}

fn iterate_landscape(state: &mut State) {
    let mut fbuf_new: Vec<Colors> = Vec::new();

    for y in 0..state.height {
        for x in 0..state.width {
            if should_be_wall(x, y, &state) {
                fbuf_new.push(Colors::Black);
            } else {
                fbuf_new.push(Colors::White);
            }
        }
    }

    state.f_buf = fbuf_new.clone();
    fbuf_new.clear();
}

fn main() {
    let mut f = File::create("test.ppm").expect("couldn't create file");
    let mut gstate = new_state(100, 100);

    noize_field(&mut gstate.f_buf);

    for _ in 0..20 {
        iterate_landscape(&mut gstate);
    }

    if field_buf_to_img(&mut gstate) == false {
        panic!("field to ppm failed!");
    }

    f.write(gstate.header.as_bytes()).expect("failed to write header");
    f.write(&gstate.img_buf).expect("failed to write image data");
}
