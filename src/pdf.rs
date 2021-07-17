use cgmath::{dot, InnerSpace, Point3, Vector3};
use rand::Rng;

use crate::{hittable::Hittable, math::random_cosine_direction, onb::Onb, Float, MyRng};
use num_traits::FloatConst;

pub trait Pdf {
    fn value(&self, direction: Vector3<Float>, rng: &mut MyRng) -> Float;
    fn generate(&self, rng: &mut MyRng) -> Vector3<Float>;
}

pub struct CosinePdf {
    pub uvw: Onb,
}

pub struct HittablePdf<T> {
    pub o: Point3<Float>,
    pub hittable: T,
}

pub struct MixturePdf<P0, P1> {
    pub p0: P0,
    pub p1: P1,
}

impl Pdf for Box<dyn Pdf> {
    fn value(&self, direction: Vector3<Float>, rng: &mut MyRng) -> Float {
        self.as_ref().value(direction, rng)
    }

    fn generate(&self, rng: &mut MyRng) -> Vector3<Float> {
        self.as_ref().generate(rng)
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vector3<Float>, _rng: &mut MyRng) -> Float {
        let cosine = dot(direction.normalize(), self.uvw.w);
        (cosine / Float::PI()).max(0.0)
    }

    fn generate(&self, rng: &mut MyRng) -> Vector3<Float> {
        self.uvw.local(random_cosine_direction(rng))
    }
}

impl<T: Hittable> Pdf for HittablePdf<T> {
    fn value(&self, direction: Vector3<Float>, rng: &mut MyRng) -> Float {
        self.hittable.pdf_value(self.o, direction, rng)
    }

    fn generate(&self, rng: &mut MyRng) -> Vector3<Float> {
        self.hittable.random(self.o, rng)
    }
}

impl<P0: Pdf, P1: Pdf> Pdf for MixturePdf<P0, P1> {
    fn value(&self, direction: Vector3<Float>, rng: &mut MyRng) -> Float {
        0.5 * self.p0.value(direction, rng) + 0.5 * self.p1.value(direction, rng)
    }

    fn generate(&self, rng: &mut MyRng) -> Vector3<Float> {
        if rng.gen() {
            self.p0.generate(rng)
        } else {
            self.p1.generate(rng)
        }
    }
}
