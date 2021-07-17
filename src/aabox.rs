use std::sync::Arc;

use cgmath::Point3;
use rand::Rng;

use crate::{
    aabb::AABB,
    aarect::{XYRect, XZRect, YZRect},
    bvh::BVHNode,
    hittable::Hittable,
    material::Material,
    Float,
};

pub struct AABox<R> {
    box_min: Point3<Float>,
    box_max: Point3<Float>,
    sides: BVHNode<R>,
}

impl<R: 'static + Rng + Send + Sync> AABox<R> {
    pub fn new(
        p0: Point3<Float>,
        p1: Point3<Float>,
        material: Arc<Box<dyn Material<R = R>>>,
        rng: &mut R,
    ) -> Self {
        let sides: Vec<Box<dyn Hittable<R = R>>> = vec![
            Box::new(XYRect {
                x0: p0.x,
                x1: p1.x,
                y0: p0.y,
                y1: p1.y,
                k: p1.z,
                material: material.clone(),
            }),
            Box::new(XYRect {
                x0: p0.x,
                x1: p1.x,
                y0: p0.y,
                y1: p1.y,
                k: p0.z,
                material: material.clone(),
            }),
            Box::new(XZRect {
                x0: p0.x,
                x1: p1.x,
                z0: p0.z,
                z1: p1.z,
                k: p1.y,
                material: material.clone(),
            }),
            Box::new(XZRect {
                x0: p0.x,
                x1: p1.x,
                z0: p0.z,
                z1: p1.z,
                k: p0.y,
                material: material.clone(),
            }),
            Box::new(YZRect {
                y0: p0.y,
                y1: p1.y,
                z0: p0.z,
                z1: p1.z,
                k: p1.x,
                material: material.clone(),
            }),
            Box::new(YZRect {
                y0: p0.y,
                y1: p1.y,
                z0: p0.z,
                z1: p1.z,
                k: p0.x,
                material,
            }),
        ];

        Self {
            box_min: p0,
            box_max: p1,
            sides: BVHNode::new(sides, 0.0, 1.0, rng),
        }
    }
}

impl<R: 'static + Rng + Send + Sync> Hittable for AABox<R> {
    type R = R;
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: Float,
        t_max: Float,
        rng: &mut R,
    ) -> Option<crate::hittable::HitRecord<R>> {
        self.sides.hit(ray, t_min, t_max, rng)
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        Some(AABB {
            minimum: self.box_min,
            maximum: self.box_max,
        })
    }
}
