use std::{fmt::Display, ops::Deref};

use crate::Float;
use cgmath::{vec3, Vector3};
use rand::{distributions::Standard, prelude::Distribution};

#[derive(Clone, Copy, Debug)]
pub struct Color(pub Vector3<Float>);

#[derive(Clone, Copy, Debug)]
pub struct SampledColor(Vector3<Float>);

impl Color {
    pub fn into_sampled(self, sample_per_pixel: usize) -> SampledColor {
        let scale = 1.0 / sample_per_pixel as Float;
        let r = if self.0.x.is_nan() { 0.0 } else { self.0.x };
        let g = if self.0.y.is_nan() { 0.0 } else { self.0.y };
        let b = if self.0.z.is_nan() { 0.0 } else { self.0.z };

        SampledColor(vec3(r, g, b) * scale)
    }
}

impl Deref for Color {
    type Target = Vector3<Float>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        Color(vec3(rng.gen(), rng.gen(), rng.gen()))
    }
}

impl Deref for SampledColor {
    type Target = Vector3<Float>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self[0]) as usize,
            (255.999 * self[1]) as usize,
            (255.999 * self[2]) as usize
        )
    }
}

impl Display for SampledColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (256.0 * self[0].sqrt().clamp(0.0, 0.999)) as usize,
            (256.0 * self[1].sqrt().clamp(0.0, 0.999)) as usize,
            (256.0 * self[2].sqrt().clamp(0.0, 0.999)) as usize
        )
    }
}
