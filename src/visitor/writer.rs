use std::fmt::Display;
use std::io::Write;
use errors::*;
use super::Visitor;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct WriterVisitor<T>
where
    T: Write,
{
    out: T,
}

impl<T> WriterVisitor<T>
where
    T: Write,
{
    pub fn new(out: T) -> Self {
        Self { out }
    }
}

impl<W, T> Visitor<T> for WriterVisitor<W>
where
    W: Write,
    T: Display,
{
    type Error = Error;

    fn visit(&mut self, value: T) -> Result<()> {
        writeln!(self.out, "{}", value).chain_err(|| "failed writing")
    }
}
