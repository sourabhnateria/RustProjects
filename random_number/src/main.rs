use rand::{rng, Rng};

fn main() {
    let mut rng = rng(); // Use `rng()` instead of `thread_rng()`
    let n: u32 = rng.random(); // Use `random()` instead of `gen()`
    println!("Random number: {}", n);
}
