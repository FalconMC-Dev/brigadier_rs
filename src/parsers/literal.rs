use nom::bytes::complete::tag_no_case;
use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::LiteralThen;
use crate::{BuildExecute, BuildPropagate, CommandArgument, CommandError, Execute, Propagate, TaskLogic, TaskLogicNoArgs, Then};

/// Create a new literal parser
///
/// This parser has 1 field; the literal that should be matched against.
pub fn literal(literal: &'static str) -> LiteralArgument { LiteralArgument { literal } }

/// Literal argument parser.
pub struct LiteralArgument {
    literal: &'static str,
}

impl CommandArgument<()> for LiteralArgument {
    fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, (), CommandError<'a>> {
        let (output, _) = tag_no_case(self.literal)(input)?;
        Ok((output, ()))
    }
}

impl<E> Then<E> for LiteralArgument {
    type Output = LiteralThen<Self, E>;

    fn then(self, executor: E) -> Self::Output {
        LiteralThen {
            argument: self,
            executor,
        }
    }
}

impl<C> BuildExecute<C, LiteralExecutor<LiteralArgument, C>> for LiteralArgument
where
    C: TaskLogicNoArgs,
{
    fn build_exec(self, task: C) -> LiteralExecutor<LiteralArgument, C> {
        LiteralExecutor {
            argument: self,
            task,
        }
    }
}

impl<C, T> BuildPropagate<C, T, LiteralExecutor<Self, C>> for LiteralArgument
where
    C: TaskLogic<T>,
{
    fn build_propagate(self, task: C) -> LiteralExecutor<LiteralArgument, C> {
        LiteralExecutor {
            argument: self,
            task,
        }
    }
}

/// Type returned when calling [`build_exec`](BuildExecute::build_exec) or
/// [`build_propagate`](BuildPropagate::build_propagate) on a
/// [`LiteralArgument`].
pub struct LiteralExecutor<A, C> {
    argument: A,
    task: C,
}

impl<A, C, U> Execute<U> for LiteralExecutor<A, C>
where
    A: CommandArgument<()>,
    C: TaskLogicNoArgs<Output = U>,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, _) = self.argument.parse(input)?;
        match self.task.run() {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<A, C, T, U> Propagate<T, U> for LiteralExecutor<A, C>
where
    T: Copy,
    A: CommandArgument<()>,
    C: TaskLogic<T, Output = U>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, _) = self.argument.parse(input)?;
        match self.task.run(data) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}
