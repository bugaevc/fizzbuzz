use std::fmt::Display;
use super::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct PrinterVisitor;

impl<T> Visitor<T> for PrinterVisitor
where
    T: Display,
{
    fn visit(&self, value: T) {
        println!("{}", value);
    }
}
