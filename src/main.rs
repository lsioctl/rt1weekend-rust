mod color;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vector3;

use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Vector3;

fn main() {
    const IMG_DIR: &str = "./images/";
    const N_COLOR: i32 = 255;

    let _ = create_dir_all(IMG_DIR).expect("unable to create img dir");
    let img_path = format!("{}/img_001.ppm", IMG_DIR);
    let f = File::create(img_path).expect("unable to create image file");

    let mut fd = BufWriter::new(f);

    let aspect_ratio = 16 as f64 / 9 as f64;
    let image_width = 400 as i32;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height_raw = (image_width as f64 / aspect_ratio) as i32;
    let image_height = match image_height_raw < 1 {
        true => 1,
        false => image_height_raw,
    };

    // Camera
    // Viewport widths less than one are ok since they are real valued.
    let viewport_height = 2 as f64;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let focal_length = 1 as f64;
    let camera_center = Vector3 {
        x: 0 as f64,
        y: 0 as f64,
        z: 0 as f64,
    };

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3 {
        x: viewport_width,
        y: 0 as f64,
        z: 0 as f64,
    };
    let viewport_v = Vector3 {
        x: 0 as f64,
        y: -viewport_height,
        z: 0 as f64,
    };

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vector3 {
            x: 0 as f64,
            y: 0 as f64,
            z: focal_length,
        }
        - viewport_u / 2 as f64
        - viewport_v / 2 as f64;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    writeln!(fd, "P3").expect("unable to write line");
    writeln!(fd, "{}", image_width).expect("unable to write line");
    writeln!(fd, "{}", image_height).expect("unable to write line");
    writeln!(fd, "{}", N_COLOR).expect("unable to write line");

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    world.push(Box::new(Sphere {
        center: Vector3 {
            x: 0 as f64,
            y: 0 as f64,
            z: -1 as f64,
        },
        radius: 0.5 as f64,
    }));

    world.push(Box::new(Sphere {
        center: Vector3 {
            x: 0 as f64,
            y: -100.5 as f64,
            z: -1 as f64,
        },
        radius: 100 as f64,
    }));

    for line in 0..image_height {
        println!("Scanlines remaining: {}", image_height - line);
        for col in 0..image_width {
            let pixel_center =
                pixel00_loc + (col as f64 * pixel_delta_u) + (line as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color = utils::pixel_color(&world, &ray);

            color::write_color(&mut fd, &pixel_color);
        }
    }
    println!("Done");
}
