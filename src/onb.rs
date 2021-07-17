use cgmath::{vec3, InnerSpace, Vector3};

use crate::Float;

pub struct Onb {
    pub u: Vector3<Float>,
    pub v: Vector3<Float>,
    pub w: Vector3<Float>,
}

impl Onb {
    #[allow(clippy::many_single_char_names)]
    pub fn from_w(n: Vector3<Float>) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            vec3(0.0, 1.0, 0.0)
        } else {
            vec3(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);

        Self { u, v, w }
    }

    pub fn local(&self, a: Vector3<Float>) -> Vector3<Float> {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}
