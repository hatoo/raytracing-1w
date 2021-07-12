use crate::Float;
use cgmath::{Point3, Vector3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<Float>,
    pub direction: Vector3<Float>,
    pub time: Float,
}

impl Ray {
    pub fn at(&self, t: Float) -> Point3<Float> {
        self.origin + t * self.direction
    }
}
