pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // pub fn size(&self) -> f64 {
    //     self.max - self.min
    // }

    pub fn contains(&self, x: f64) -> bool {
        x <= self.max && x >= self.min
    }

    // pub fn surrounds(&self, x: f64) -> bool {
    //     x < self.max && x > self.min
    // }
}
