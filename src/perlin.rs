use cgmath::{dot, vec3, InnerSpace, Point3, Vector3};
use rand::{prelude::SliceRandom, Rng};

use crate::{color::Color, texture::Texture, Float};

#[derive(Debug)]
pub struct Perlin<const POINT_COUNT: usize> {
    ranvec: [Vector3<Float>; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

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
        let mut ranvec = [vec3(0.0, 0.0, 0.0); POINT_COUNT];

        for v in ranvec.iter_mut() {
            *v = InnerSpace::normalize(vec3(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ));
        }

        Self {
            ranvec,
            perm_x: Self::generate_perm(rng),
            perm_y: Self::generate_perm(rng),
            perm_z: Self::generate_perm(rng),
        }
    }

    pub fn noise(&self, p: Point3<Float>) -> Float {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = [[[vec3(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = (i + di) & (POINT_COUNT as isize - 1);
                    let j = (j + dj) & (POINT_COUNT as isize - 1);
                    let k = (k + dk) & (POINT_COUNT as isize - 1);

                    c[di as usize][dj as usize][dk as usize] = self.ranvec[self.perm_x[i as usize]
                        ^ self.perm_y[j as usize]
                        ^ self.perm_z[k as usize]];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3<Float>, depth: usize) -> Float {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_interp(c: [[[Vector3<Float>; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = vec3(u - i as Float, v - j as Float, w - k as Float);
                    accum += (i as Float * uu + (1 - i) as Float * (1.0 - uu))
                        * (j as Float * vv + (1 - j) as Float * (1.0 - vv))
                        * (k as Float * ww + (1 - k) as Float * (1.0 - ww))
                        * dot(c[i][j][k], weight_v);
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
