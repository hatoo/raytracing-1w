use cgmath::{Angle, Deg, EuclideanSpace, InnerSpace, Point3, Rad, Vector3};
use rand::Rng;

use crate::{math::random_vec3_in_unit_disk, ray::Ray, Float};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3<Float>,
    lower_left_corner: Point3<Float>,
    horizontal: Vector3<Float>,
    vertical: Vector3<Float>,
    u: Vector3<Float>,
    v: Vector3<Float>,
    w: Vector3<Float>,
    lens_radius: Float,
    time0: Float,
    time1: Float,
}

impl Camera {
    pub fn new(
        look_from: Point3<Float>,
        look_at: Point3<Float>,
        vup: Vector3<Float>,
        vfov: Deg<Float>,
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
        time0: Float,
        time1: Float,
    ) -> Self {
        let theta: Rad<Float> = vfov.into();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = InnerSpace::normalize(look_from - look_at);
        let u = InnerSpace::normalize(vup.cross(w));
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * random_vec3_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: EuclideanSpace::to_vec(
                self.lower_left_corner + s * self.horizontal + t * self.vertical
                    - EuclideanSpace::to_vec(self.origin)
                    - offset,
            ),
            time: rng.gen_range(self.time0..self.time1),
        }
    }
}
