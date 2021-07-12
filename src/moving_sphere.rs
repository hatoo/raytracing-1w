use std::sync::Arc;

use cgmath::{dot, InnerSpace, Point3};

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    Float,
};

pub struct MovingSphere {
    pub center0: Point3<Float>,
    pub center1: Point3<Float>,
    pub time0: Float,
    pub time1: Float,
    pub radius: Float,
    pub material: Arc<Box<dyn Material>>,
}

impl MovingSphere {
    pub fn center(&self, time: Float) -> Point3<Float> {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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
            (position - self.center(ray.time)) / self.radius,
            root,
            ray,
            self.material.clone(),
        ))
    }
}
