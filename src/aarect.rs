use std::sync::Arc;

use cgmath::{dot, point3, vec3, EuclideanSpace, InnerSpace, Point3, Vector3};
use rand::Rng;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    Float, MyRng,
};

#[derive(Debug)]
pub struct XYRect {
    pub x0: Float,
    pub x1: Float,
    pub y0: Float,
    pub y1: Float,
    pub k: Float,
    pub material: Arc<Box<dyn Material>>,
}

#[derive(Debug)]
pub struct XZRect {
    pub x0: Float,
    pub x1: Float,
    pub z0: Float,
    pub z1: Float,
    pub k: Float,
    pub material: Arc<Box<dyn Material>>,
}

#[derive(Debug)]
pub struct YZRect {
    pub y0: Float,
    pub y1: Float,
    pub z0: Float,
    pub z1: Float,
    pub k: Float,
    pub material: Arc<Box<dyn Material>>,
}

impl Hittable for XYRect {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, _rng: &mut MyRng) -> Option<HitRecord> {
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

impl Hittable for XZRect {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, _rng: &mut MyRng) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = vec3(0.0, 1.0, 0.0);

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
            minimum: point3(self.x0, self.k - 0.0001, self.z0),
            maximum: point3(self.x1, self.k + 0.0001, self.z1),
        })
    }

    fn pdf_value(&self, origin: Point3<Float>, v: Vector3<Float>, rng: &mut MyRng) -> Float {
        if let Some(hit_record) = self.hit(
            &Ray {
                origin,
                direction: v,
                time: 0.0,
            },
            0.001,
            Float::INFINITY,
            rng,
        ) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = hit_record.t * hit_record.t * v.magnitude2();
            let cosine = (dot(v, hit_record.normal) / v.magnitude()).abs();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: Point3<Float>, rng: &mut MyRng) -> Vector3<Float> {
        let random_point = vec3(
            rng.gen_range(self.x0..self.x1),
            self.k,
            rng.gen_range(self.z0..self.z1),
        );
        random_point - origin.to_vec()
    }
}

impl Hittable for YZRect {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, _rng: &mut MyRng) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = vec3(1.0, 0.0, 0.0);

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
            minimum: point3(self.k - 0.0001, self.y0, self.z0),
            maximum: point3(self.k + 0.0001, self.y1, self.z1),
        })
    }
}
