use std::sync::Arc;

use cgmath::{point3, vec3};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    Float,
};

pub struct XYRect {
    pub x0: Float,
    pub x1: Float,
    pub y0: Float,
    pub y1: Float,
    pub k: Float,
    pub material: Arc<Box<dyn Material>>,
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = vec3(0.0, 0.0, 1.0);

        Some(HitRecord::new(
            ray.at(t),
            outward_normal,
            t,
            u,
            v,
            ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        Some(AABB {
            minimum: point3(self.x0, self.y0, self.k - 0.0001),
            maximum: point3(self.x1, self.y1, self.k + 0.0001),
        })
    }
}
