use std::cmp;
use image::{ImageBuffer, Rgba, Pixel};
use std::path::Path;

type Pos = (u32, u32);
type RGB = (u8, u8, u8);
type Transparency = u8;
pub type Pix = Rgba<u8>;
pub type Bitmap = ImageBuffer<Pix, Vec<u8>>; 

enum Color {
    RGB(RGB),
    A(Transparency)
}

type Bucket = Vec<Color>;

#[derive(Clone, Copy, Debug)]
enum Dir { N, E, S, W}

const BLACK: RGB = (0,0,0);
const RED: RGB = (255,0,0);
const GREEN: RGB = (0,255,0);
const YELLOW: RGB = (255,255,0);
const BLUE: RGB = (0,0,255);
const MAGENTA: RGB = (255,0,255);
const CYAN: RGB = (0,255,255);
const WHITE: RGB = (255,255,255);
const TRANSPARENT: Transparency = 0;
const OPAQUE: Transparency = 255;

struct State {
    bucket: Bucket,
    position: Pos,
    mark: Pos,
    dir: Dir,
    bitmaps: Vec<Bitmap> 
}

fn transparent_bitmap() -> Bitmap {
    ImageBuffer::from_pixel(600, 600, Rgba::from_channels(0,0,0,0))
}

impl State {
    fn new() -> State {
        State {
            bucket: Vec::new(),
            position: (0, 0),
            mark: (0, 0),
            dir: Dir::E,
            bitmaps: vec![transparent_bitmap()]
        }
    }
    
    fn move_dir(&mut self) {
        self.position = match (self.position, self.dir) {
            ((x,y), Dir::N) => (x, (y + 600 -1) % 600),
            ((x,y), Dir::E) => ((x + 1) % 600, y),
            ((x,y), Dir::S) => (x, (y + 1) % 600),
            ((x,y), Dir::W) => ((x + 600 - 1) % 600, y)
        };
    }
    
    fn turn_counterclockwise(&mut self) {
        self.dir = match self.dir {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S
        };
    }
    
    fn turn_clockwise(&mut self) {
        self.dir = match self.dir {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N
        };
    }
    
    fn get_pixel(&mut self, (x, y): Pos) -> Pix {
        self.bitmaps[0].get_pixel(x, y).clone()
    }
    
    fn set_pixel(&mut self, (x, y): Pos) {
        let pix = self.current_pixel();
        self.bitmaps[0].put_pixel(x, y, pix)
    }
    
    fn line(&mut self) {
        let (x0, y0) = self.position;
        let (x1, y1) = self.mark;
        let deltax = x1 as i32 - x0 as i32;
        let deltay = y1 as i32 - y0 as i32;
        let d = cmp::max(deltax.abs(), deltay.abs()); 
        let c = if deltax * deltay <= 0 { 1 } else { 0 };
        let offset = (d - c) / 2;
        let mut x = x0 as i32 * d + offset;
        let mut y = y0 as i32 * d + offset;
        for _ in 0..d {
            self.set_pixel(((x/d) as u32, (y/d) as u32));
            x += deltax;
            y += deltay;
        }
        self.set_pixel((x1, y1));
    }
    
    fn try_fill(&mut self) {
        let pos = self.position;
        let new = self.current_pixel();
        let old = self.get_pixel(pos);
        if new != old {
            self.fill(pos, old);
        }
    }
        
    fn fill(&mut self, p: Pos, initial: Pix) {
        let mut to_fill = vec![p];
        loop {
            match to_fill.pop() {
                None => return,
                Some((x,y)) => if self.get_pixel((x, y)) == initial {
                    self.set_pixel((x, y));
                    if x > 0 { to_fill.push((x-1, y)) }
                    if x < 599 { to_fill.push((x+1, y)) }
                    if y > 0 { to_fill.push((x, y-1)) }
                    if y < 599 { to_fill.push((x, y+ 1)) }
                } 
            }
        }
    }
    
    fn add_bitmap(&mut self, bitmap: Bitmap) {
        if self.bitmaps.len() >= 10 { return }
        self.bitmaps.insert(0, bitmap)
    }
    
    fn compose(&mut self) {
        if self.bitmaps.len() < 2 { return }
        for x in 0..600 {
            for y in 0..600 {
                let (r0, g0, b0, a0) = self.bitmaps[0].get_pixel(x, y).channels4();
                let (r1, g1, b1, a1) = self.bitmaps[1].get_pixel(x, y).channels4();
                self.bitmaps[1].put_pixel(x, y, Rgba::from_channels(
                    r0 + (((r1 as u32) * ((255 - a0) as u32))/255) as u8,
                    g0 + (((g1 as u32) * ((255 - a0) as u32))/255) as u8,
                    b0 + (((b1 as u32) * ((255 - a0) as u32))/255) as u8,
                    a0 + (((a1 as u32) * ((255 - a0) as u32))/255) as u8
                ))
            }
        }
        let _ = self.bitmaps.remove(0);
    }
    
