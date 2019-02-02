
extern crate logfx;

use logfx::draw;
use logfx::format::*;
use logfx::image::*;
use logfx::transform;

use std::fs::File;

pub fn main() {
    let mut img = Image::new(1920, 1080);
    let w = img.get_width();
    let h = img.get_height();

    for x in 0..w {
        for y in 0..h {
            img.at(Point{x: x, y: y}).red = 255;
        }
    }

    draw::triangle(&mut img, Point{x: 0, y: 0}, Point{x: w, y: h/2},
                   Point{x: w/2, y: h}, BLUE);

    draw::line(&mut img, Point{x: 0, y: h}, Point{x: w, y: 0}, GREEN);

    draw::circle(&mut img, Point{x: w/2, y: h/2}, h/10, YELLOW);

    transform::simple_blur(&mut img, 64);

    let mut file = File::create("simple_blur.ppm").unwrap();
    write_ppm(&img, &mut file).unwrap();
}


