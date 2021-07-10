type Float = f64;

mod color;
mod ray;

use cgmath::{point3, prelude::*, vec3};
use color::Color;
use ray::Ray;

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = InnerSpace::normalize(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0))
}

fn main() {
    const ASPECT_RATIO: Float = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as Float / ASPECT_RATIO) as usize;

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

            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let color = ray_color(&r);

            println!("{}", color);
        }
    }

    eprintln!("\nDone");
}
