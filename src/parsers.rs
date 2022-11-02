pub(crate) mod bool;
pub(crate) mod literal;
pub(crate) mod number;
pub(crate) mod then;

use std::marker::PhantomData;

pub use literal::{LiteralArgument, LiteralExecutor};
pub use number::IntegerArgument;
pub use then::{CommandThen, LiteralThen, LiteralThenExecutor, ThenExecutor, ThenWrapper};

pub use self::bool::BoolArgument;

pub struct DefaultExecutor<A, C, O> {
    pub(crate) argument: A,
    pub(crate) task: C,
    pub(crate) output: PhantomData<O>,
}
