type Float = f64;
type MyRng = StdRng;

mod aabb;
mod aabox;
mod aarect;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod material;
mod math;
mod moving_sphere;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod sphere;
mod texture;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use cgmath::{dot, point3, prelude::*, vec3, Deg};
use color::Color;
use hittable::Hittable;
use image::load_from_memory;
use material::{Scatter, ScatterKind};
use onb::Onb;
use pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf};
use rand::{distributions::weighted::alias_method, prelude::*};
use ray::Ray;
use rayon::prelude::*;

use crate::{
    aabox::AABox,
    aarect::{XYRect, XZRect, YZRect},
    bvh::BVHNode,
    camera::Camera,
    color::SampledColor,
    constant_medium::ConstantMedium,
    hittable::{FlipFace, RotateY, Translate},
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    texture::{CheckerTexture, NoiseTexture256, SolidColor},
};

fn ray_color<H: Hittable, L: Hittable>(
    ray: &Ray,
    background: Color,
    world: &H,
    lights: &L,
    depth: usize,
    rng: &mut MyRng,
) -> Color {
    if depth == 0 {
        return Color(vec3(0.0, 0.0, 0.0));
    }
    if let Some(hit_record) = world.hit(ray, 0.001, Float::INFINITY, rng) {
        let emitted = hit_record.material.emitted(
            ray,
            &hit_record,
            hit_record.u,
            hit_record.v,
            hit_record.position,
        );

        return if let Some(Scatter { attenuation, kind }) =
            hit_record.material.scatter(ray, &hit_record, rng)
        {
            match kind {
                ScatterKind::Pdf(pdf) => {
                    let p0 = HittablePdf {
                        hittable: lights,
                        o: hit_record.position,
                    };

                    let mixed_pdf = MixturePdf { p0, p1: pdf };

                    let scatterd = Ray {
                        origin: hit_record.position,
                        direction: mixed_pdf.generate(rng),
                        time: hit_record.t,
                    };

                    let pdf = mixed_pdf.value(scatterd.direction, rng);

                    Color(
                        emitted.0
                            + (attenuation.0
                                * hit_record.material.scattering_pdf(
                                    ray,
                                    &hit_record,
                                    &scatterd,
                                    rng,
                                ))
                            .mul_element_wise(
                                ray_color(&scatterd, background, world, lights, depth - 1, rng).0
                                    / pdf,
                            ),
                    )
                }
                ScatterKind::Spacular(specular_ray) => Color(attenuation.0.mul_element_wise(
                    ray_color(&specular_ray, background, world, lights, depth - 1, rng).0,
                )),
            }
        } else {
            emitted
        };
    } else {
        background
    }
}

fn random_scene(rng: &mut impl Rng) -> BVHNode {
    let ground_material: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: CheckerTexture {
            even: Box::new(SolidColor {
                color_value: Color(vec3(0.2, 0.3, 0.1)),
            }),
            odd: Box::new(SolidColor {
                color_value: Color(vec3(0.9, 0.9, 0.9)),
            }),
        },
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
                            albedo: SolidColor {
                                color_value: albedo,
                            },
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
            albedo: SolidColor {
                color_value: Color(vec3(0.4, 0.2, 0.1)),
            },
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
        albedo: CheckerTexture {
            even: Box::new(SolidColor {
                color_value: Color(vec3(0.2, 0.3, 0.1)),
            }),
            odd: Box::new(SolidColor {
                color_value: Color(vec3(0.9, 0.9, 0.9)),
            }),
        },
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
        albedo: NoiseTexture256::new(4.0, rng),
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
    let earth_surface: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian { albedo: image }));

    let globe = Box::new(Sphere {
        center: point3(0.0, 0.0, 0.0),
        radius: 2.0,
        material: earth_surface,
    });

    BVHNode::new(vec![globe], 0.0, 1.0, rng)
}

