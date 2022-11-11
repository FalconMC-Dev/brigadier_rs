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

impl<E1, E2, U> Execute<U> for ThenWrapper<E1, E2>
where
    E1: Execute<U>,
    E2: Execute<U>,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        alt((|i| self.first.execute(i), |i| self.second.execute(i)))(input)
    }
}

impl<E1, E2, T, U> Propagate<T, U> for ThenWrapper<E1, E2>
where
    T: Copy,
    E1: Propagate<T, U>,
    E2: Propagate<T, U>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        alt((|i| self.first.propagate(i, data), |i| self.second.propagate(i, data)))(input)
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
