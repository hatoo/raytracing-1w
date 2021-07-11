type Float = f64;

mod camera;
mod color;
mod hittable;
mod math;
mod ray;
mod sphere;

use cgmath::{point3, prelude::*, vec3};
use color::Color;
use hittable::Hittable;
use math::random_vec3_in_unit_sphere;
use rand::prelude::*;
use ray::Ray;

use crate::{camera::Camera, sphere::Sphere};

fn ray_color<H: Hittable + ?Sized>(
    ray: &Ray,
    world: &H,
    depth: usize,
    rng: &mut impl Rng,
) -> Color {
    if depth == 0 {
        return Color(vec3(0.0, 0.0, 0.0));
    }
    if let Some(hit_record) = world.hit(ray, 0.001, Float::INFINITY) {
        let target = hit_record.position + hit_record.normal + random_vec3_in_unit_sphere(rng);
        let new_ray = Ray {
            origin: hit_record.position,
            direction: target - hit_record.position,
        };
        return Color(0.5 * ray_color(&new_ray, world, depth - 1, rng).0);
    }
    let unit_direction = InnerSpace::normalize(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0))
}

fn main() {
    const ASPECT_RATIO: Float = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as Float / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: point3(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: point3(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    let camera = Camera::default();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = StdRng::from_entropy();

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
