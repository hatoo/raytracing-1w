use std::fmt::Debug;

use crate::{texture::Texture, Float};
use cgmath::{dot, vec3, InnerSpace, Point3, Vector3};
use rand::Rng;

use crate::{
    color::Color,
    hittable::HitRecord,
    math::{random_vec3_in_unit_sphere, IsNearZero},
    ray::Ray,
    MyRng,
};

#[derive(Debug, Clone)]
pub struct Scatter {
    pub color: Color,
    pub ray: Ray,
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<Scatter>;

    fn emitted(&self, _u: Float, _v: Float, _p: Point3<Float>) -> Color {
        Color(vec3(0.0, 0.0, 0.0))
    }
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: Float,
}

#[derive(Debug)]
pub struct DiffuseLight {
    pub emit: Box<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<Scatter> {
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
            time: ray.time,
        };

        Some(Scatter {
            color: self
                .albedo
                .value(hit_record.u, hit_record.v, hit_record.position),
            ray: scatterd,
        })
    }
}

fn reflect(v: Vector3<Float>, n: Vector3<Float>) -> Vector3<Float> {
    v - 2.0 * dot(v, n) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<Scatter> {
        let reflected = reflect(ray.direction, hit_record.normal);
        let scatterd = reflected + self.fuzz * random_vec3_in_unit_sphere(rng);
        if dot(scatterd, hit_record.normal) > 0.0 {
            Some(Scatter {
                color: self.albedo,
                ray: Ray {
                    origin: hit_record.position,
                    direction: scatterd,
                    time: ray.time,
                },
            })
        } else {
            None
        }
    }
}

fn refract(uv: Vector3<Float>, n: Vector3<Float>, etai_over_etat: Float) -> Vector3<Float> {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - InnerSpace::magnitude2(r_out_perp)).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: Float, ref_idx: Float) -> Float {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[derive(Debug)]
pub struct Dielectric {
    pub ir: Float,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<Scatter> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = InnerSpace::normalize(ray.direction);
        let cos_theta = dot(-unit_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<Float>() {
                reflect(unit_direction, hit_record.normal)
            } else {
                refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        Some(Scatter {
            color: Color(vec3(1.0, 1.0, 1.0)),
            ray: Ray {
                origin: hit_record.position,
                direction: direction,
                time: ray.time,
            },
        })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord, _rng: &mut MyRng) -> Option<Scatter> {
        None
    }

    fn emitted(&self, u: Float, v: Float, p: Point3<Float>) -> Color {
        self.emit.value(u, v, p)
    }
}
