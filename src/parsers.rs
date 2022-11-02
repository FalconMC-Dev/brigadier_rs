pub(crate) mod literal;
pub(crate) mod then;
pub(crate) mod bool;
pub(crate) mod number;

use std::marker::PhantomData;

pub use literal::{LiteralArgument, LiteralExecutor};
pub use then::{LiteralThen, LiteralThenExecutor, CommandThen, ThenWrapper, ThenExecutor};
pub use self::bool::BoolArgument;
pub use number::IntegerArgument;

pub struct DefaultExecutor<A, C, O> {
    pub(crate) argument: A,
    pub(crate) task: C,
    pub(crate) output: PhantomData<O>,
}
