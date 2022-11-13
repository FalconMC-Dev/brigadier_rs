mod argument;
mod literal;

pub use argument::*;
pub use literal::*;
use nom::branch::alt;
use nom::IResult;

use crate::{Chain, CommandError, Execute, IntoMultipleUsage, MultipleUsage, Propagate};

/// Parser wrapper that correctly tries both child parsers.
pub struct ThenWrapper<E1, E2> {
    pub(crate) first: E1,
    pub(crate) second: E2,
}

impl<E1, E2, U, S> Execute<S, U> for ThenWrapper<E1, E2>
where
    E1: Execute<S, U>,
    E2: Execute<S, U>,
    S: Copy,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        alt((|i| self.first.execute(source, i), |i| self.second.execute(source, i)))(input)
    }
}

impl<E1, E2, T, U, S> Propagate<S, T, U> for ThenWrapper<E1, E2>
where
    T: Copy,
    S: Copy,
    E1: Propagate<S, T, U>,
    E2: Propagate<S, T, U>,
{
    fn propagate<'a>(&self, source: S, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        alt((|i| self.first.propagate(source, i, data), |i| self.second.propagate(source, i, data)))(input)
    }
}

impl<E1, E2> IntoMultipleUsage for ThenWrapper<E1, E2>
where
    E1: IntoMultipleUsage,
    E2: IntoMultipleUsage,
{
    type Item = Chain<E1::Item, E2::Item>;

    fn usage_gen(&self) -> Self::Item { self.first.usage_gen().chain(self.second.usage_gen()) }
}
