use std::fmt::Display;
use std::io::{self, Write};
use errors::*;
use super::Visitor;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct PrinterVisitor;

impl<T> Visitor<T> for PrinterVisitor
where
    T: Display,
{
    type Error = Error;

    fn visit(&self, value: T) -> Result<()> {
        writeln!(io::stdout(), "{}", value).chain_err(|| "failed writing to stdout")
    }
}
