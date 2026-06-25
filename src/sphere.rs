use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector3::{Vector3, dot, len_squared};

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Hittable for Sphere {
    ///
    /// Based on the sphere equation x^2 + y^2 + z^2 = radius^2
    ///
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = len_squared(ray.direction);
        let h = dot(ray.direction, oc);
        let c = len_squared(oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0 as f64 {
            None
        } else {
            let t = (h - discriminant.sqrt()) / a;
            let point = ray.at(t);
            let normal = (point - self.center) / self.radius;

            Some(HitRecord { point, normal, t })
        }
    }
}
