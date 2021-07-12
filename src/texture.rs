use cgmath::{vec3, Point3};
use image::{DynamicImage, GenericImageView};
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
        Color(
            vec3(1.0, 1.0, 1.0)
                * 0.5
                * (1.0 + (self.scale * point.z + 10.0 * self.perlin.turb(point, 7)).sin()),
        )
    }
}

impl Texture for DynamicImage {
    fn value(&self, u: Float, v: Float, _point: Point3<Float>) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let (width, height) = self.dimensions();
        let i = (u * width as Float) as u32;
        let j = (v * height as Float) as u32;

        let i = i.min(width - 1);
        let j = j.min(height - 1);

        let pixel = self.get_pixel(i, j);

        const COLOR_SCALE: Float = 1.0 / 255.0;

        Color(vec3(
            pixel.0[0] as Float * COLOR_SCALE,
            pixel.0[1] as Float * COLOR_SCALE,
            pixel.0[2] as Float * COLOR_SCALE,
        ))
    }
}
