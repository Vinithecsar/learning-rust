use rand::Rng;

pub fn print_random() {
    let r = rand::rng().random_range(1..=100);
    println!("Hello, world! {r}");
}
