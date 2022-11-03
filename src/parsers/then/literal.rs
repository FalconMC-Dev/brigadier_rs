use nom::branch::alt;
use nom::character::complete::char;
use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::ThenWrapper;
use crate::{BuildExecute, BuildPropagate, CommandArgument, CommandError, Execute, Propagate, TaskLogic, TaskLogicNoArgs, Then};

/// Default [`Then`] implementation for argument parsers that return `()`.
pub struct LiteralThen<A, E> {
    pub(crate) argument: A,
    pub(crate) executor: E,
}

impl<A, E> CommandArgument<()> for LiteralThen<A, E>
where
    A: CommandArgument<()>,
{
    fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, (), CommandError<'a>> { self.argument.parse(input) }
}

impl<A, E, C> BuildExecute<C, LiteralThenExecutor<A, E, C>> for LiteralThen<A, E>
where
    C: TaskLogicNoArgs,
{
    fn build_exec(self, task: C) -> LiteralThenExecutor<A, E, C> {
        LiteralThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, E, C, T> BuildPropagate<C, T, LiteralThenExecutor<A, E, C>> for LiteralThen<A, E>
where
    C: TaskLogic<T>,
{
    fn build_propagate(self, task: C) -> LiteralThenExecutor<A, E, C> {
        LiteralThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, E> Execute for LiteralThen<A, E>
where
    A: CommandArgument<()>,
    E: Execute,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, bool, CommandError<'a>> {
        let (input, _) = self.argument.parse(input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.execute(input)
    }
}

impl<A, E, T> Propagate<T> for LiteralThen<A, E>
where
    A: CommandArgument<()>,
    E: Propagate<T>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, bool, CommandError<'a>> {
        let (input, _) = self.argument.parse(input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.propagate(input, data)
    }
}

impl<A, E1, E2> Then<E2> for LiteralThen<A, E1>
where
    A: CommandArgument<()>,
{
    type Output = LiteralThen<A, ThenWrapper<E1, E2>>;

    fn then(self, executor: E2) -> Self::Output {
        LiteralThen {
            argument: self.argument,
            executor: ThenWrapper {
                first: self.executor,
                second: executor,
            },
        }
    }
}

/// Default executor for [`LiteralThen`].
pub struct LiteralThenExecutor<A, E, C> {
    pub(crate) argument: LiteralThen<A, E>,
    pub(crate) task: C,
}

impl<A, E, C> Execute for LiteralThenExecutor<A, E, C>
where
    A: CommandArgument<()>,
    E: Execute,
    C: TaskLogicNoArgs,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, bool, CommandError<'a>> {
        alt((
            |i| {
                let (input, _) = self.argument.parse(i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.execute(input)
            },
            |i| {
                let (input, _) = self.argument.parse(i)?;
                match self.task.run() {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}

impl<A, E, C, T> Propagate<T> for LiteralThenExecutor<A, E, C>
where
    T: Copy,
    A: CommandArgument<()>,
    E: Propagate<T>,
    C: TaskLogic<T>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, bool, CommandError<'a>> {
        alt((
            |i| {
                let (input, _) = self.argument.parse(i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.propagate(input, data)
            },
            |i| {
                let (input, _) = self.argument.parse(i)?;
                match self.task.run(data) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}
