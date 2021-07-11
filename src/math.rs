use crate::Float;
use cgmath::{vec3, InnerSpace, Vector3};
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
