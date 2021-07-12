use std::sync::Arc;

use cgmath::{vec3, InnerSpace};
use rand::Rng;

use crate::{
    hittable::{HitRecord, Hittable},
    material::{Material, Scatter},
    math::random_vec3_in_unit_sphere,
    ray::Ray,
    texture::Texture,
    Float, MyRng,
};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Arc<Box<dyn Material>>,
    neg_inv_density: Float,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, d: Float, texture: Box<dyn Texture>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Box::new(Isotropic { albedo: texture })),
            neg_inv_density: -1.0 / d,
        }
    }
}

#[derive(Debug)]
pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<Scatter> {
        let scattered = Ray {
            origin: hit_record.position,
            direction: random_vec3_in_unit_sphere(rng),
            time: ray.time,
        };
        let color = self
            .albedo
            .value(hit_record.u, hit_record.v, hit_record.position);

        Some(Scatter {
            ray: scattered,
            color,
        })
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<crate::aabb::AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord> {
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && rng.gen::<Float>() < 0.00001;

        if let Some(mut rec1) = self
            .boundary
            .hit(ray, Float::NEG_INFINITY, Float::INFINITY, rng)
        {
            if let Some(mut rec2) = self
                .boundary
                .hit(ray, rec1.t + 0.0001, Float::INFINITY, rng)
            {
                if debugging {
                    eprintln!("\nt_min={}, t_max={}", rec1.t, rec2.t);
                }

                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);

                if rec1.t >= rec2.t {
                    return None;
                }

                rec1.t = rec1.t.max(0.0);

                let ray_length = InnerSpace::magnitude(ray.direction);
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<Float>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = ray.at(t);

                if debugging {
                    eprintln!("hit_distance = {}\nrec.t={},rec.p={:?}", hit_distance, t, p);
                }

                Some(HitRecord::new(
                    p,
                    vec3(1.0, 0.0, 0.0),
                    t,
                    0.0,
                    0.0,
                    ray,
                    self.phase_function.clone(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
