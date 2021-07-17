use crate::Float;
use cgmath::{dot, vec3, InnerSpace, Point3, Vector3};
use num_traits::float::FloatConst;
use rand::prelude::*;

pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vector3<Float> {
    loop {
        let v = vec3(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if v.magnitude2() < 1.0 {
            break v;
        }
    }
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: Vector3<Float>, rng: &mut impl Rng) -> Vector3<Float> {
    let v = random_in_unit_sphere(rng).normalize();
    if dot(normal, v) > 0.0 {
        v
    } else {
        -v
    }
}

pub fn random_in_unit_disk(rng: &mut impl Rng) -> Vector3<Float> {
    loop {
        let p = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.magnitude2() < 1.0 {
            break p;
        }
    }
}

pub fn random_cosine_direction(rng: &mut impl Rng) -> Vector3<Float> {
    let r1: Float = rng.gen();
    let r2: Float = rng.gen();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * Float::PI() * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    vec3(x, y, z)
}

pub fn random_to_sphere(
    radius: Float,
    distance_squared: Float,
    rng: &mut impl Rng,
) -> Vector3<Float> {
    let r1 = rng.gen::<Float>();
    let r2 = rng.gen::<Float>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * Float::PI() * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    vec3(x, y, z)
}

pub fn sphere_uv(point: Point3<Float>) -> (Float, Float) {
    let theta = (-point.y).acos();
    let phi = (-point.z).atan2(point.x) + Float::PI();
    (phi / (2.0 * Float::PI()), theta / Float::PI())
}

pub trait IsNearZero {
    fn is_near_zero(&self) -> bool;
}

impl IsNearZero for Vector3<Float> {
    fn is_near_zero(&self) -> bool {
        const S: Float = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}
