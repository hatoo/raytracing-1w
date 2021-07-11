use crate::Float;
use cgmath::{dot, vec3, InnerSpace, Vector3};
use rand::prelude::*;

pub fn random_vec3_in_unit_sphere(rng: &mut impl Rng) -> Vector3<Float> {
    loop {
        let v = vec3(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if InnerSpace::magnitude2(v) < 1.0 {
            break v;
        }
    }
}

pub fn random_vec3_in_hemisphere(normal: Vector3<Float>, rng: &mut impl Rng) -> Vector3<Float> {
    let v = InnerSpace::normalize(random_vec3_in_unit_sphere(rng));
    if dot(normal, v) > 0.0 {
        v
    } else {
        -v
    }
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
