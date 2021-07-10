use crate::Float;
use cgmath::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<Float>,
    pub direction: Vector3<Float>,
}

impl Ray {
    pub fn at(&self, t: Float) -> Point3<Float> {
        self.origin + t * self.direction
    }
}
