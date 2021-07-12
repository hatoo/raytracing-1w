use cgmath::{vec3, Point3};
use rand::Rng;
use std::fmt::Debug;

use crate::{color::Color, perlin::Perlin, Float};

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: Float, v: Float, point: Point3<Float>) -> Color;
}

#[derive(Debug)]
pub struct SolidColor {
    pub color_value: Color,
}

#[derive(Debug)]
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

#[derive(Debug)]
pub struct NoiseTexture<const POINT_COUNT: usize> {
    perlin: Perlin<POINT_COUNT>,
    scale: Float,
}

pub type NoiseTexture256 = NoiseTexture<256>;

impl<const POINT_COUNT: usize> NoiseTexture<POINT_COUNT> {
    pub fn new(scale: Float, rng: &mut impl Rng) -> Self {
        Self {
            perlin: Perlin::new(rng),
            scale,
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: Float, _v: Float, _point: Point3<Float>) -> Color {
        self.color_value
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: Float, v: Float, point: Point3<Float>) -> Color {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

impl<const POINT_COUNT: usize> Texture for NoiseTexture<POINT_COUNT> {
    fn value(&self, _u: Float, _v: Float, point: Point3<Float>) -> Color {
        Color(vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.perlin.noise(self.scale * point)))
    }
}
