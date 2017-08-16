pub mod printer;

pub use self::printer::PrinterVisitor;

pub trait Visitor<T> {
    fn visit(&self, value: T);
}
