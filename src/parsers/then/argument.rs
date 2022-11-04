use std::marker::PhantomData;

use nom::branch::alt;
use nom::character::complete::char;
use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::ThenWrapper;
use crate::{BuildExecute, BuildPropagate, CommandArgument, CommandError, Execute, Propagate, TaskLogic, Then};

/// Default [`Then`] implementation for any argument type.
pub struct CommandThen<A, E, O> {
    pub(crate) argument: A,
    pub(crate) executor: E,
    pub(crate) output: PhantomData<O>,
}

impl<A, E, O> CommandArgument<O> for CommandThen<A, E, O>
where
    A: CommandArgument<O>,
{
    fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, O, CommandError<'a>> { self.argument.parse(input) }
}

impl<A, E, O, C> BuildExecute<C, ThenExecutor<A, E, C, O>> for CommandThen<A, E, O>
where
    C: TaskLogic<O>,
{
    fn build_exec(self, task: C) -> ThenExecutor<A, E, C, O> {
        ThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, E, O, C, T> BuildPropagate<C, T, ThenExecutor<A, E, C, O>> for CommandThen<A, E, O>
where
    C: TaskLogic<(T, O)>,
{
    fn build_propagate(self, task: C) -> ThenExecutor<A, E, C, O> {
        ThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, O, E, U> Execute<U> for CommandThen<A, E, O>
where
    A: CommandArgument<O>,
    E: Propagate<O, U>,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.propagate(input, result)
    }
}

impl<A, O, E, T, U> Propagate<T, U> for CommandThen<A, E, O>
where
    A: CommandArgument<O>,
    E: Propagate<(T, O), U>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.propagate(input, (data, result))
    }
}

impl<A, O, E1, E2> Then<E2> for CommandThen<A, E1, O>
where
    A: CommandArgument<O>,
{
    type Output = CommandThen<A, ThenWrapper<E1, E2>, O>;

    fn then(self, executor: E2) -> Self::Output {
        CommandThen {
            argument: self.argument,
            executor: ThenWrapper {
                first: self.executor,
                second: executor,
            },
            output: PhantomData,
        }
    }
}

/// Default executor for [`CommandThen`].
pub struct ThenExecutor<A, E, C, O> {
    pub(crate) argument: CommandThen<A, E, O>,
    pub(crate) task: C,
}

impl<A, O, E, C, U> Execute<U> for ThenExecutor<A, E, C, O>
where
    A: CommandArgument<O>,
    E: Propagate<O, U>,
    C: TaskLogic<O, Output = U>,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        alt((
            |i| {
                let (input, result) = self.argument.parse(i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.propagate(input, result)
            },
            |i| {
                let (input, result) = self.argument.parse(i)?;
                match self.task.run(result) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}

impl<A, O, E, C, T, U> Propagate<T, U> for ThenExecutor<A, E, C, O>
where
    T: Copy,
    A: CommandArgument<O>,
    E: Propagate<(T, O), U>,
    C: TaskLogic<(T, O), Output = U>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        alt((
            |i| {
                let (input, result) = self.argument.parse(i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.propagate(input, (data, result))
            },
            |i| {
                let (input, result) = self.argument.parse(i)?;
                match self.task.run((data, result)) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}
