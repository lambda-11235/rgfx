
extern crate rgfx;

use rgfx::format::*;
use rgfx::image::*;
use rgfx::transform;

use std::fs::File;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const SSAA: usize = 8;


fn calc(rad: f32, ang: f32, _color: Color, _img: &Image) -> Color {
    let mut r = rad - ang;
    let mut g = rad*ang;
    let mut b = 1.0 - r - g;

    let mut sum = r+g+b;
    if sum == 0.0 {
        sum = 1.0;
    }

    r = r*255.0/sum;
    g = g*255.0/sum;
    b = b*255.0/sum;

    Color{red: r as u8, green: g as u8, blue: b as u8}
}


pub fn main() {
    let mut img = Image::new(SSAA*WIDTH, SSAA*HEIGHT);

    transform::map_polar(&mut img, calc);
    transform::scale_down(&mut img, SSAA);

    let mut file = File::create("cool_curve.ppm").unwrap();
    write_ppm(&img, &mut file).unwrap();
}

