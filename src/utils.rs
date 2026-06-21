use crate::ray::Ray;
use crate::vector3::{Vector3, unit, dot};

pub fn blue_to_white_gradient(ray: &Ray) -> Vector3 {
    // Linear interpolation, or "Lerp"
    let unit_direction = unit(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Vector3{ x: 1.0, y: 1.0, z: 1.0} + a * Vector3{x: 0.5, y: 0.7, z: 1.0}
}

///
/// Based on the sphere equation x^2 + y^2 + z^2 = radius^2
/// 
pub fn hit_sphere(center: &Vector3, radius: f64, ray: &Ray) -> bool {
    // and here comes trouble with my lazy operator implementation :D
    let oc = *center - ray.origin;
    let a = dot(ray.direction, ray.direction);
    let b = -2 as f64 * dot(ray.direction, oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4 as f64 * a * c;

    discriminant >= 0 as f64
}

pub fn pixel_color(center: &Vector3, radius: f64, ray: &Ray) -> Vector3 {
    if hit_sphere(center, radius, ray) {
        Vector3 { x: 255 as f64, y: 0 as f64, z: 0 as f64 }
    } else {
        blue_to_white_gradient(ray)
    }
}