fn simple_light(rng: &mut impl Rng) -> BVHNode {
    let pertext: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: NoiseTexture256::new(4.0, rng),
    }));

    let difflight: Arc<Box<dyn Material>> = Arc::new(Box::new(DiffuseLight {
        emit: SolidColor {
            color_value: Color(vec3(4.0, 4.0, 4.0)),
        },
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
        Box::new(XYRect {
            x0: 3.0,
            x1: 5.0,
            y0: 1.0,
            y1: 3.0,
            k: -2.0,
            material: difflight,
        }),
    ];

    BVHNode::new(world, 0.0, 1.0, rng)
}

fn cornel_box(rng: &mut impl Rng) -> BVHNode {
    let red: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.65, 0.05, 0.05)),
        },
    }));

    let white: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.73, 0.73, 0.73)),
        },
    }));

    let green: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.12, 0.45, 0.15)),
        },
    }));

    let light: Arc<Box<dyn Material>> = Arc::new(Box::new(DiffuseLight {
        emit: SolidColor {
            color_value: Color(vec3(15.0, 15.0, 15.0)),
        },
    }));

    let aluminum: Arc<Box<dyn Material>> = Arc::new(Box::new(Metal {
        fuzz: 0.0,
        albedo: Color(vec3(0.8, 0.85, 0.88)),
    }));

    let box1 = AABox::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 330.0, 165.0),
        aluminum,
        rng,
    );
    let box1 = RotateY::new(box1, 0.0, 1.0, Deg(15.0));
    let box1 = Box::new(Translate {
        hittable: box1,
        offset: vec3(265.0, 0.0, 295.0),
    });

    let box2 = AABox::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 165.0, 165.0),
        white.clone(),
        rng,
    );
    let box2 = RotateY::new(box2, 0.0, 1.0, Deg(-18.0));
    let box2 = Box::new(Translate {
        hittable: box2,
        offset: vec3(130.0, 0.0, 65.0),
    });

    let grass: Arc<Box<dyn Material>> = Arc::new(Box::new(Dielectric { ir: 1.5 }));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: green,
        }),
        Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: red,
        }),
        Box::new(FlipFace(XZRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            material: light,
        })),
        Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: white.clone(),
        }),
        Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: white.clone(),
        }),
        Box::new(XYRect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            material: white.clone(),
        }),
        box1,
        // box2,
        Box::new(Sphere {
            center: point3(190.0, 90.0, 190.0),
            radius: 90.0,
            material: grass,
        }),
    ];

    BVHNode::new(world, 0.0, 1.0, rng)
}

fn cornel_smoke(rng: &mut impl Rng) -> BVHNode {
    let red: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.65, 0.05, 0.05)),
        },
    }));

    let white: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.73, 0.73, 0.73)),
        },
    }));

    let green: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.12, 0.45, 0.15)),
        },
    }));

    let light: Arc<Box<dyn Material>> = Arc::new(Box::new(DiffuseLight {
        emit: SolidColor {
            color_value: Color(vec3(15.0, 15.0, 15.0)),
        },
    }));

    let box1 = AABox::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 330.0, 165.0),
        white.clone(),
        rng,
    );
    let box1 = RotateY::new(box1, 0.0, 1.0, Deg(15.0));
    let box1 = Box::new(Translate {
        hittable: box1,
        offset: vec3(265.0, 0.0, 295.0),
    });

    let box2 = AABox::new(
        point3(0.0, 0.0, 0.0),
        point3(165.0, 165.0, 165.0),
        white.clone(),
        rng,
    );
    let box2 = RotateY::new(box2, 0.0, 1.0, Deg(-18.0));
    let box2 = Box::new(Translate {
        hittable: box2,
        offset: vec3(130.0, 0.0, 65.0),
    });

    let smoke1 = Box::new(ConstantMedium::new(
        box1,
        0.01,
        Box::new(SolidColor {
            color_value: Color(vec3(0.0, 0.0, 0.0)),
        }),
    ));

    let smoke2 = Box::new(ConstantMedium::new(
        box2,
        0.01,
        Box::new(SolidColor {
            color_value: Color(vec3(1.0, 1.0, 1.0)),
        }),
    ));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: green,
        }),
        Box::new(YZRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: red,
        }),
        Box::new(XZRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            material: light,
        }),
        Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            material: white.clone(),
        }),
        Box::new(XZRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            material: white.clone(),
        }),
        Box::new(XYRect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            material: white.clone(),
        }),
        smoke1,
        smoke2,
    ];

    BVHNode::new(world, 0.0, 1.0, rng)
}

