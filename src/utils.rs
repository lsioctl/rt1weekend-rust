use crate::ray::Ray;
use crate::vector3::{Vector3, unit};

pub fn blue_to_white_gradient(ray: &Ray) -> Vector3 {
    // Linear interpolation, or "Lerp"
    let unit_direction = unit(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Vector3{ x: 1.0, y: 1.0, z: 1.0} + a * Vector3{x: 0.5, y: 0.7, z: 1.0}
}