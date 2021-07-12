use std::mem::swap;

use crate::{ray::Ray, Float};
use cgmath::{point3, Point3};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub minimum: Point3<Float>,
    pub maximum: Point3<Float>,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, mut t_min: Float, mut t_max: Float) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.minimum[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = point3(
        box0.minimum.x.min(box1.minimum.x),
        box0.minimum.y.min(box1.minimum.y),
        box0.minimum.z.min(box1.minimum.z),
    );

    let big = point3(
        box0.maximum.x.max(box1.maximum.x),
        box0.maximum.y.max(box1.maximum.y),
        box0.maximum.z.max(box1.maximum.z),
    );

    AABB {
        minimum: small,
        maximum: big,
    }
}