fn final_scene(rng: &mut impl Rng) -> BVHNode {
    let ground: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.48, 0.83, 0.53)),
        },
    }));

    const BOXES_PER_SIDE: usize = 20;

    let mut boxes1: Vec<Box<dyn Hittable>> = Vec::new();

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as Float * w;
            let z0 = -1000.0 + j as Float * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.push(Box::new(AABox::new(
                point3(x0, y0, z0),
                point3(x1, y1, z1),
                ground.clone(),
                rng,
            )));
        }
    }

    let mut objects: Vec<Box<dyn Hittable>> = vec![Box::new(BVHNode::new(boxes1, 0.0, 1.0, rng))];

    let light: Arc<Box<dyn Material>> = Arc::new(Box::new(DiffuseLight {
        emit: SolidColor {
            color_value: Color(vec3(7.0, 7.0, 7.0)),
        },
    }));

    objects.push(Box::new(XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: light,
    }));

    let center1 = point3(400.0, 400.0, 200.0);
    let center2 = center1 + vec3(30.0, 0.0, 0.0);

    let moving_sphere_material: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.7, 0.3, 0.1)),
        },
    }));

    objects.push(Box::new(MovingSphere {
        center0: center1,
        center1: center2,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        material: moving_sphere_material,
    }));

    objects.push(Box::new(Sphere {
        center: point3(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Arc::new(Box::new(Dielectric { ir: 1.5 })),
    }));

    objects.push(Box::new(Sphere {
        center: point3(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Arc::new(Box::new(Metal {
            albedo: Color(vec3(0.8, 0.8, 0.9)),
            fuzz: 1.0,
        })),
    }));

    let boundary = Box::new(Sphere {
        center: point3(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Arc::new(Box::new(Dielectric { ir: 1.5 })),
    });

    objects.push(Box::new(Sphere {
        center: point3(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Arc::new(Box::new(Dielectric { ir: 1.5 })),
    }));
    objects.push(Box::new(ConstantMedium::new(
        boundary,
        0.2,
        Box::new(SolidColor {
            color_value: Color(vec3(0.2, 0.4, 0.9)),
        }),
    )));

    let boundary = Box::new(Sphere {
        center: point3(0.0, 0.0, 0.0),
        radius: 5000.0,
        material: Arc::new(Box::new(Dielectric { ir: 1.5 })),
    });
    objects.push(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        Box::new(SolidColor {
            color_value: Color(vec3(1.0, 1.0, 1.0)),
        }),
    )));

    let emat: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: load_from_memory(include_bytes!("../assets/earthmap.jpg")).unwrap(),
    }));

    objects.push(Box::new(Sphere {
        center: point3(400.0, 200.0, 400.0),
        radius: 100.0,
        material: emat,
    }));

    let pertext: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: NoiseTexture256::new(0.1, rng),
    }));
    objects.push(Box::new(Sphere {
        center: point3(220.0, 280.0, 300.0),
        radius: 80.0,
        material: pertext,
    }));

    let mut boxes2: Vec<Box<dyn Hittable>> = Vec::new();
    let white: Arc<Box<dyn Material>> = Arc::new(Box::new(Lambertian {
        albedo: SolidColor {
            color_value: Color(vec3(0.73, 0.73, 0.73)),
        },
    }));

    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Box::new(Sphere {
            center: point3(
                rng.gen_range(0.0..165.0),
                rng.gen_range(0.0..165.0),
                rng.gen_range(0.0..165.0),
            ),
            radius: 10.0,
            material: white.clone(),
        }))
    }

    let boxes2 = RotateY::new(BVHNode::new(boxes2, 0.0, 1.0, rng), 0.0, 1.0, Deg(15.0));

    let boxes2: Box<dyn Hittable> = Box::new(Translate {
        hittable: boxes2,
        offset: vec3(-100.0, 270.0, 395.0),
    });

    objects.push(boxes2);
    BVHNode::new(objects, 0.0, 1.0, rng)
}

