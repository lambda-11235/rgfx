
use image::*;


/// Draws a line between two points.
pub fn line(img: &mut Image, p1: Point, p2: Point, color: Color) {
    let (slope, inter) = p1.slope_inter(p2);

    for x in (p1.x.min(p2.x))..(p1.x.max(p2.x)) {
        let y = (slope*(x as f32) + inter) as usize;

        if x < img.get_width() && y < img.get_height() {
            *img.at(Point{x: x, y: y}) = color;
        }
    }

    for y in (p1.y.min(p2.y))..(p1.y.max(p2.y)) {
        let x = (((y as f32) - inter)/slope) as usize;

        if x < img.get_width() && y < img.get_height() {
            *img.at(Point{x: x, y: y}) = color;
        }
    }
}


/// Draws a triangle.
pub fn triangle(img: &mut Image, p1: Point, p2: Point, p3: Point, color: Color) {
    let mut low = p1;
    let mut mid = p2;
    let mut high = p3;
    let mut tmp: Point;

    if low.y > mid.y {
        tmp = low;
        low = mid;
        mid = tmp;
    }

    if mid.y > high.y {
        tmp = mid;
        mid = high;
        high = tmp;
    }

    if low.y > mid.y {
        tmp = low;
        low = mid;
        mid = tmp;
    }

    let (m1, b1) = low.slope_inter(mid);
    let (m2, b2) = low.slope_inter(high);
    let (m3, b3) = mid.slope_inter(high);

    for y in low.y..mid.y {
        let mut x1 = (((y as f32) - b1)/m1) as usize;
        let mut x2 = (((y as f32) - b2)/m2) as usize;

        if x1 > img.get_width() {
            x1 = img.get_width();
        }

        if x2 > img.get_width() {
            x2 = img.get_width();
        }

        for x in x1.min(x2)..x1.max(x2) {
            if x < img.get_width() && y < img.get_height() {
                *img.at(Point{x: x, y: y}) = color;
            }
        }
    }

    for y in mid.y..high.y {
        let mut x1 = (((y as f32) - b2)/m2) as usize;
        let mut x2 = (((y as f32) - b3)/m3) as usize;

        if x1 > img.get_width() {
            x1 = img.get_width();
        }

        if x2 > img.get_width() {
            x2 = img.get_width();
        }

        for x in x1.min(x2)..x1.max(x2) {
            if x < img.get_width() && y < img.get_height() {
                *img.at(Point{x: x, y: y}) = color;
            }
        }
    }
}


/// Draws a rectangle.
pub fn rectangle(img: &mut Image, p: Point, width: usize, height: usize, color: Color) {
    for x in p.x..((p.x + width).min(img.get_width())) {
        for y in p.y..((p.y + height).min(img.get_height())) {
            *img.at(Point{x: x, y: y}) = color;
        }
    }
}


/// Draws a circle.
pub fn circle(img: &mut Image, p: Point, radius: usize, color: Color) {
    let x1 = if p.x < radius { 0 } else { p.x - radius };
    let x2 = if p.x + radius < img.get_width() { p.x + radius } else { img.get_width() };
    let y1 = if p.y < radius { 0 } else { p.y - radius };
    let y2 = if p.y + radius < img.get_height() { p.y + radius } else { img.get_height() };

    for x in x1..x2 {
        for y in y1..y2 {
            let xf = x as f32;
            let yf = y as f32;

            if (xf - p.x as f32).hypot(yf - p.y as f32) < radius as f32 {
                *img.at(Point{x: x, y: y}) = color;
            }
        }
    }
}
