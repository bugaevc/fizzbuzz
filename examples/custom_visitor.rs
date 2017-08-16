extern crate fizzbuzz;

use std::cell::RefCell;

struct VectorizeVisitor<'a, T: 'a> {
    values: &'a RefCell<Vec<T>>,
}

impl<'a, T> VectorizeVisitor<'a, T>
where
    T: 'a,
{
    fn new(values: &'a RefCell<Vec<T>>) -> Self {
        Self { values }
    }
}

impl<'a, T> fizzbuzz::Visitor<T> for VectorizeVisitor<'a, T>
where
    T: 'a,
{
    fn visit(&self, value: T) {
        self.values.borrow_mut().push(value);
    }
}

fn main() {
    let values = RefCell::new(Vec::new());

    {
        let fizzbuzz = fizzbuzz::default_builder()
            .with_visitor(VectorizeVisitor::new(&values))
            .build();
        fizzbuzz.work(1..100);
    }

    for v in values.into_inner() {
        println!("{}", v);
    }
}
