extern crate fizzbuzz;

use std::fmt;

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
    values: &'a mut Vec<T>,
}

impl<'a, T> VectorizeVisitor<'a, T>
where
    T: 'a,
{
    fn new(values: &'a mut Vec<T>) -> Self {
        Self { values }
    }
}

impl<'a, T> fizzbuzz::Visitor<T> for VectorizeVisitor<'a, T>
where
    T: 'a,
{
    type Error = ImpossibleError;

    fn visit(&mut self, value: T) -> Result<(), Self::Error> {
        Ok(self.values.push(value))
    }
}

fn main() {
    let mut values = Vec::new();

    {
        let mut fizzbuzz = fizzbuzz::default_builder()
            .with_visitor(VectorizeVisitor::new(&mut values))
            .build();
        fizzbuzz.work(1..100).unwrap();
    }

    for v in values {
        println!("{}", v);
    }
}
