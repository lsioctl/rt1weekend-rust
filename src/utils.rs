use rand::RngExt;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector3::{Vector3, unit};

const INFINITY: f64 = f64::MAX;

pub fn random() -> f64 {
    let mut rng = rand::rng();

    // TODO: by default uniformly distributed between 0 an 1 excluded for f64
    rng.random_range(0_f64..=1_f64)
}

pub fn random_ranged(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();

    rng.random_range(min..=max)
}

pub fn blue_to_white_gradient(ray: &Ray) -> Vector3 {
    // Linear interpolation, or "Lerp"
    let unit_direction = unit(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a)
        * Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + a * Vector3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

pub fn pixel_color(world: &Vec<Box<dyn Hittable>>, ray: &Ray) -> Vector3 {
    let mut t_max = INFINITY;
    let mut hit_record = HitRecord {
        point: Vector3 {
            x: 0 as f64,
            y: 0 as f64,
            z: 0 as f64,
        },
        normal: Vector3 {
            x: 0 as f64,
            y: 0 as f64,
            z: 0 as f64,
        },
        t: 0 as f64,
    };
    let mut hit = false;

    for object in world.iter() {
        if let Some(current_hit_record) = object.hit(&ray, 0 as f64, t_max) {
            hit = true;
            // update t_max as we may hit object closer to the ray origin later
            t_max = current_hit_record.t;
            hit_record = current_hit_record;
        }
    }

    match hit {
        true => {
            // unit is range -1, 1 and we map it to 0, 1 range for color
            0.5 * Vector3 {
                x: hit_record.normal.x + 1 as f64,
                y: hit_record.normal.y + 1 as f64,
                z: hit_record.normal.z + 1 as f64,
            }
        }
        false => blue_to_white_gradient(ray),
    }
}
