type Float = f64;

mod color;
mod ray;

use cgmath::vec3;
use color::Color;

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
