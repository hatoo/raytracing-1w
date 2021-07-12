use cgmath::Point3;
use std::fmt::Debug;

use crate::{color::Color, Float};

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
