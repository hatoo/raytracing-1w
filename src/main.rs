type Float = f64;
type MyRng = StdRng;

mod aabb;
mod bvd;
mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod moving_sphere;
mod perlin;
mod ray;
mod sphere;
mod texture;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use cgmath::{point3, prelude::*, vec3, Deg};
use color::Color;
use hittable::Hittable;
use image::load_from_memory;
use material::Scatter;
use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;

use crate::{
    bvd::BVHNode,
    camera::Camera,
    color::SampledColor,
    material::{Dielectric, Lambertian, Material, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    texture::{CheckerTexture, NoiseTexture256, SolidColor},
};

fn ray_color<H: Hittable + ?Sized>(
    ray: &Ray,
    background: Color,
    world: &H,
    depth: usize,
    rng: &mut MyRng,
) -> Color {
    if depth == 0 {
        return Color(vec3(0.0, 0.0, 0.0));
    }
    if let Some(hit_record) = world.hit(ray, 0.001, Float::INFINITY) {
        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, hit_record.position);

        return if let Some(Scatter {
            color,
            ray: scatterd,
        }) = hit_record.material.scatter(ray, &hit_record, rng)
        {
            Color(
                emitted.0
                    + color.0.mul_element_wise(
                        ray_color(&scatterd, background, world, depth - 1, rng).0,
                    ),
            )
        } else {
            emitted
        };
    } else {
        background
    }
}

fn random_scene(rng: &mut impl Rng) -> BVHNode {
    let ground_material: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: Box::new(CheckerTexture {
            even: Box::new(SolidColor {
                color_value: Color(vec3(0.2, 0.3, 0.1)),
            }),
            odd: Box::new(SolidColor {
                color_value: Color(vec3(0.9, 0.9, 0.9)),
            }),
        }),
    }));

    let mut world: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere {
        center: point3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    })];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: Float = rng.gen();
            let center = point3(
                a as Float + 0.9 * rng.gen::<Float>(),
                0.2,
                b as Float + 0.9 * rng.gen::<Float>(),
            );

            if InnerSpace::magnitude(center - point3(4.0, 0.2, 0.0)) > 0.9 {
                let hittable: Box<dyn Hittable> = match choose_mat {
                    x if x < 0.8 => {
                        let albedo =
                            Color(rng.gen::<Color>().0.mul_element_wise(rng.gen::<Color>().0));
                        let center2 = center + vec3(0.0, rng.gen_range(0.0..0.5), 0.0);
                        let material: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
                            albedo: Box::new(SolidColor {
                                color_value: albedo,
                            }),
                        }));
                        Box::new(MovingSphere {
                            center0: center,
                            center1: center2,
                            time0: 0.0,
                            time1: 1.0,
                            radius: 0.2,
                            material,
                        })
                    }
                    x if x < 0.95 => {
                        let albedo = Color(vec3(
                            rng.gen_range(0.5..1.0),
                            rng.gen_range(0.5..1.0),
                            rng.gen_range(0.5..1.0),
                        ));
                        let fuzz = rng.gen_range(0.5..1.0);
                        let material: Arc<Box<dyn Material>> =
                            Arc::new(Box::new(Metal { albedo, fuzz }));
                        Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material,
                        })
                    }
                    _ => {
                        let material: Arc<Box<dyn Material>> =
                            Arc::new(Box::new(Dielectric { ir: 1.5 }));
                        Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material,
                        })
                    }
                };
                world.push(hittable);
            }
        }
    }

    world.push(Box::new(Sphere {
        center: point3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Box::new(Dielectric { ir: 1.5 })),
    }));

    world.push(Box::new(Sphere {
        center: point3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Box::new(Lambertian {
            albedo: Box::new(SolidColor {
                color_value: Color(vec3(0.4, 0.2, 0.1)),
            }),
        })),
    }));

    world.push(Box::new(Sphere {
        center: point3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Box::new(Metal {
            albedo: Color(vec3(0.7, 0.6, 0.5)),
            fuzz: 0.0,
        })),
    }));

    BVHNode::new(world, 0.0, 1.0, rng)
}

