use std::sync::Arc;

use crate::aabb::{surrounding_box, AABB};
use crate::{material::Material, ray::Ray};
use crate::{Float, MyRng};
use cgmath::{dot, point3, vec3, Angle, Deg, Point3, Rad, Vector3};
use rand::prelude::SliceRandom;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub position: Point3<Float>,
    pub normal: Vector3<Float>,
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub front_face: bool,
    pub material: Arc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        position: Point3<Float>,
        outward_normal: Vector3<Float>,
        t: Float,
        u: Float,
        v: Float,
        ray: &Ray,
        material: Arc<Box<dyn Material>>,
    ) -> Self {
        let front_face = dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            position,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        }
    }
}

pub struct Translate<T> {
    pub hittable: T,
    pub offset: Vector3<Float>,
}

pub struct RotateY<T> {
    hittable: T,
    sin_theta: Float,
    cos_theta: Float,
    aabb: Option<AABB>,
}

pub struct FlipFace<T>(pub T);

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord>;
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB>;
    fn pdf_value(&self, _o: Point3<Float>, _v: Vector3<Float>, _rng: &mut MyRng) -> Float {
        0.0
    }
    fn random(&self, _o: Vector3<Float>, _rng: &mut MyRng) -> Vector3<Float> {
        vec3(1.0, 0.0, 0.0)
    }
}

impl<T: Hittable + ?Sized> Hittable for &T {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord> {
        (*self).hit(ray, t_min, t_max, rng)
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        (*self).bounding_box(time0, time1)
    }

    fn pdf_value(&self, o: Point3<Float>, v: Vector3<Float>, rng: &mut MyRng) -> Float {
        (*self).pdf_value(o, v, rng)
    }

    fn random(&self, o: Vector3<Float>, rng: &mut MyRng) -> Vector3<Float> {
        (*self).random(o, rng)
    }
}

impl<T: Hittable + ?Sized> Hittable for Box<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max, rng)
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.as_ref().bounding_box(time0, time1)
    }

    fn pdf_value(&self, o: Point3<Float>, v: Vector3<Float>, rng: &mut MyRng) -> Float {
        self.as_ref().pdf_value(o, v, rng)
    }

    fn random(&self, o: Vector3<Float>, rng: &mut MyRng) -> Vector3<Float> {
        self.as_ref().random(o, rng)
    }
}

impl<T: Hittable> Hittable for [T] {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for hittable in self {
            if let Some(new_hit_record) = hittable.hit(ray, t_min, closest_so_far, rng) {
                closest_so_far = new_hit_record.t;
                hit_record = Some(new_hit_record);
            }
        }

        hit_record
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        let mut b = None;

        for hittable in self {
            if let Some(b0) = hittable.bounding_box(time0, time1) {
                b = Some(if let Some(b) = b {
                    surrounding_box(b, b0)
                } else {
                    b0
                });
            } else {
                {
                    return None;
                }
            }
        }
        b
    }

    fn pdf_value(&self, o: Point3<Float>, v: Vector3<Float>, rng: &mut MyRng) -> Float {
        let weight = 1.0 / self.len() as Float;

        self.iter()
            .map(|hittable| weight * hittable.pdf_value(o, v, rng))
            .sum()
    }

    fn random(&self, o: Vector3<Float>, rng: &mut MyRng) -> Vector3<Float> {
        self.choose(rng).unwrap().random(o, rng)
    }
}

impl<T: Hittable> RotateY<T> {
    pub fn new(hittable: T, time0: Float, time1: Float, angle: Deg<Float>) -> Self {
        let radians = Into::<Rad<Float>>::into(angle);
        let (sin_theta, cos_theta) = radians.sin_cos();

        let bbox = hittable.bounding_box(time0, time1);

        let bbox = bbox.map(|bbox| {
            let mut min = point3(Float::INFINITY, Float::INFINITY, Float::INFINITY);
            let mut max = point3(
                Float::NEG_INFINITY,
                Float::NEG_INFINITY,
                Float::NEG_INFINITY,
            );
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as Float * bbox.maximum.x + (1.0 - i as Float) * bbox.minimum.x;
                        let y = j as Float * bbox.maximum.y + (1.0 - j as Float) * bbox.minimum.y;
                        let z = k as Float * bbox.maximum.z + (1.0 - k as Float) * bbox.minimum.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = vec3(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            AABB {
                minimum: min,
                maximum: max,
            }
        });

        Self {
            hittable,
            sin_theta,
            cos_theta,
            aabb: bbox,
        }
    }
}

impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord> {
        let moved = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        };

        self.hittable
            .hit(&moved, t_min, t_max, rng)
            .map(|hit_record| {
                HitRecord::new(
                    hit_record.position + self.offset,
                    hit_record.normal,
                    hit_record.t,
                    hit_record.u,
                    hit_record.v,
                    &moved,
                    hit_record.material,
                )
            })
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.hittable.bounding_box(time0, time1).map(|aabb| AABB {
            minimum: aabb.minimum + self.offset,
            maximum: aabb.maximum + self.offset,
        })
    }
}

impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_r = Ray {
            origin,
            direction,
            time: ray.time,
        };

        self.hittable
            .hit(&rotated_r, t_min, t_max, rng)
            .map(|hit_record| {
                let mut p = hit_record.position;
                let mut normal = hit_record.normal;

                p[0] = self.cos_theta * hit_record.position[0]
                    + self.sin_theta * hit_record.position[2];
                p[2] = -self.sin_theta * hit_record.position[0]
                    + self.cos_theta * hit_record.position[2];

                normal[0] =
                    self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
                normal[2] =
                    -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

                HitRecord::new(
                    p,
                    normal,
                    hit_record.t,
                    hit_record.u,
                    hit_record.v,
                    &rotated_r,
                    hit_record.material,
                )
            })
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        self.aabb
    }
}

impl<T: Hittable> Hittable for FlipFace<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut MyRng) -> Option<HitRecord> {
        self.0.hit(ray, t_min, t_max, rng).map(|mut hit_record| {
            hit_record.front_face = !hit_record.front_face;
            hit_record
        })
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.0.bounding_box(time0, time1)
    }
}
