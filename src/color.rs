use crate::vector3;
use std::fs::File;
use std::io::{BufWriter, Write};

pub type Color = vector3::Vector3;

pub fn write_color(fd: &mut BufWriter<File>, pixel_color: &Color) {
    let r = (255.999 * pixel_color.x) as i32;
    let g = (255.999 * pixel_color.y) as i32;
    let b = (255.999 * pixel_color.z) as i32;

    writeln!(fd, "{} {} {}", r, g, b).expect("unable to write line");
}
