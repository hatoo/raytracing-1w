use std::fmt::Debug;

use crate::Float;
use cgmath::{dot, InnerSpace, Vector3};

use crate::{
    color::Color,
    hittable::HitRecord,
    math::{random_vec3_in_unit_sphere, IsNearZero},
    ray::Ray,
    MyRng,
};

pub trait Material: Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<(Color, Ray)>;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: Float,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<(Color, Ray)> {
        let scatter_direction =
            hit_record.normal + InnerSpace::normalize(random_vec3_in_unit_sphere(rng));

        let scatter_direction = if scatter_direction.is_near_zero() {
            hit_record.normal
        } else {
            scatter_direction
        };

        let scatterd = Ray {
            origin: hit_record.position,
            direction: scatter_direction,
        };

        Some((self.albedo, scatterd))
    }
}

fn reflect(v: Vector3<Float>, n: Vector3<Float>) -> Vector3<Float> {
    v - 2.0 * dot(v, n) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<(Color, Ray)> {
        let reflected = reflect(ray.direction, hit_record.normal);
        let scatterd = reflected + self.fuzz * random_vec3_in_unit_sphere(rng);
        if dot(scatterd, hit_record.normal) > 0.0 {
            Some((
                self.albedo,
                Ray {
                    origin: hit_record.position,
                    direction: scatterd,
                },
            ))
        } else {
            None
        }
    }
}
