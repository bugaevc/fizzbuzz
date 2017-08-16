use std::fmt::{self, Display, Formatter};
use errors::*;
use super::Policy;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Value {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Value::Fizz => write!(f, "Fizz"),
            Value::Buzz => write!(f, "Buzz"),
            Value::FizzBuzz => write!(f, "FizzBuzz"),
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct DefaultPolicy;

macro_rules! impl_policy {
    ($type:ty) => (
        impl Policy<$type> for DefaultPolicy {
            type Value = Value;
            type Error = Error;

            fn accept(&self, x: $type) -> Result<Value> {
                ensure!(x > 0, Error::from_kind(ErrorKind::DomainError));

                Ok(match (x % 3, x % 5) {
                    (0, 0) => Value::FizzBuzz,
                    (0, _) => Value::Fizz,
                    (_, 0) => Value::Buzz,
                    (_, _) => Value::Number(x as u64),
                })
            }
        }
    )
}

impl_policy!(i8);
impl_policy!(u8);
impl_policy!(i16);
impl_policy!(u16);
impl_policy!(i32);
impl_policy!(u32);
impl_policy!(i64);
impl_policy!(u64);
