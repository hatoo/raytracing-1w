use std::sync::Arc;

use cgmath::{dot, prelude::*, vec3, Point3};
use num_traits::FloatConst;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    math::{random_to_sphere, sphere_uv},
    onb::Onb,
    ray::Ray,
    Float, MyRng,
};

pub struct Sphere {
    pub center: Point3<Float>,
    pub radius: Float,
    pub material: Arc<Box<dyn Material>>,
}

impl Hittable for Sphere {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: Float,
        t_max: Float,
        _rng: &mut MyRng,
    ) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
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

    fn pdf_value(&self, o: Point3<Float>, v: cgmath::Vector3<Float>, rng: &mut MyRng) -> Float {
        self.hit(
            &Ray {
                origin: o,
                direction: v,
                time: 0.0,
            },
            0.001,
            Float::INFINITY,
            rng,
        )
        .map(|_| {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - o).magnitude2()).sqrt();
            let solid_angle = 2.0 * Float::PI() * (1.0 - cos_theta_max);
            1.0 / solid_angle
        })
        .unwrap_or(0.0)
    }

    fn random(&self, o: cgmath::Vector3<Float>, rng: &mut MyRng) -> cgmath::Vector3<Float> {
        let direction = (self.center - o).to_vec();
        let distance_squared = direction.magnitude2();

        let uvw = Onb::from_w(direction);

        uvw.local(random_to_sphere(self.radius, distance_squared, rng))
    }
}
