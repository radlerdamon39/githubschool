// This is an example of how you could use the rand crate to generate a random number between 1 and 6.
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, 7);
    println!("You rolled a {}", roll);
}
