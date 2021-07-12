use std::sync::Arc;

use cgmath::{dot, prelude::*, vec3, Point3};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    math::sphere_uv,
    Float, MyRng,
};

pub struct Sphere {
    pub center: Point3<Float>,
    pub radius: Float,
    pub material: Arc<Box<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: Float,
        t_max: Float,
        _rng: &mut MyRng,
    ) -> Option<HitRecord> {
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
        let outward_normal = (position - self.center) / self.radius;
        let (u, v) = sphere_uv(EuclideanSpace::from_vec(outward_normal));

        Some(HitRecord::new(
            position,
            outward_normal,
            root,
            u,
            v,
            ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        Some(AABB {
            minimum: self.center - vec3(self.radius, self.radius, self.radius),
            maximum: self.center + vec3(self.radius, self.radius, self.radius),
        })
    }
}
