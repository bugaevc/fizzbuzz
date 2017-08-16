use std::error::Error;

pub mod locking_printer;
pub mod printer;
pub mod writer;

pub use self::locking_printer::LockingPrinterVisitor;
pub use self::printer::PrinterVisitor;
pub use self::writer::WriterVisitor;

pub trait Visitor<T> {
    type Error: Error + Send + 'static;

    fn visit(&mut self, value: T) -> Result<(), Self::Error>;
}