    fn clip(&mut self) {
        if self.bitmaps.len() < 2 { return }
        for x in 0..600 {
            for y in 0..600 {
                let (_, _, _, a0) = self.bitmaps[0].get_pixel(x, y).channels4();
                let (r1, g1, b1, a1) = self.bitmaps[1].get_pixel(x, y).channels4();
                self.bitmaps[1].put_pixel(x, y, Rgba::from_channels(
                    (((r1 as u32) * (a0 as u32))/255) as u8,
                    (((g1 as u32) * (a0 as u32))/255) as u8,
                    (((b1 as u32) * (a0 as u32))/255) as u8,
                    (((a1 as u32) * (a0 as u32))/255) as u8
                ))
            }
        }
        let _ = self.bitmaps.remove(0);
    }
    
    fn add_color(&mut self, c: Color) {
        self.bucket.push(c)       
    }
    
    fn current_pixel(&self) -> Pix {
        let mut rsum = 0;
        let mut gsum = 0;
        let mut bsum = 0;
        let mut asum = 0;
        let mut rgbcount = 0;
        let mut acount = 0;
        for c in &self.bucket {
            match c {
                &Color::RGB((r,g,b)) => {
                    rgbcount += 1;
                    rsum += r as u32;
                    gsum += g as u32;
                    bsum += b as u32;
                },
                &Color::A(a) => {
                    acount += 1;
                    asum += a as u32;
                }
            }
        }
        let rc = if rgbcount == 0 { 0 } else { rsum / rgbcount };
        let gc = if rgbcount == 0 { 0 } else { gsum / rgbcount };
        let bc = if rgbcount == 0 { 0 } else { bsum / rgbcount };
        let ac = if acount == 0 { 255 } else { asum / acount };
        
        Rgba::from_channels((rc * ac / 255) as u8, (gc * ac / 255) as u8, (bc * ac / 255) as u8, ac as u8)
    }
}

#[test]
fn current_pixel_1() {
    let mut state = State::new();
    state.add_color(Color::A(TRANSPARENT));
    state.add_color(Color::A(OPAQUE));
    state.add_color(Color::A(OPAQUE));
    let pixel = state.current_pixel().channels4();
    assert_eq!((0, 0, 0, 170), pixel);
}

#[test]
fn current_pixel_2() {
    let mut state = State::new();
    state.add_color(Color::RGB(BLACK));
    state.add_color(Color::RGB(YELLOW));
    state.add_color(Color::RGB(CYAN));
    let pixel = state.current_pixel().channels4();
    assert_eq!((85, 170, 85, 255), pixel);
}

#[test]
fn current_pixel_3() {
    let mut state = State::new();
    state.add_color(Color::RGB(YELLOW));
    state.add_color(Color::A(TRANSPARENT));
    state.add_color(Color::A(OPAQUE));
    let pixel = state.current_pixel().channels4();
    assert_eq!((127, 127, 0, 127), pixel);
}

#[test]
fn current_pixel_4() {
    let mut state = State::new();
    for _ in 0..18 { state.add_color(Color::RGB(BLACK)) }
    for _ in 0..7 { state.add_color(Color::RGB(RED)) }
    for _ in 0..39 { state.add_color(Color::RGB(MAGENTA)) }
    for _ in 0..10 { state.add_color(Color::RGB(WHITE)) }
    for _ in 0..3 { state.add_color(Color::A(OPAQUE)) }
    for _ in 0..1 { state.add_color(Color::A(TRANSPARENT)) }
    let pixel = state.current_pixel().channels4();
    assert_eq!((143, 25, 125, 191), pixel);
}

pub fn build(rna: Vec<String>, out_file: &str, render_intermediate: bool) {
    let mut state = State::new();
    let mut iter = 0;
    for r in rna {
        iter += 1;
        if iter % 100 == 0 && render_intermediate {
            let s = format!("{}-{}", iter, out_file);
            state.bitmaps[0].save(&Path::new(&s)).unwrap();
        }
        match r.as_ref() {
            "PIPIIIC" => state.add_color(Color::RGB(BLACK)),
            "PIPIIIP" => state.add_color(Color::RGB(RED)),
            "PIPIICC" => state.add_color(Color::RGB(GREEN)),
            "PIPIICF" => state.add_color(Color::RGB(YELLOW)),
            "PIPIICP" => state.add_color(Color::RGB(BLUE)),
            "PIPIIFC" => state.add_color(Color::RGB(MAGENTA)),
            "PIPIIFF" => state.add_color(Color::RGB(CYAN)),
            "PIPIIPC" => state.add_color(Color::RGB(WHITE)),
            "PIPIIPF" => state.add_color(Color::A(TRANSPARENT)),
            "PIPIIPP" => state.add_color(Color::A(OPAQUE)),
            "PIIPICP" => state.bucket.clear(),
            "PIIIIIP" => state.move_dir(),
            "PCCCCCP" => state.turn_counterclockwise(),
            "PFFFFFP" => state.turn_clockwise(),
            "PCCIFFP" => state.mark = state.position,
            "PFFICCP" => state.line(),
            "PIIPIIP" => state.try_fill(),
            "PCCPFFP" => state.add_bitmap(transparent_bitmap()),
            "PFFPCCP" => state.compose(),
            "PFFICCF" => state.clip(),
            _ => ()
        }
    }
    let mut ret = state.bitmaps.remove(0);
    for x in 0..600 {
        for y in 0..600 {
            let p = ret.get_pixel(x, y).to_rgb().to_rgba();
            ret.put_pixel(x, y, p);
        }
    }
    let _ = ret.save(&Path::new(&out_file)).unwrap();
}