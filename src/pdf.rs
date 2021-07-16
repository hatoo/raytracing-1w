use cgmath::{dot, EuclideanSpace, InnerSpace, Point3, Vector3};

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
        self.hittable.random(self.o.to_vec(), rng)
    }
}
