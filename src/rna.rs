use image::{ImageBuffer, Rgba};

type Pos = (u32, u32);
type RGB = (u8, u8, u8);
type Transparency = u8;
type Pixel = (RGB, Transparency);
pub type Bitmap = ImageBuffer<Rgba<u8>, Vec<u8>>; 

enum Color {
    RGB(RGB),
    A(Transparency)
}

type Bucket = Vec<Color>;
enum Dir { N, E, S, W}

const black: RGB = (0,0,0);
const red: RGB = (255,0,0);
const green: RGB = (0,255,0);
const yellow: RGB = (255,255,0);
const blue: RGB = (0,0,255);
const magenta: RGB = (255,0,255);
const cyan: RGB = (0,255,255);
const white: RGB = (255,255,255);
const transparent: Transparency = 0;
const opaque: Transparency = 255;

struct State {
    bucket: Bucket,
    position: Pos,
    mark: Pos,
    dir: Dir,
    bitmaps: Vec<Bitmap> 
}

fn transparent_bitmap() -> Bitmap {
    ImageBuffer::new(600, 600)
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
    
    fn addColor(&mut self, c: Color) {
        self.bucket.push(c)       
    }
    
    fn currentPixel(&self) -> Pixel {
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
        
        (((rc * ac / 255) as u8, (gc * ac / 255) as u8, (bc * ac / 255) as u8), ac as u8)
    }
}

#[test]
fn current_pixel_1() {
    let mut state = State::new();
    state.addColor(Color::A(transparent));
    state.addColor(Color::A(opaque));
    state.addColor(Color::A(opaque));
    let pixel = state.currentPixel();
    assert_eq!(((0, 0, 0), 170), pixel);
}

#[test]
fn current_pixel_2() {
    let mut state = State::new();
    state.addColor(Color::RGB(black));
    state.addColor(Color::RGB(yellow));
    state.addColor(Color::RGB(cyan));
    let pixel = state.currentPixel();
    assert_eq!(((85, 170, 85), 255), pixel);
}

#[test]
fn current_pixel_3() {
    let mut state = State::new();
    state.addColor(Color::RGB(yellow));
    state.addColor(Color::A(transparent));
    state.addColor(Color::A(opaque));
    let pixel = state.currentPixel();
    assert_eq!(((127, 127, 0), 127), pixel);
}

#[test]
fn current_pixel_4() {
    let mut state = State::new();
    for _ in 0..18 { state.addColor(Color::RGB(black)) }
    for _ in 0..7 { state.addColor(Color::RGB(red)) }
    for _ in 0..39 { state.addColor(Color::RGB(magenta)) }
    for _ in 0..10 { state.addColor(Color::RGB(white)) }
    for _ in 0..3 { state.addColor(Color::A(opaque)) }
    for _ in 0..1 { state.addColor(Color::A(transparent)) }
    let pixel = state.currentPixel();
    assert_eq!(((143, 25, 125), 191), pixel);
}

pub fn build(rna: Vec<String>) -> Bitmap {
    let mut state = State::new();
    for r in rna {
        match r.as_ref() {
            "PIPIIIC" => state.addColor(Color::RGB(black)),
            "PIPIIIP" => state.addColor(Color::RGB(red)),
            "PIPIICC" => state.addColor(Color::RGB(green)),
            "PIPIICF" => state.addColor(Color::RGB(yellow)),
            "PIPIICP" => state.addColor(Color::RGB(blue)),
            "PIPIIFC" => state.addColor(Color::RGB(magenta)),
            "PIPIIFF" => state.addColor(Color::RGB(cyan)),
            "PIPIIPC" => state.addColor(Color::RGB(white)),
            "PIPIIPF" => state.addColor(Color::A(transparent)),
            "PIPIIPP" => state.addColor(Color::A(opaque)),
            "PIIPICP" => state.bucket.clear(),            
            _ => panic!("nyi - {}", r)
        }
    }
    state.bitmaps.remove(0)
}