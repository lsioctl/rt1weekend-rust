use std::fs::File;
use std::io::{BufWriter, Write};

mod vector3;

fn main() {
    const IMG_WIDTH: i32 = 256;
    const IMG_HEIGHT: i32 = 256;

    const IMG_PATH: &str = "./images/img_001.ppm";

    let f = File::create(IMG_PATH).expect("unable to create file");

    let mut f = BufWriter::new(f);

    writeln!(f, "P3").expect("unable to write line");
    writeln!(f, "{}", IMG_WIDTH).expect("unable to write line");
    writeln!(f, "{}", IMG_HEIGHT).expect("unable to write line");
    writeln!(f, "255").expect("unable to write line");

    for i in 0..IMG_HEIGHT {
        for j in 0..IMG_WIDTH {
            let r = i as f32 / (IMG_WIDTH - 1) as f32;
            let g = j as f32 / (IMG_HEIGHT - 1) as f32;
            const B: f32 = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * B) as i32;

            writeln!(f, "{} {} {}", ir, ig, ib).expect("unable to write line");
        }
    }



}
