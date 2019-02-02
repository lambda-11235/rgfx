
#[derive(Clone,Copy,Debug)]
pub struct Point {
    pub x : usize,
    pub y : usize,
}

impl Point {
    pub fn slope_inter(&self, other: Point) -> (f32, f32) {
        let slope = (self.y as f32 - other.y as f32)/(self.x as f32 - other.x as f32);
        let inter = self.y as f32 - slope*(self.x as f32);

        (slope, inter)
    }
}


#[derive(Clone,Copy,Debug)]
pub struct Color {
    pub red : u8,
    pub green : u8,
    pub blue : u8,
}

pub const BLACK: Color = Color { red: 0, green: 0, blue: 0 };
pub const RED: Color = Color { red: 255, green: 0, blue: 0 };
pub const GREEN: Color = Color { red: 0, green: 255, blue: 0 };
pub const BLUE: Color = Color { red: 0, green: 0, blue: 255 };
pub const YELLOW: Color = Color { red: 255, green: 255, blue: 0 };
pub const CYAN: Color = Color { red: 0, green: 255, blue: 255 };
pub const PINK: Color = Color { red: 255, green: 0, blue: 255 };
pub const WHITE: Color = Color { red: 255, green: 255, blue: 255 };


#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Image {
    /// Creates a new image filled with black.
    pub fn new(width: usize, height: usize) -> Image {
        let mut img = Image {width: width, height: height
            , pixels: Vec::with_capacity(width*height) };

        for _ in 0..(width*height) {
            img.pixels.push(BLACK);
        }

        img
    }

    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }

    /// Gets a mutable reference to the pixel at a point.
    pub fn at(&mut self, p: Point) -> &mut Color {
        &mut self.pixels[p.y*self.width + p.x]
    }

    /// Gets an immutable reference to the pixel at a point.
    pub fn at_imm(&self, p: Point) -> Color {
        self.pixels[p.y*self.width + p.x]
    }


    /// Create a new image from a pixel buffer. If `buf` is not of length
    /// `width*height`, then only part of the image will be colored, while the
    /// rest will be filled black.
    pub fn from_buf(width: usize, height: usize, buf: &mut [u8]) -> Image {
        let mut img = Image::new(width, height);

        for i in 0..(buf.len()/3) {
            let x = i%width;
            let y = i/width;
            let mut pixel = img.at(Point{x: x, y: y});

            pixel.red = buf[3*i];
            pixel.green = buf[3*i+1];
            pixel.blue = buf[3*i+2];
        }

        img
    }


    /// Stores the render buffer into another buffer. It is up to the user to
    /// make sure that `buf` is big enough too hold the data (i.e. has length
    /// `self.get_width() * self.get_height()`). If buf is not big enough only
    /// part of the image will be written.
    pub fn to_buf(&self, buf: &mut [u8]) {
        for i in 0..(buf.len()/3) {
            let x = i%self.width;
            let y = i/self.width;
            let pixel = self.at_imm(Point{x: x, y: y});

            buf[3*i] = pixel.red;
            buf[3*i+1] = pixel.green;
            buf[3*i+2] = pixel.blue;
        }
    }
}
