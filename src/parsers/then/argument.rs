use std::marker::PhantomData;

use nom::branch::alt;
use nom::character::complete::char;
use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::ThenWrapper;
use crate::{
    prefix, BuildExecute, BuildPropagate, Chain, ChildUsage, CmdErrorKind, CommandArgument, CommandError, Execute, IntoMultipleUsage,
    MultipleUsage, Prefix, Propagate, TaskLogic, Then,
};

/// Default [`Then`] implementation for any argument type.
pub struct CommandThen<A, E, O, S> {
    pub(crate) argument: A,
    pub(crate) executor: E,
    pub(crate) output: PhantomData<O>,
    pub(crate) source: PhantomData<S>,
}

impl<A, E, O, S> CommandArgument<S, O> for CommandThen<A, E, O, S>
where
    A: CommandArgument<S, O>,
{
    fn parse<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, O, CommandError<'a>> { self.argument.parse(source, input) }
}

impl<S, A, E, O, C> BuildExecute<C, ThenExecutor<A, E, C, O, S>> for CommandThen<A, E, O, S>
where
    C: TaskLogic<S, O>,
{
    fn build_exec(self, task: C) -> ThenExecutor<A, E, C, O, S> {
        ThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, E, O, C, T, S> BuildPropagate<C, T, ThenExecutor<A, E, C, O, S>> for CommandThen<A, E, O, S>
where
    C: TaskLogic<S, (T, O)>,
{
    fn build_propagate(self, task: C) -> ThenExecutor<A, E, C, O, S> {
        ThenExecutor {
            argument: self,
            task,
        }
    }
}

impl<A, E, O, S> IntoMultipleUsage for CommandThen<A, E, O, S>
where
    A: IntoMultipleUsage + ChildUsage,
    E: IntoMultipleUsage,
{
    type Item = Prefix<(A::Child, &'static str), E::Item>;

    fn usage_gen(&self) -> Self::Item { prefix((self.argument.usage_child(), " "), self.executor.usage_gen()) }
}

impl<A, E, O, S> ChildUsage for CommandThen<A, E, O, S>
where
    A: ChildUsage,
{
    type Child = A::Child;

    fn usage_child(&self) -> Self::Child { self.argument.usage_child() }
}

impl<A, O, E, U, S> Execute<S, U> for CommandThen<A, E, O, S>
where
    A: CommandArgument<S, O>,
    E: Propagate<S, O, U>,
    S: Copy,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(source, input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.propagate(source, input, result)
    }
}

impl<A, O, E, T, U, S> Propagate<S, T, U> for CommandThen<A, E, O, S>
where
    A: CommandArgument<S, O>,
    E: Propagate<S, (T, O), U>,
    S: Copy,
{
    fn propagate<'a>(&self, source: S, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(source, input)?;
        let (input, _) = char(' ')(input)?;
        self.executor.propagate(source, input, (data, result))
    }
}

impl<A, O, E1, E2, S> Then<E2> for CommandThen<A, E1, O, S>
where
    A: CommandArgument<S, O>,
{
    type Output = CommandThen<A, ThenWrapper<E1, E2>, O, S>;

    fn then(self, executor: E2) -> Self::Output {
        CommandThen {
            argument: self.argument,
            executor: ThenWrapper {
                first: self.executor,
                second: executor,
            },
            output: PhantomData,
            source: PhantomData,
        }
    }
}

/// Default executor for [`CommandThen`].
pub struct ThenExecutor<A, E, C, O, S> {
    pub(crate) argument: CommandThen<A, E, O, S>,
    pub(crate) task: C,
}

impl<A, E, C, O, S> CommandArgument<S, O> for ThenExecutor<A, E, C, O, S>
where
    A: CommandArgument<S, O>,
{
    fn parse<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, O, CommandError<'a>> { self.argument.parse(source, input) }
}

impl<A, O, E, C, U, S> Execute<S, U> for ThenExecutor<A, E, C, O, S>
where
    A: CommandArgument<S, O>,
    E: Propagate<S, O, U>,
    C: TaskLogic<S, O, Output = U>,
    S: Copy,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        alt((
            |i| {
                let (input, result) = self.argument.parse(source, i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.propagate(source, input, result)
            },
            |i| {
                let (input, result) = self.argument.parse(source, i)?;
                if !input.is_empty() {
                    return Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::IsNot, CmdErrorKind::NonEmpty)));
                }
                match self.task.run(source, result) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}

impl<A, O, E, C, T, U, S> Propagate<S, T, U> for ThenExecutor<A, E, C, O, S>
where
    T: Copy,
    S: Copy,
    A: CommandArgument<S, O>,
    E: Propagate<S, (T, O), U>,
    C: TaskLogic<S, (T, O), Output = U>,
{
    fn propagate<'a>(&self, source: S, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        alt((
            |i| {
                let (input, result) = self.argument.parse(source, i)?;
                let (input, _) = char(' ')(input)?;
                self.argument.executor.propagate(source, input, (data, result))
            },
            |i| {
                let (input, result) = self.argument.parse(source, i)?;
                if !input.is_empty() {
                    return Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::IsNot, CmdErrorKind::NonEmpty)));
                }
                match self.task.run(source, (data, result)) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
        ))(input)
    }
}

impl<A, E, C, O, S> IntoMultipleUsage for ThenExecutor<A, E, C, O, S>
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

impl<A, E, C, O, S> ChildUsage for ThenExecutor<A, E, C, O, S>
where
    CommandThen<A, E, O, S>: ChildUsage,
{
    type Child = <CommandThen<A, E, O, S> as ChildUsage>::Child;

    fn usage_child(&self) -> Self::Child { self.argument.usage_child() }
}
