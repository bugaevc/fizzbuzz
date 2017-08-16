extern crate fizzbuzz;

use std::fmt;
use std::cell::RefCell;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum ImpossibleError {}

impl fmt::Display for ImpossibleError {
    fn fmt(&self, _: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        unreachable!()
    }
}

impl std::error::Error for ImpossibleError {
    fn description(&self) -> &str {
        unreachable!()
    }

    fn cause(&self) -> Option<&std::error::Error> {
        unreachable!()
    }
}

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
    type Error = ImpossibleError;

    fn visit(&self, value: T) -> Result<(), Self::Error> {
        Ok(self.values.borrow_mut().push(value))
    }
}

fn main() {
    let values = RefCell::new(Vec::new());

    {
        let fizzbuzz = fizzbuzz::default_builder()
            .with_visitor(VectorizeVisitor::new(&values))
            .build();
        fizzbuzz.work(1..100).unwrap();
    }

    for v in values.into_inner() {
        println!("{}", v);
    }
}
