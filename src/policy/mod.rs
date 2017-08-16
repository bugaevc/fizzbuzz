pub mod default;

pub use self::default::DefaultPolicy;

pub trait Policy<T> {
    type Value;

    // TODO: replace with Result<Value, DomainError>.
    fn accept(&self, x: T) -> Option<Self::Value>;
}
