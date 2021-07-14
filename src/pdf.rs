use cgmath::{dot, InnerSpace, Vector3};

use crate::{math::random_cosine_direction, onb::Onb, Float, MyRng};
use num_traits::FloatConst;

pub trait Pdf {
    fn value(&self, direction: Vector3<Float>) -> Float;
    fn generate(&self, rng: &mut MyRng) -> Vector3<Float>;
}

pub struct CosinePdf {
    pub uvw: Onb,
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vector3<Float>) -> Float {
        let cosine = dot(direction.normalize(), self.uvw.w);
        (cosine / Float::PI()).max(0.0)
    }

    fn generate(&self, rng: &mut MyRng) -> Vector3<Float> {
        self.uvw.local(random_cosine_direction(rng))
    }
}
