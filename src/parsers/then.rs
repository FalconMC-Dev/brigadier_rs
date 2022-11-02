mod argument;
mod literal;

pub use argument::*;
pub use literal::*;
use nom::{IResult, branch::alt};

use crate::{Execute, CommandError, Propagate};

pub struct ThenWrapper<E1, E2> {
    pub(crate) first: E1,
    pub(crate) second: E2,
}

impl<E1, E2> Execute for ThenWrapper<E1, E2>
where
    E1: Execute,
    E2: Execute,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, bool, CommandError<'a>> {
        alt((|i| self.first.execute(i), |i| self.second.execute(i)))(input)
    }
}

impl<E1, E2, T> Propagate<T> for ThenWrapper<E1, E2>
where
    T: Copy,
    E1: Propagate<T>,
    E2: Propagate<T>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, bool, CommandError<'a>> {
        alt((|i| self.first.propagate(i, data), |i| self.second.propagate(i, data)))(input)
    }
}
