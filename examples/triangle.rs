
extern crate rgfx;

use rgfx::draw;
use rgfx::format::*;
use rgfx::image::*;

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

    let mut file = File::create("triangle.ppm").unwrap();
    write_ppm(&img, &mut file).unwrap();
}

