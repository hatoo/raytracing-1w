use rand::prelude::*;

fn main() {
    let mut rng = StdRng::from_entropy();
    let mut inside_circle = 0;
    for n in 0.. {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);

        if x * x + y * y < 1.0 {
            inside_circle += 1;
        }

        if n % 100000 == 0 {
            println!("Estimate of Pi = {}", 4.0 * inside_circle as f64 / n as f64);
        }
    }
}
