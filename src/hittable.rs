use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
