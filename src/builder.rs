use std::marker::PhantomData;
use super::*;
use policy::*;
use visitor::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct FizzBuzzBuilder<P, V, T>
where
    P: Policy<T>,
    V: Visitor<P::Value>,
{
    policy: P,
    visitor: V,
    _p: PhantomData<fn(T)>,
}

impl<P, V, T> FizzBuzzBuilder<P, V, T>
where
    P: Policy<T>,
    V: Visitor<P::Value>,
{
    pub fn with_policy<Q>(self, policy: Q) -> FizzBuzzBuilder<Q, V, T>
    where
        Q: Policy<T, Value = P::Value>,
    {
        FizzBuzzBuilder {
            policy,
            visitor: self.visitor,
            _p: PhantomData,
        }
    }

    pub fn with_visitor<W>(self, visitor: W) -> FizzBuzzBuilder<P, W, T>
    where
        W: Visitor<P::Value>,
    {
        FizzBuzzBuilder {
            policy: self.policy,
            visitor,
            _p: PhantomData,
        }
    }

    pub fn with_policy_and_visitor(policy: P, visitor: V) -> FizzBuzzBuilder<P, V, T> {
        FizzBuzzBuilder {
            policy,
            visitor,
            _p: PhantomData,
        }
    }

    pub fn build(self) -> FizzBuzz<P, V, T> {
        FizzBuzz {
            policy: self.policy,
            visitor: self.visitor,
            _p: PhantomData,
        }
    }
}

pub fn default_builder() -> FizzBuzzBuilder<DefaultPolicy, PrinterVisitor, u64> {
    FizzBuzzBuilder {
        policy: DefaultPolicy,
        visitor: PrinterVisitor,
        _p: PhantomData,
    }
}
