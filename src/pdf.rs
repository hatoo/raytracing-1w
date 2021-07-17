use std::marker::PhantomData;

use cgmath::{dot, InnerSpace, Point3, Vector3};
use rand::Rng;

use crate::{hittable::Hittable, math::random_cosine_direction, onb::Onb, Float};
use num_traits::FloatConst;

pub trait Pdf {
    type R: Rng;
    fn value(&self, direction: Vector3<Float>, rng: &mut Self::R) -> Float;
    fn generate(&self, rng: &mut Self::R) -> Vector3<Float>;
}

pub struct CosinePdf<R> {
    pub uvw: Onb,
    pub _phantom: PhantomData<R>,
}

pub struct HittablePdf<T, R> {
    pub o: Point3<Float>,
    pub hittable: T,
    pub _phantom: PhantomData<R>,
}

pub struct MixturePdf<P0, P1, R> {
    pub p0: P0,
    pub p1: P1,
    pub _phantom: PhantomData<R>,
}

impl<R: Rng, T: Pdf<R = R> + ?Sized> Pdf for Box<T> {
    type R = R;

    fn value(&self, direction: Vector3<Float>, rng: &mut R) -> Float {
        self.as_ref().value(direction, rng)
    }

    fn generate(&self, rng: &mut R) -> Vector3<Float> {
        self.as_ref().generate(rng)
    }
}

impl<R: Rng> Pdf for CosinePdf<R> {
    type R = R;

    fn value(&self, direction: Vector3<Float>, _rng: &mut R) -> Float {
        let cosine = dot(direction.normalize(), self.uvw.w);
        (cosine / Float::PI()).max(0.0)
    }

    fn generate(&self, rng: &mut R) -> Vector3<Float> {
        self.uvw.local(random_cosine_direction(rng))
    }
}

impl<R: Rng, T: Hittable<R = R>> Pdf for HittablePdf<T, R> {
    type R = R;

    fn value(&self, direction: Vector3<Float>, rng: &mut R) -> Float {
        self.hittable.pdf_value(self.o, direction, rng)
    }

    fn generate(&self, rng: &mut R) -> Vector3<Float> {
        self.hittable.random(self.o, rng)
    }
}

impl<R: Rng, P0: Pdf<R = R>, P1: Pdf<R = R>> Pdf for MixturePdf<P0, P1, R> {
    type R = R;

    fn value(&self, direction: Vector3<Float>, rng: &mut R) -> Float {
        0.5 * self.p0.value(direction, rng) + 0.5 * self.p1.value(direction, rng)
    }

    fn generate(&self, rng: &mut R) -> Vector3<Float> {
        if rng.gen() {
            self.p0.generate(rng)
        } else {
            self.p1.generate(rng)
        }
    }
}
