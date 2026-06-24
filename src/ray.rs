use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn at(&self, time: f64) -> Vector3 {
        self.origin + time * self.direction
    }
}