fn main() {
    let mut aspect_ratio: Float = 16.0 / 9.0;
    let mut image_width: usize = 400;
    let mut samples_per_pixel: usize = 100;
    const MAX_DEPTH: usize = 50;

    let mut rng = MyRng::from_entropy();

    let null_mat: Arc<Box<dyn Material>> = Arc::new(Box::new(()));

    let lights: Vec<Box<dyn Hittable>> = vec![
        Box::new(XZRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            material: null_mat.clone(),
        }),
        Box::new(Sphere {
            center: point3(190.0, 90.0, 190.0),
            radius: 90.0,
            material: null_mat,
        }),
    ];

    let (world, background, look_from, look_at, vfov, aperture) = match 5 {
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
        3 => (
            earth(&mut rng),
            Color(vec3(0.70, 0.80, 1.00)),
            point3(13.0, 2.0, 3.0),
            point3(0.0, 0.0, 0.0),
            Deg(20.0),
            0.0,
        ),
        4 => {
            samples_per_pixel = 400;
            (
                simple_light(&mut rng),
                Color(vec3(0.0, 0.0, 0.0)),
                point3(26.0, 3.0, 6.0),
                point3(0.0, 2.0, 0.0),
                Deg(20.0),
                0.0,
            )
        }
        5 => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 1000;
            (
                cornel_box(&mut rng),
                Color(vec3(0.0, 0.0, 0.0)),
                point3(278.0, 278.0, -800.0),
                point3(278.0, 278.0, 0.0),
                Deg(40.0),
                0.0,
            )
        }
        6 => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            (
                cornel_smoke(&mut rng),
                Color(vec3(0.0, 0.0, 0.0)),
                point3(278.0, 278.0, -800.0),
                point3(278.0, 278.0, 0.0),
                Deg(40.0),
                0.0,
            )
        }
        _ => {
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            (
                final_scene(&mut rng),
                Color(vec3(0.0, 0.0, 0.0)),
                point3(478.0, 278.0, -600.0),
                point3(278.0, 278.0, 0.0),
                Deg(40.0),
                0.0,
            )
        }
    };

    let image_height: usize = (image_width as Float / aspect_ratio) as usize;
    let vup = vec3(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        10.0,
        0.0,
        1.0,
    );

    println!("P3\n{} {}\n255", image_width, image_height);

    let sacans_remaining = AtomicUsize::new(image_height);

    let image: Vec<Vec<SampledColor>> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            let row = (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut rng = MyRng::seed_from_u64((j * image_width + i) as u64);
                    let mut pixel_color = Color(vec3(0.0, 0.0, 0.0));

                    for _ in 0..samples_per_pixel {
                        let u = (i as Float + rng.gen::<Float>()) / (image_width - 1) as Float;
                        let v = (j as Float + rng.gen::<Float>()) / (image_height - 1) as Float;

                        let ray = camera.get_ray(u, v, &mut rng);
                        pixel_color = Color(
                            pixel_color.0
                                + ray_color(&ray, background, &world, &lights, MAX_DEPTH, &mut rng)
                                    .0,
                        );
                    }

                    pixel_color.into_sampled(samples_per_pixel)
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
