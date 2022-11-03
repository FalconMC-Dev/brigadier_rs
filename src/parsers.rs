//! Collection of builtin parsers for several basic argument types.
//!
//! ## Currently builtin
//! Currently there are argument type implementations for:
//! - literals: [`LiteralArgument`]
//! - boolean: [`BoolArgument`]
//! - i8, u8, i16, u16, i32, u32, i64, u64, f32, f64: [`NumberArgument`]

pub(crate) mod bool;
pub(crate) mod literal;
pub(crate) mod number;
pub(crate) mod then;

use std::marker::PhantomData;

pub use literal::{LiteralArgument, LiteralExecutor};
pub use number::NumberArgument;
pub use then::{CommandThen, LiteralThen, LiteralThenExecutor, ThenExecutor, ThenWrapper};

pub use self::bool::BoolArgument;

/// Default executor for command argument parsers.
///
/// This type implements `Execute` and `Propagate`.
pub struct DefaultExecutor<A, C, O> {
    pub(crate) argument: A,
    pub(crate) task: C,
    pub(crate) output: PhantomData<O>,
}
