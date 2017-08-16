extern crate fizzbuzz;

use std::io;

fn main() {
    let stdout = io::stdout();
    let visitor = fizzbuzz::LockingPrinterVisitor::new(&stdout);
    let mut fizzbuzz = fizzbuzz::default_builder().with_visitor(visitor).build();
    fizzbuzz.work(1..100).unwrap();
}
