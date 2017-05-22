extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let b:bool = rng.gen();
}