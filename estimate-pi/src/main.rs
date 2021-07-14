use rand::prelude::*;

fn main() {
    const N: usize = 1000;
    let mut rng = StdRng::from_entropy();
    let inside_circle = (0..N)
        .filter(|_| {
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);

            x * x + y * y < 1.0
        })
        .count();

    println!("Estimate of Pi = {}", 4.0 * inside_circle as f64 / N as f64);
}
