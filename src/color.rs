use std::{fmt::Display, ops::Deref};

use crate::Float;
use cgmath::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub Vector3<Float>);

impl Deref for Color {
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
