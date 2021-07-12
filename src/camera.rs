use cgmath::{point3, vec3, Angle, Deg, EuclideanSpace, Point3, Rad, Vector3};

use crate::{ray::Ray, Float};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3<Float>,
    lower_left_corner: Point3<Float>,
    horizontal: Vector3<Float>,
    vertical: Vector3<Float>,
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: Float = 16.0 / 9.0;

        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = point3(0.0, 0.0, 0.0);
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}

impl Camera {
    pub fn new(vfov: Deg<Float>, aspect_ratio: Float) -> Self {
        let theta: Rad<Float> = vfov.into();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = point3(0.0, 0.0, 0.0);
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        Ray {
            origin: self.origin,
            direction: EuclideanSpace::to_vec(
                self.lower_left_corner + u * self.horizontal + v * self.vertical,
            ),
        }
    }
}
