use std::sync::Arc;

use crate::Float;
use crate::{material::Material, ray::Ray};
use cgmath::{dot, Point3, Vector3};

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub position: Point3<Float>,
    pub normal: Vector3<Float>,
    pub t: Float,
    pub front_face: bool,
    pub material: Arc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        position: Point3<Float>,
        outward_normal: Vector3<Float>,
        t: Float,
        ray: &Ray,
        material: Arc<Box<dyn Material>>,
    ) -> Self {
        let front_face = dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            position,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for [T] {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for hittable in self {
            if let Some(new_hit_record) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = new_hit_record.t;
                hit_record = Some(new_hit_record);
            }
        }

        hit_record
    }
}
