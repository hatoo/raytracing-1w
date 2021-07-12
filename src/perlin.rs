use cgmath::{vec3, Point3};
use rand::{prelude::SliceRandom, Rng};

use crate::{color::Color, texture::Texture, Float};

#[derive(Debug)]
pub struct Perlin<const POINT_COUNT: usize> {
    ranfloat: [Float; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

pub type Perlin256 = Perlin<256>;

impl<const POINT_COUNT: usize> Perlin<POINT_COUNT> {
    fn generate_perm(rng: &mut impl Rng) -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }

        p.shuffle(rng);
        p
    }

    pub fn new(rng: &mut impl Rng) -> Self {
        let mut ranfloat = [0.0; POINT_COUNT];
        rng.fill(&mut ranfloat[..]);

        Self {
            ranfloat,
            perm_x: Self::generate_perm(rng),
            perm_y: Self::generate_perm(rng),
            perm_z: Self::generate_perm(rng),
        }
    }

    pub fn noise(&self, p: Point3<Float>) -> Float {
        let i = (4.0 * p.x) as isize & (POINT_COUNT as isize - 1);
        let j = (4.0 * p.y) as isize & (POINT_COUNT as isize - 1);
        let k = (4.0 * p.z) as isize & (POINT_COUNT as isize - 1);

        self.ranfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }
}

impl<const POINT_COUNT: usize> Texture for Perlin<POINT_COUNT> {
    fn value(&self, _u: Float, _v: Float, point: Point3<Float>) -> Color {
        return Color(self.noise(point) * vec3(1.0, 1.0, 1.0));
    }
}
