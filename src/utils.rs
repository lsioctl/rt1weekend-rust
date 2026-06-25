use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::{Vector3, unit};

const INFINITY: f64 = f64::MAX;

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
    for object in world.iter() {
        if let Some(hit_record) = object.hit(&ray, 0 as f64, INFINITY) {
            // unit is range -1, 1 and we map it to 0, 1 range for color
            return 0.5
                * Vector3 {
                    x: hit_record.normal.x + 1 as f64,
                    y: hit_record.normal.y + 1 as f64,
                    z: hit_record.normal.z + 1 as f64,
                };
        }
    }

    blue_to_white_gradient(ray)
}
