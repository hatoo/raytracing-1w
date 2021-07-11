type Float = f64;

mod color;
mod ray;

use cgmath::{dot, point3, prelude::*, vec3, Point3, Vector3};
use color::Color;
use ray::Ray;

fn hit_sphere(center: &Point3<Float>, radius: Float, r: &Ray) -> Option<Float> {
    let oc = r.origin - center;
    let a = InnerSpace::magnitude2(r.direction);
    let half_b = dot(oc, r.direction);
    let c = InnerSpace::magnitude2(oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

fn ray_color(ray: &Ray) -> Color {
    if let Some(t) = hit_sphere(&point3(0.0, 0.0, -1.0), 0.5, ray) {
        let n: Vector3<Float> =
            InnerSpace::normalize(EuclideanSpace::to_vec(ray.at(t)) - vec3(0.0, 0.0, -1.0));
        return Color(0.5 * vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0));
    }
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
