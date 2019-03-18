
extern crate rgfx;

use rgfx::draw;
use rgfx::format::*;
use rgfx::image::*;
use rgfx::transform;

use std::fs::File;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;
const SSAA: usize = 8;


pub fn main() {
    let mut img = Image::new(SSAA*WIDTH, SSAA*HEIGHT);
    let w = img.get_width();
    let h = img.get_height();

    for x in 0..w {
        for y in 0..h {
            img.at(Point{x: x, y: y}).red = 255;
        }
    }

    draw::triangle(&mut img, Point{x: 0, y: 0}, Point{x: w, y: h/2},
                   Point{x: w/32, y: h}, BLUE);
    transform::scale_down(&mut img, SSAA);

    let mut file = File::create("triangle.ppm").unwrap();
    write_ppm(&img, &mut file).unwrap();
}