fn two_spheres(rng: &mut impl Rng) -> BVHNode {
    let checker_material: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: Box::new(CheckerTexture {
            even: Box::new(SolidColor {
                color_value: Color(vec3(0.2, 0.3, 0.1)),
            }),
            odd: Box::new(SolidColor {
                color_value: Color(vec3(0.9, 0.9, 0.9)),
            }),
        }),
    }));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: point3(0.0, -10.0, 0.0),
            radius: 10.0,
            material: checker_material.clone(),
        }),
        Box::new(Sphere {
            center: point3(0.0, 10.0, 0.0),
            radius: 10.0,
            material: checker_material.clone(),
        }),
    ];

    BVHNode::new(world, 0.0, 1.0, rng)
}

fn two_perlin_spheres(rng: &mut impl Rng) -> BVHNode {
    let pertext: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: Box::new(NoiseTexture256::new(4.0, rng)),
    }));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: point3(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: pertext.clone(),
        }),
        Box::new(Sphere {
            center: point3(0.0, 2.0, 0.0),
            radius: 2.0,
            material: pertext.clone(),
        }),
    ];

    BVHNode::new(world, 0.0, 1.0, rng)
}

fn earth(rng: &mut impl Rng) -> BVHNode {
    const EARTH_JPG: &[u8] = include_bytes!("../assets/earthmap.jpg");
    let image = load_from_memory(EARTH_JPG).unwrap();
    let earth_surface: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: Box::new(image),
    }));

    let globe = Box::new(Sphere {
        center: point3(0.0, 0.0, 0.0),
        radius: 2.0,
        material: earth_surface,
    });

    BVHNode::new(vec![globe], 0.0, 1.0, rng)
}

fn main() {
    const ASPECT_RATIO: Float = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as Float / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let mut rng = MyRng::from_entropy();

    let (world, background, look_from, look_at, vfov, aperture) = match 3 {
        0 => (
            random_scene(&mut rng),
            Color(vec3(0.70, 0.80, 1.00)),
            point3(13.0, 2.0, 3.0),
            point3(0.0, 0.0, 0.0),
            Deg(20.0),
            0.1,
        ),
        1 => (
            two_spheres(&mut rng),
            Color(vec3(0.70, 0.80, 1.00)),
            point3(13.0, 2.0, 3.0),
            point3(0.0, 0.0, 0.0),
            Deg(20.0),
            0.0,
        ),
        2 => (
            two_perlin_spheres(&mut rng),
            Color(vec3(0.70, 0.80, 1.00)),
            point3(13.0, 2.0, 3.0),
            point3(0.0, 0.0, 0.0),
            Deg(20.0),
            0.0,
        ),
        _ => (
            earth(&mut rng),
            Color(vec3(0.70, 0.80, 1.00)),
            point3(13.0, 2.0, 3.0),
            point3(0.0, 0.0, 0.0),
            Deg(20.0),
            0.0,
        ),
    };

    let vup = vec3(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        10.0,
        0.0,
        1.0,
    );

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let sacans_remaining = AtomicUsize::new(IMAGE_HEIGHT);

    let image: Vec<Vec<SampledColor>> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(|j| {
            let row = (0..IMAGE_WIDTH)
                .into_par_iter()
                .map(|i| {
                    let mut rng = MyRng::seed_from_u64((j * IMAGE_WIDTH + i) as u64);
                    let mut pixel_color = Color(vec3(0.0, 0.0, 0.0));

                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as Float + rng.gen::<Float>()) / (IMAGE_WIDTH - 1) as Float;
                        let v = (j as Float + rng.gen::<Float>()) / (IMAGE_HEIGHT - 1) as Float;

                        let ray = camera.get_ray(u, v, &mut rng);
                        pixel_color = Color(
                            pixel_color.0
                                + ray_color(&ray, background, &world, MAX_DEPTH, &mut rng).0,
                        );
                    }

                    pixel_color.into_sampled(SAMPLES_PER_PIXEL)
                })
                .collect();
            eprint!(
                "\rScanlines remaining: {} ",
                sacans_remaining.fetch_sub(1, Ordering::Relaxed) - 1
            );
            row
        })
        .collect();

    for row in image {
        for color in row {
            println!("{}", color);
        }
    }

    eprintln!("\nDone");
}
