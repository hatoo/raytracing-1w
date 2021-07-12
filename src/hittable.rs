use std::sync::Arc;

use crate::aabb::{surrounding_box, AABB};
use crate::Float;
use crate::{material::Material, ray::Ray};
use cgmath::{dot, Point3, Vector3};

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub position: Point3<Float>,
    pub normal: Vector3<Float>,
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub front_face: bool,
    pub material: Arc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        position: Point3<Float>,
        outward_normal: Vector3<Float>,
        t: Float,
        u: Float,
        v: Float,
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
            u,
            v,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB>;
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.as_ref().bounding_box(time0, time1)
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

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        let mut b = None;

        for hittable in self {
            if let Some(b0) = hittable.bounding_box(time0, time1) {
                b = Some(if let Some(b) = b {
                    surrounding_box(b, b0)
                } else {
                    b0
                });
            } else {
                {
                    return None;
                }
            }
        }
        b
    }
}
