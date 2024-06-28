use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..10);
    println!("Random number: {}", n);
}
