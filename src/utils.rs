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
pub fn hit_sphere(center: &Vector3, radius: f64, ray: &Ray) -> f64 {
    // and here comes trouble with my lazy operator implementation :D
    let oc = *center - ray.origin;
    let a = dot(ray.direction, ray.direction);
    let b = -2 as f64 * dot(ray.direction, oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4 as f64 * a * c;

    if discriminant < 0 as f64 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt() ) / (2.0 * a);
    }
}

pub fn pixel_color(center: &Vector3, radius: f64, ray: &Ray) -> Vector3 {
    let t = hit_sphere(center, radius, ray);
    if t > 0 as f64 {
        let normal = unit(ray.at(t) - *center);

        // unit is range -1, 1 and we map it to 0, 1 range for color
        0.5 * Vector3{ 
            x: normal.x + 1 as f64, 
            y: normal.y + 1 as f64, 
            z: normal.z + 1 as f64, 
        }
    } else {
        blue_to_white_gradient(ray)
    }
}