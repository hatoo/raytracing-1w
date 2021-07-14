use rand::prelude::*;

fn main() {
    let mut rng = StdRng::from_entropy();
    let mut inside_circle = 0;
    let mut inside_circle_stratified = 0;

    const SQRT_N: usize = 10000;

    for i in 0..SQRT_N {
        for j in 0..SQRT_N {
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }

            let x = 2.0 * ((i as f64 + rng.gen::<f64>()) / SQRT_N as f64) - 1.0;
            let y = 2.0 * ((j as f64 + rng.gen::<f64>()) / SQRT_N as f64) - 1.0;

            if x * x + y * y < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }

    println!(
        "Regular\tEstimate of Pi = {}",
        4.0 * inside_circle as f64 / (SQRT_N * SQRT_N) as f64
    );
    println!(
        "Stratified\tEstimate of Pi = {}",
        4.0 * inside_circle_stratified as f64 / (SQRT_N * SQRT_N) as f64
    );
}
