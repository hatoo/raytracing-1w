type Float = f64;

mod color;
mod hittable;
mod ray;
mod sphere;

use cgmath::{point3, prelude::*, vec3};
use color::Color;
use hittable::Hittable;
use ray::Ray;

use crate::sphere::Sphere;

fn ray_color<H: Hittable + ?Sized>(ray: &Ray, h: &H) -> Color {
    if let Some(hit_record) = h.hit(ray, 0.0, Float::INFINITY) {
        return Color(0.5 * (hit_record.normal + vec3(1.0, 1.0, 1.0)));
    }
    let unit_direction = InnerSpace::normalize(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0))
}

fn main() {
    const ASPECT_RATIO: Float = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as Float / ASPECT_RATIO) as usize;

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

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = point3(0.0, 0.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as Float / (IMAGE_WIDTH - 1) as Float;
            let v = j as Float / (IMAGE_HEIGHT - 1) as Float;

            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let color = ray_color(&ray, world.as_slice());

            println!("{}", color);
        }
    }

    eprintln!("\nDone");
}
