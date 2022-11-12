use std::marker::PhantomData;

use nom::branch::alt;
use nom::character::complete::char;
use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::ThenWrapper;
use crate::{
    prefix, BuildExecute, BuildPropagate, Chain, ChildUsage, CommandArgument, CommandError, Execute, IntoMultipleUsage, MultipleUsage,
    Prefix, Propagate, TaskLogic, TaskLogicNoArgs, Then,
};

/// Default [`Then`] implementation for argument parsers that return `()`.
pub struct LiteralThen<A, E, S> {
    pub(crate) argument: A,
    pub(crate) executor: E,
    pub(crate) source: PhantomData<S>,
}

impl<A, E, S> CommandArgument<S, ()> for LiteralThen<A, E, S>
where
    A: CommandArgument<S, ()>,
{
    fn parse<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, (), CommandError<'a>> { self.argument.parse(source, input) }
}

impl<A, E, C, S> BuildExecute<C, LiteralThenExecutor<A, E, C, S>> for LiteralThen<A, E, S>
where
    C: TaskLogicNoArgs<S>,
{
    fn build_exec(self, task: C) -> LiteralThenExecutor<A, E, C, S> {
        LiteralThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, E, C, T, S> BuildPropagate<C, T, LiteralThenExecutor<A, E, C, S>> for LiteralThen<A, E, S>
where
    C: TaskLogic<S, T>,
{
    fn build_propagate(self, task: C) -> LiteralThenExecutor<A, E, C, S> {
        LiteralThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, E, S> IntoMultipleUsage for LiteralThen<A, E, S>
where
    A: IntoMultipleUsage + ChildUsage,
    E: IntoMultipleUsage,
{
    type Item = Prefix<(A::Child, &'static str), E::Item>;

    fn usage_gen(&self) -> Self::Item { prefix((self.argument.usage_child(), " "), self.executor.usage_gen()) }
}

impl<A, E, S> ChildUsage for LiteralThen<A, E, S>
where
    A: ChildUsage,
{
    type Child = A::Child;

    fn usage_child(&self) -> Self::Child { self.argument.usage_child() }
}

impl<A, E, U, S> Execute<S, U> for LiteralThen<A, E, S>
where
    A: CommandArgument<S, ()>,
    E: Execute<S, U>,
    S: Copy,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, _) = self.argument.parse(source, input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.execute(source, input)
    }
}

impl<A, E, T, U, S> Propagate<S, T, U> for LiteralThen<A, E, S>
where
    A: CommandArgument<S, ()>,
    E: Propagate<S, T, U>,
    S: Copy,
{
    fn propagate<'a>(&self, source: S, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, _) = self.argument.parse(source, input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.propagate(source, input, data)
    }
}

impl<A, E1, E2, S> Then<E2> for LiteralThen<A, E1, S>
where
    A: CommandArgument<S, ()>,
{
    type Output = LiteralThen<A, ThenWrapper<E1, E2>, S>;

    fn then(self, executor: E2) -> Self::Output {
        LiteralThen {
            argument: self.argument,
            executor: ThenWrapper {
                first: self.executor,
                second: executor,
            },
            source: PhantomData,
        }
    }
}

/// Default executor for [`LiteralThen`].
pub struct LiteralThenExecutor<A, E, C, S> {
    pub(crate) argument: LiteralThen<A, E, S>,
    pub(crate) task: C,
}

impl<A, E, C, U, S> Execute<S, U> for LiteralThenExecutor<A, E, C, S>
where
    A: CommandArgument<S, ()>,
    E: Execute<S, U>,
    C: TaskLogicNoArgs<S, Output = U>,
    S: Copy,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        alt((
            |i| {
                let (input, _) = self.argument.parse(source, i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.execute(source, input)
            },
            |i| {
                let (input, _) = self.argument.parse(source, i)?;
                match self.task.run(source) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}

impl<A, E, C, T, U, S> Propagate<S, T, U> for LiteralThenExecutor<A, E, C, S>
where
    T: Copy,
    S: Copy,
    A: CommandArgument<S, ()>,
    E: Propagate<S, T, U>,
    C: TaskLogic<S, T, Output = U>,
{
    fn propagate<'a>(&self, source: S, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        alt((
            |i| {
                let (input, _) = self.argument.parse(source, i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.propagate(source, input, data)
            },
            |i| {
                let (input, _) = self.argument.parse(source, i)?;
                match self.task.run(source, data) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}

impl<A, E, C, S> CommandArgument<S, ()> for LiteralThenExecutor<A, E, C, S>
where
    A: CommandArgument<S, ()>,
{
    fn parse<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, (), CommandError<'a>> { self.argument.parse(source, input) }
}

impl<A, E, C, S> IntoMultipleUsage for LiteralThenExecutor<A, E, C, S>
where
    A: IntoMultipleUsage + ChildUsage,
    E: IntoMultipleUsage,
{
    type Item = Chain<A::Item, Prefix<(A::Child, &'static str), E::Item>>;

    fn usage_gen(&self) -> Self::Item {
        self.argument
            .argument
            .usage_gen()
            .chain(prefix((self.argument.argument.usage_child(), " "), self.argument.executor.usage_gen()))
    }
}

impl<A, E, C, S> ChildUsage for LiteralThenExecutor<A, E, C, S>
where
    LiteralThen<A, E, S>: ChildUsage,
{
    type Child = <LiteralThen<A, E, S> as ChildUsage>::Child;

    fn usage_child(&self) -> Self::Child { self.argument.usage_child() }
}
