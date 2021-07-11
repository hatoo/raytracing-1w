type Float = f64;
type MyRng = StdRng;

mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod ray;
mod sphere;

use std::sync::Arc;

use cgmath::{point3, prelude::*, vec3};
use color::Color;
use hittable::Hittable;
use rand::prelude::*;
use ray::Ray;

use crate::{
    camera::Camera,
    material::{Lambertian, Material, Metal},
    sphere::Sphere,
};

fn ray_color<H: Hittable + ?Sized>(ray: &Ray, world: &H, depth: usize, rng: &mut MyRng) -> Color {
    if depth == 0 {
        return Color(vec3(0.0, 0.0, 0.0));
    }
    if let Some(hit_record) = world.hit(ray, 0.001, Float::INFINITY) {
        return if let Some((color, scatterd)) = hit_record.material.scatter(ray, &hit_record, rng) {
            Color(
                color
                    .0
                    .mul_element_wise(ray_color(&scatterd, world, depth - 1, rng).0),
            )
        } else {
            Color(vec3(0.0, 0.0, 0.0))
        };
    }
    let unit_direction = InnerSpace::normalize(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color(vec3(1.0, 1.0, 1.0).lerp(vec3(0.5, 0.7, 1.0), t))
}

fn main() {
    const ASPECT_RATIO: Float = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as Float / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let material_ground: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: Color(vec3(0.8, 0.8, 0.0)),
    }));

    let material_center: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: Color(vec3(0.7, 0.3, 0.3)),
    }));

    let material_left: Arc<Box<dyn Material>> = Arc::new(Box::new(Metal {
        albedo: Color(vec3(0.8, 0.8, 0.8)),
        fuzz: 0.3,
    }));

    let material_right: Arc<Box<dyn Material>> = Arc::new(Box::new(Metal {
        albedo: Color(vec3(0.8, 0.6, 0.2)),
        fuzz: 1.0,
    }));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: point3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground,
        }),
        Box::new(Sphere {
            center: point3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_center,
        }),
        Box::new(Sphere {
            center: point3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left,
        }),
        Box::new(Sphere {
            center: point3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right,
        }),
    ];

    let camera = Camera::default();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = MyRng::from_entropy();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color(vec3(0.0, 0.0, 0.0));

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as Float + rng.gen::<Float>()) / (IMAGE_WIDTH - 1) as Float;
                let v = (j as Float + rng.gen::<Float>()) / (IMAGE_HEIGHT - 1) as Float;

                let ray = camera.get_ray(u, v);
                pixel_color =
                    Color(pixel_color.0 + ray_color(&ray, world.as_slice(), MAX_DEPTH, &mut rng).0);
            }

            println!("{}", pixel_color.into_sampled(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("\nDone");
}
