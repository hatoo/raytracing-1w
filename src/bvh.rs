use float_ord::FloatOrd;
use rand::Rng;

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::Hittable,
    Float,
};

enum BVHChild<R: Rng> {
    One(Box<dyn Hittable<R = R>>),
    Two(Box<dyn Hittable<R = R>>, Box<dyn Hittable<R = R>>),
}

pub struct BVHNode<R: Rng> {
    child: BVHChild<R>,
    aabb: AABB,
}

impl<R: 'static + Rng + Send + Sync> Hittable for BVHNode<R> {
    type R = R;

    fn bounding_box(&self, _time0: crate::Float, _time1: crate::Float) -> Option<AABB> {
        Some(self.aabb)
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
        rng: &mut R,
    ) -> Option<crate::hittable::HitRecord<R>> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        match &self.child {
            BVHChild::One(obj) => obj.hit(ray, t_min, t_max, rng),
            BVHChild::Two(left, right) => {
                if let Some(hit_left) = left.hit(ray, t_min, t_max, rng) {
                    if let Some(hit_right) = right.hit(ray, t_min, hit_left.t, rng) {
                        Some(hit_right)
                    } else {
                        Some(hit_left)
                    }
                } else {
                    right.hit(ray, t_min, t_max, rng)
                }
            }
        }
    }
}

impl<R: Rng + 'static + Send + Sync> BVHNode<R> {
    pub fn new(
        mut objects: Vec<Box<dyn Hittable<R = R>>>,
        time0: Float,
        time1: Float,
        rng: &mut R,
    ) -> Self {
        match objects.len() {
            0 => panic!("objects mut not be empty"),
            1 => {
                let obj = objects.pop().unwrap();
                Self {
                    aabb: obj
                        .bounding_box(time0, time1)
                        .expect("Bounding Box is required"),
                    child: BVHChild::One(obj),
                }
            }
            2 => {
                let left = objects.pop().unwrap();
                let right = objects.pop().unwrap();
                let left_box = left.bounding_box(time0, time1).unwrap();
                let right_box = right.bounding_box(time0, time1).unwrap();
                let aabb = surrounding_box(left_box, right_box);

                Self {
                    child: BVHChild::Two(left, right),
                    aabb,
                }
            }
            len => {
                let axis = rng.gen_range(0..=2);
                objects
                    .sort_by_key(|o| FloatOrd(o.bounding_box(time0, time1).unwrap().minimum[axis]));
                let right = objects.split_off(len / 2);

                let left = objects;
                let right = right;

                let left = Self::new(left, time0, time1, rng);
                let right = Self::new(right, time0, time1, rng);

                let aabb = surrounding_box(left.aabb, right.aabb);

                Self {
                    child: BVHChild::Two(Box::new(left), Box::new(right)),
                    aabb,
                }
            }
        }
    }
}
