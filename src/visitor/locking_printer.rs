use std::fmt::Display;
use std::io::{Stdout, StdoutLock};
use super::{Visitor, WriterVisitor};

#[derive(Debug)]
pub struct LockingPrinterVisitor<'a>(WriterVisitor<StdoutLock<'a>>);

impl<'a> LockingPrinterVisitor<'a> {
    pub fn new(out: &'a Stdout) -> Self {
        LockingPrinterVisitor(WriterVisitor::new(out.lock()))
    }
}

impl<'a, T> Visitor<T> for LockingPrinterVisitor<'a>
where
    T: Display,
{
    type Error = <WriterVisitor<StdoutLock<'a>> as Visitor<T>>::Error;

    fn visit(&mut self, value: T) -> Result<(), Self::Error> {
        self.0.visit(value)
    }
}
