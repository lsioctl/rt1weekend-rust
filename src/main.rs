use std::fs::File;
use std::io::{BufWriter, Write};

mod color;
mod vector3;

fn main() {
    const IMG_WIDTH: i32 = 256;
    const IMG_HEIGHT: i32 = 256;

    const IMG_PATH: &str = "./images/img_001.ppm";

    let f = File::create(IMG_PATH).expect("unable to create file");

    let mut fd = BufWriter::new(f);

    writeln!(fd, "P3").expect("unable to write line");
    writeln!(fd, "{}", IMG_WIDTH).expect("unable to write line");
    writeln!(fd, "{}", IMG_HEIGHT).expect("unable to write line");
    writeln!(fd, "255").expect("unable to write line");

    for i in 0..IMG_HEIGHT {
        for j in 0..IMG_WIDTH {
            let pixel_color = color::Color {
                x: i as f64 / (IMG_WIDTH - 1) as f64,
                y: j as f64 / (IMG_HEIGHT - 1) as f64,
                z: 0 as f64,
            };

            color::write_color(&mut fd, &pixel_color);
        }
    }
}
