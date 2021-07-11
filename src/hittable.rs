use crate::ray::Ray;
use crate::Float;
use cgmath::{Point3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub position: Point3<Float>,
    pub normal: Vector3<Float>,
    pub t: Float,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}
