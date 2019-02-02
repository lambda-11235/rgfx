

extern crate logfx;

use logfx::format::*;
use logfx::image::*;
use logfx::transform;

use std::fs::File;


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
    let mut img = Image::new(1920, 1080);

    transform::map_polar(&mut img, calc);
    transform::simple_blur(&mut img, 8);

    let mut file = File::create("cool_curve.ppm").unwrap();
    write_ppm(&img, &mut file).unwrap();
}
