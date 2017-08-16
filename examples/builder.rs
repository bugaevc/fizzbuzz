extern crate fizzbuzz;

fn main() {
    let mut fizzbuzz = fizzbuzz::default_builder().build();
    fizzbuzz.work(1..100).unwrap();
}
