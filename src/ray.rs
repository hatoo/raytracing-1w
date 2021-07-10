use crate::Float;
use cgmath::{Point3, Vector3};

struct Ray {
    origin: Point3<Float>,
    direction: Vector3<Float>,
}

impl Ray {
    pub fn at(&self, t: Float) -> Point3<Float> {
        self.origin + t * self.direction
    }
}
