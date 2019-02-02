
use image::*;


/// A simple blur algorithm that repeatedly replaces the values of pixels with
/// the average of their nearest neighbors. Useful for simple aliasing.
pub fn simple_blur(img: &mut Image, factor: u32) {
    let mut tmp = Image::new(img.get_width(), img.get_height());

    for _ in 0..factor {
        for x in 0..img.get_width() {
            for y in 0..img.get_height() {
                let mut r: u32 = 0;
                let mut g: u32 = 0;
                let mut b: u32 = 0;

                for xp in (x as isize - 1)..=(x as isize + 1) {
                    for yp in (y as isize - 1)..=(y as isize + 1) {
                        let mut xpp = if xp < 0 { 0 } else { xp as usize };
                        let mut ypp = if yp < 0 { 0 } else { yp as usize };
                        
                        if xpp >= img.get_width() {
                            xpp = img.get_width() - 1;
                        }

                        if ypp >= img.get_height() {
                            ypp = img.get_height() - 1;
                        }

                        let c = img.at_imm(Point{x: xpp, y: ypp});

                        r += c.red as u32;
                        g += c.green as u32;
                        b += c.blue as u32;
                    }
                }

                r /= 9;
                b /= 9;
                g /= 9;

                let mut c: &mut Color = tmp.at(Point{x: x, y: y});
                c.red = r as u8;
                c.green = g as u8;
                c.blue = b as u8;
            }
        }

        for x in 0..img.get_width() {
            for y in 0..img.get_height() {
                *img.at(Point{x: x, y: y}) = *tmp.at(Point{x: x, y: y});
            }
        }
    }
}


/// Maps a function across every pixel in the image. The function should take
/// the x,y coordinates, color, and corresponding image for a pixel and output
/// its new color.
pub fn map<F>(img: &mut Image, f: F)
    where F: Fn(usize, usize, Color, &Image) -> Color {
    let w = img.get_width();
    let h = img.get_height();

    for x in 0..w {
        for y in 0..h {
            let color = f(x, y, img.at_imm(Point{x: x, y: y}), img);
            *img.at(Point{x: x, y: y}) = color;
        }
    }
}


/// Maps a function across every pixel in the image. The function should take
/// the Cartesian coordinates, color, and corresponding image for a pixel and
/// output its new color.  For this function the coordinates are scaled between
/// 0 and 1.
pub fn map_cart<F>(img: &mut Image, f: F)
    where F: Fn(f32, f32, Color, &Image) -> Color {
    let w = img.get_width();
    let h = img.get_height();

    for x in 0..w {
        for y in 0..h {
            let xf = (x as f32)/(w as f32);
            let yf = (y as f32)/(h as f32);

            let color = f(xf, yf, img.at_imm(Point{x: x, y: y}), img);
            *img.at(Point{x: x, y: y}) = color;
        }
    }
}


/// Maps a function across every pixel in the image. The function should take
/// the polar coordinates (radius and then angle), color, and corresponding
/// image for a pixel and output its new color. For this function the radius is
/// scaled between 0 and 1, and the angle is between 0 and pi/2.
pub fn map_polar<F>(img: &mut Image, f: F)
    where F: Fn(f32, f32, Color, &Image) -> Color {
    let max_radius = (img.get_width() as f32).hypot(img.get_height() as f32);

    for x in 0..img.get_width() {
        for y in 0..img.get_height() {
            let rad = (x as f32).hypot(y as f32)/max_radius;
            let ang = (x as f32).atan2(y as f32);

            let color = f(rad, ang, img.at_imm(Point{x: x, y: y}), img);
            *img.at(Point{x: x, y: y}) = color;
        }
    }
}
