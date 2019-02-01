

#[derive(Clone,Copy,Debug)]
pub struct Color {
    pub red : u8,
    pub green : u8,
    pub blue : u8,
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red: red, green: green, blue: blue }
    }
}

pub const BLACK: Color = Color { red: 0, green: 0, blue: 0 };
pub const RED: Color = Color { red: 255, green: 0, blue: 0 };
pub const GREEN: Color = Color { red: 0, green: 255, blue: 0 };
pub const BLUE: Color = Color { red: 0, green: 0, blue: 255 };
pub const WHITE: Color = Color { red: 255, green: 255, blue: 255 };


#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Image {
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

    pub fn at(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[y*self.width + x]
    }

    pub fn at_imm(&self, x: usize, y: usize) -> Color {
        self.pixels[y*self.width + x]
    }


    pub fn copy_buf(&self, buf: &mut [u8]) {
        for i in 0..(buf.len()/3) {
            let x = i%self.width;
            let y = i/self.width;

            buf[i] = self.at_imm(x, y).red;
            buf[i+1] = self.at_imm(x, y).green;
            buf[i+2] = self.at_imm(x, y).blue;
        }
    }
}
