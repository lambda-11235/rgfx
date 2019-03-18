
extern crate rgfx;

use rgfx::format::*;
use rgfx::image::*;
use rgfx::transform;

use std::fs::File;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const SSAA: usize = 2;


fn calc(rad: f32, ang: f32, _color: Color, _img: &Image) -> Color {
    let mut r = (ang - 0.4).max(0.0001);
    let mut g = 0.7*rad * (ang - 0.6).max(0.0001);
    let mut b = ((1.0 - r - g)/(r + g)).abs();

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

    let mut file = File::create("cool_curve_2.ppm").unwrap();
    write_ppm(&img, &mut file).unwrap();
}


