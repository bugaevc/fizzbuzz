extern crate fizzbuzz;

fn main() {
    let fizzbuzz = fizzbuzz::default_builder().build();
    fizzbuzz.work(1..100);
}
