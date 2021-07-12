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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = (i + di) & (POINT_COUNT as isize - 1);
                    let j = (j + dj) & (POINT_COUNT as isize - 1);
                    let k = (k + dk) & (POINT_COUNT as isize - 1);

                    c[di as usize][dj as usize][dk as usize] = self.ranfloat[self.perm_x
                        [i as usize]
                        ^ self.perm_y[j as usize]
                        ^ self.perm_z[k as usize]];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[Float; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as Float * u + (1 - i) as Float * (1.0 - u))
                        * (j as Float * v + (1 - j) as Float * (1.0 - v))
                        * (k as Float * w + (1 - k) as Float * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
}

impl<const POINT_COUNT: usize> Texture for Perlin<POINT_COUNT> {
    fn value(&self, _u: Float, _v: Float, point: Point3<Float>) -> Color {
        return Color(self.noise(point) * vec3(1.0, 1.0, 1.0));
    }
}
