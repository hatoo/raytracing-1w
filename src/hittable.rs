use crate::ray::Ray;
use crate::Float;
use cgmath::{dot, Point3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub position: Point3<Float>,
    pub normal: Vector3<Float>,
    pub t: Float,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        position: Point3<Float>,
        outward_normal: Vector3<Float>,
        t: Float,
        ray: &Ray,
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
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

impl Hittable for [Box<dyn Hittable>] {
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
