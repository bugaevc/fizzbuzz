use std::error::Error;

pub mod printer;

pub use self::printer::PrinterVisitor;

pub trait Visitor<T> {
    type Error: Error + Send + 'static;

    fn visit(&self, value: T) -> Result<(), Self::Error>;
}
