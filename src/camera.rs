use cgmath::{Angle, Deg, EuclideanSpace, InnerSpace, Point3, Rad, Vector3};

use crate::{ray::Ray, Float};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3<Float>,
    lower_left_corner: Point3<Float>,
    horizontal: Vector3<Float>,
    vertical: Vector3<Float>,
}

impl Camera {
    pub fn new(
        look_from: Point3<Float>,
        look_at: Point3<Float>,
        vup: Vector3<Float>,
        vfov: Deg<Float>,
        aspect_ratio: Float,
    ) -> Self {
        let theta: Rad<Float> = vfov.into();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = InnerSpace::normalize(look_from - look_at);
        let u = InnerSpace::normalize(vup.cross(w));
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        Ray {
            origin: self.origin,
            direction: EuclideanSpace::to_vec(
                self.lower_left_corner + s * self.horizontal + t * self.vertical
                    - EuclideanSpace::to_vec(self.origin),
            ),
        }
    }
}
