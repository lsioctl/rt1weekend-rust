use crate::vector3;
use std::fs::File;
use std::io::{BufWriter, Write};

pub type Color = vector3::Vector3;

pub fn write_color(fd: &mut BufWriter<File>, pixel_color: &Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Translate the [0,1] component values to the byte range [0,255].
    let ir = 255.999 * r;
    let ig = 255.999 * g;
    let ib = 255.999 * b;

    writeln!(fd, "{} {} {}", ir, ig, ib).expect("unable to write line");
}
