use std::io;
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
    iter_c: usize,
    header: String,
    img_buf: Vec<u8>,
    f_buf: Vec<Colors>
}

struct Point (usize, usize);

const BLACK: RGB = RGB { r:0, g:0, b:0 };
const WHITE: RGB = RGB { r:255, g:255, b:255 };

fn new_state(w: usize, h: usize, i: usize) -> State {
    State {
        width: w,
        height: h,
        iter_c: i,
        header: format!("P6 {} {} 255\n", w, h),
        img_buf: vec!(0; 3 * w * h),
        f_buf: vec!(Colors::White; w * h),
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
        if state.f_buf.get(p.0 + (p.1 * state.width)).unwrap() == &Colors::Black {
            acc = acc + 1;
        }
    }

    if acc >= 5 {
        return true
    } else {
        return false
    }
}

fn gen_landscape(st: &mut State) {
    let mut rng = rand::thread_rng();
    let uni_random = Uniform::from(1..100);

    for p in &mut st.f_buf {
        let blackorwhite = uni_random.sample(&mut rng);

        *p = if blackorwhite < 45 {
            continue
        } else {
            Colors::Black
        }
    }

    let mut fbuf_new: Vec<Colors> = Vec::new();

    for _ in 0..st.iter_c {
        for y in 0..st.height {
            for x in 0..st.width {
                if should_be_wall(x, y, &st) {
                    fbuf_new.push(Colors::Black);
                } else {
                    fbuf_new.push(Colors::White);
                }
            }
        }
    
        st.f_buf = fbuf_new.clone();
        fbuf_new.clear();
    }
}

fn output_file(st: &mut State) -> io::Result<()> {
    let mut f = File::create("test.ppm")?;

    for y in 0..st.height {
        for x in 0..st.width {
            let i_ofs = (x * 3) + (y * st.width * 3);
            let color = match st.f_buf.get(x + (y * st.width)).unwrap() {
                &Colors::Black => BLACK,
                &Colors::White => WHITE
            };
            
            st.img_buf[i_ofs] = color.r;
            st.img_buf[i_ofs + 1] = color.g;
            st.img_buf[i_ofs + 2] = color.b;
        }
    }

    f.write(st.header.as_bytes())?;
    f.write(&st.img_buf)?;

    Ok(())
}

fn main() {
    let mut gstate = new_state(100, 100, 20);

    gen_landscape(&mut gstate);

    output_file(&mut gstate).unwrap();
}
