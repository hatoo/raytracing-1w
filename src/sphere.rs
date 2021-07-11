use cgmath::{dot, prelude::*, Point3};

use crate::{
    hittable::{HitRecord, Hittable},
    Float,
};

pub struct Sphere {
    pub center: Point3<Float>,
    pub radius: Float,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = InnerSpace::magnitude2(ray.direction);
        let half_b = dot(oc, ray.direction);
        let c = InnerSpace::magnitude2(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let position = ray.at(root);

        Some(HitRecord::new(
            position,
            (position - self.center) / self.radius,
            root,
            ray,
        ))
    }
}
