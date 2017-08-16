#[macro_use]
extern crate error_chain;

use std::io::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use Value::*;

mod errors;
pub mod builder;
pub mod policy;
pub mod visitor;

use errors::*;
pub use builder::*;
pub use policy::*;
pub use visitor::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct FizzBuzz<P, V, T>
where
    P: Policy<T>,
    V: Visitor<P::Value>,
{
    policy: P,
    visitor: V,
    _p: PhantomData<fn(T)>,
}

impl<P, V, T> FizzBuzz<P, V, T>
where
    P: Policy<T>,
    V: Visitor<P::Value>,
{
    pub fn work<I>(&self, iter: I) -> Result<()>
    where
        I: Iterator<Item = T>,
    {
        for x in iter {
            let value = self.policy.accept(x).chain_err(|| "failed accepting a value")?;
            self.visitor.visit(value).chain_err(|| "failed visiting the value")?;
        }

        Ok(())
    }
}

/// A FizzBuzz value.
///
/// Unsigned integers can be converted to FizzBuzz values.
///
/// # Examples
/// ```
/// use fizzbuzz::Value::*;
///
/// let mut iter = (1..).map(fizzbuzz::Value::from);
///
/// assert_eq!(iter.next(), Some(Number(1)));
/// assert_eq!(iter.next(), Some(Number(2)));
/// assert_eq!(iter.next(), Some(Fizz));
/// assert_eq!(iter.next(), Some(Number(4)));
/// assert_eq!(iter.next(), Some(Buzz));
/// assert_eq!(iter.next(), Some(Fizz));
/// assert_eq!(iter.next(), Some(Number(7)));
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Value {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl From<u32> for Value {
    fn from(n: u32) -> Value {
        match (n % 3, n % 5) {
            (0, 0) => FizzBuzz,
            (0, _) => Fizz,
            (_, 0) => Buzz,
            (_, _) => Number(n),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Fizz => write!(f, "Fizz"),
            Buzz => write!(f, "Buzz"),
            FizzBuzz => write!(f, "FizzBuzz"),
            Number(n) => write!(f, "{}", n),
        }
    }
}

/// Prints FizzBuzz values over the specified sequence to the standard output.
///
/// # Examples
/// ```
/// fizzbuzz::fizzbuzz(1..100);
///
/// // 1
/// // 2
/// // Fizz
/// // 4
/// // Buzz
/// // Fizz
/// // 7
/// // <...>
/// ```
///
/// # Panics
///
/// Panics on `stdout` output error.
pub fn fizzbuzz<I>(it: I)
where
    I: Iterator<Item = u32>,
{
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for n in it {
        writeln!(stdout, "{}", Value::from(n)).expect("Failed to output");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected = [
            FizzBuzz,
            Number(1),
            Number(2),
            Fizz,
            Number(4),
            Buzz,
            Fizz,
            Number(7),
            Number(8),
            Fizz,
            Buzz,
            Number(11),
            Fizz,
            Number(13),
            Number(14),
            FizzBuzz,
            Number(16),
            Number(17),
            Fizz,
            Number(19),
        ];

        for (i, &exp) in expected.iter().enumerate() {
            assert_eq!(Value::from(i as u32), exp);
        }
    }
}
