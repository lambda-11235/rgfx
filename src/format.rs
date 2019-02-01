
extern crate std;

use std::io::Write;

use image::*;


pub fn write_ppm<W>(img: &Image, wr: &mut W) -> std::io::Result<()>
where W: Write {
    try!(wr.write_fmt(format_args!("P6\n{} {} {}\n", img.get_width(), img.get_height(), 255)));

    let mut buf = vec![0; img.get_width()*img.get_height()];
    img.copy_buf(&mut buf);
    try!(wr.write(&buf));

    Ok(())
}
