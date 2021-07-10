use std::{fmt::Display, ops::Deref};

use cgmath::{vec3, Vector3};

type Float = f64;

#[derive(Clone, Copy)]
struct Color(pub Vector3<Float>);

impl Deref for Color {
    type Target = Vector3<Float>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self[0]) as usize,
            (255.999 * self[1]) as usize,
            (255.999 * self[2]) as usize
        )
    }
}

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let color = Color(vec3(
                i as Float / (IMAGE_WIDTH - 1) as Float,
                j as Float / (IMAGE_HEIGHT - 1) as Float,
                0.25,
            ));

            println!("{}", color);
        }
    }

    eprintln!("\nDone");
}
