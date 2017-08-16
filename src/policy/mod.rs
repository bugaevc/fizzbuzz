use std::error::Error;

pub mod default;

pub use self::default::DefaultPolicy;

pub trait Policy<T> {
    type Value;
    type Error: Error + Send + 'static;

    fn accept(&self, x: T) -> Result<Self::Value, Self::Error>;
}
