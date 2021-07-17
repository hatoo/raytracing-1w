use std::fmt::Debug;

use crate::{
    onb::Onb,
    pdf::{CosinePdf, Pdf},
    texture::Texture,
    Float,
};
use cgmath::{dot, vec3, InnerSpace, Point3, Vector3};
use num_traits::FloatConst;
use rand::Rng;

use crate::{color::Color, hittable::HitRecord, math::random_vec3_in_unit_sphere, ray::Ray, MyRng};

pub enum ScatterKind {
    Spacular(Ray),
    Pdf(Box<dyn Pdf>),
}

pub struct Scatter {
    pub kind: ScatterKind,
    pub attenuation: Color,
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord, _rng: &mut MyRng) -> Option<Scatter> {
        None
    }

    fn scattering_pdf(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _ray_scatterd: &Ray,
        _rng: &mut MyRng,
    ) -> Float {
        0.0
    }

    fn emitted(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _u: Float,
        _v: Float,
        _p: Point3<Float>,
    ) -> Color {
        Color(vec3(0.0, 0.0, 0.0))
    }
}

#[derive(Debug)]
pub struct Lambertian<T> {
    pub albedo: T,
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: Float,
}

#[derive(Debug)]
pub struct DiffuseLight<T> {
    pub emit: T,
}

impl Material for () {}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord, _rng: &mut MyRng) -> Option<Scatter> {
        Some(Scatter {
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, hit_record.position),
            kind: ScatterKind::Pdf(Box::new(CosinePdf {
                uvw: Onb::from_w(hit_record.normal),
            })),
        })
    }

    fn scattering_pdf(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        ray_scatterd: &Ray,
        _rng: &mut MyRng,
    ) -> Float {
        let cosine = dot(hit_record.normal, ray_scatterd.direction.normalize());
        (cosine / Float::PI()).max(0.0)
    }
}

fn reflect(v: Vector3<Float>, n: Vector3<Float>) -> Vector3<Float> {
    v - 2.0 * dot(v, n) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut MyRng) -> Option<Scatter> {
        let reflected = reflect(ray.direction.normalize(), hit_record.normal);
        let spacular_ray = Ray {
            origin: hit_record.position,
            direction: reflected + self.fuzz * random_vec3_in_unit_sphere(rng),
            time: ray.time,
        };

        Some(Scatter {
            kind: ScatterKind::Spacular(spacular_ray),
            attenuation: self.albedo,
        })
    }
}

fn refract(uv: Vector3<Float>, n: Vector3<Float>, etai_over_etat: Float) -> Vector3<Float> {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude2()).abs().sqrt() * n;
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

        let unit_direction = ray.direction.normalize();
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
            attenuation: Color(vec3(1.0, 1.0, 1.0)),
            kind: ScatterKind::Spacular(Ray {
                origin: hit_record.position,
                direction,
                time: ray.time,
            }),
        })
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord, _rng: &mut MyRng) -> Option<Scatter> {
        None
    }

    fn emitted(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        u: Float,
        v: Float,
        p: Point3<Float>,
    ) -> Color {
        if hit_record.front_face {
            self.emit.value(u, v, p)
        } else {
            Color(vec3(0.0, 0.0, 0.0))
        }
    }
}
