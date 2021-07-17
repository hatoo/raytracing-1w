use std::sync::Arc;

use cgmath::{dot, vec3, EuclideanSpace, InnerSpace, Point3};
use rand::Rng;

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    material::Material,
    math::sphere_uv,
    Float,
};

pub struct MovingSphere<R> {
    pub center0: Point3<Float>,
    pub center1: Point3<Float>,
    pub time0: Float,
    pub time1: Float,
    pub radius: Float,
    pub material: Arc<Box<dyn Material<R = R>>>,
}

impl<R: Rng> MovingSphere<R> {
    pub fn center(&self, time: Float) -> Point3<Float> {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<R: Rng + Send + Sync> Hittable for MovingSphere<R> {
    type R = R;

    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: Float,
        t_max: Float,
        _rng: &mut R,
    ) -> Option<HitRecord<R>> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.magnitude2();
        let half_b = dot(oc, ray.direction);
        let c = oc.magnitude2() - self.radius * self.radius;

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
        let outward_normal = (position - self.center(ray.time)) / self.radius;
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

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        let box0 = AABB {
            minimum: self.center(time0) - vec3(self.radius, self.radius, self.radius),
            maximum: self.center(time0) + vec3(self.radius, self.radius, self.radius),
        };

        let box1 = AABB {
            minimum: self.center(time1) - vec3(self.radius, self.radius, self.radius),
            maximum: self.center(time1) + vec3(self.radius, self.radius, self.radius),
        };

        Some(surrounding_box(box0, box1))
    }
}
