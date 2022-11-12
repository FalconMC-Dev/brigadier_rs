use std::marker::PhantomData;

use nom::bytes::complete::tag_no_case;
use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::LiteralThen;
use crate::{
    BuildExecute, BuildPropagate, ChildUsage, CommandArgument, CommandError, Execute, IntoMultipleUsage, Propagate, TaskLogic,
    TaskLogicNoArgs, Then,
};

/// Create a new literal parser
///
/// This parser has 1 field; the literal that should be matched against.
pub fn literal<S>(literal: &'static str) -> LiteralArgument<S> {
    LiteralArgument {
        literal,
        source: PhantomData,
    }
}

/// Literal argument parser.
pub struct LiteralArgument<S> {
    literal: &'static str,
    source: PhantomData<S>,
}

impl<S> CommandArgument<S, ()> for LiteralArgument<S> {
    fn parse<'a>(&self, _source: S, input: &'a str) -> IResult<&'a str, (), CommandError<'a>> {
        let (output, _) = tag_no_case(self.literal)(input)?;
        Ok((output, ()))
    }
}

impl<S, E> Then<E> for LiteralArgument<S> {
    type Output = LiteralThen<Self, E, S>;

    fn then(self, executor: E) -> Self::Output {
        LiteralThen {
            argument: self,
            executor,
            source: PhantomData,
        }
    }
}

impl<S, C> BuildExecute<C, LiteralExecutor<LiteralArgument<S>, C, S>> for LiteralArgument<S>
where
    C: TaskLogicNoArgs<S>,
{
    fn build_exec(self, task: C) -> LiteralExecutor<LiteralArgument<S>, C, S> {
        LiteralExecutor {
            argument: self,
            task,
            source: PhantomData,
        }
    }
}

impl<S, C, T> BuildPropagate<C, T, LiteralExecutor<Self, C, S>> for LiteralArgument<S>
where
    C: TaskLogic<S, T>,
{
    fn build_propagate(self, task: C) -> LiteralExecutor<LiteralArgument<S>, C, S> {
        LiteralExecutor {
            argument: self,
            task,
            source: PhantomData,
        }
    }
}

impl<S> IntoMultipleUsage for LiteralArgument<S> {
    type Item = <&'static str as IntoMultipleUsage>::Item;

    fn usage_gen(&self) -> Self::Item { self.usage_child().usage_gen() }
}

impl<S> ChildUsage for LiteralArgument<S> {
    type Child = &'static str;

    fn usage_child(&self) -> Self::Child { self.literal }
}

/// Type returned when calling [`build_exec`](BuildExecute::build_exec) or
/// [`build_propagate`](BuildPropagate::build_propagate) on a
/// [`LiteralArgument`].
pub struct LiteralExecutor<A, C, S> {
    argument: A,
    task: C,
    source: PhantomData<S>,
}

impl<A, C, U, S> Execute<S, U> for LiteralExecutor<A, C, S>
where
    S: Copy,
    A: CommandArgument<S, ()>,
    C: TaskLogicNoArgs<S, Output = U>,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, _) = self.argument.parse(source, input)?;
        match self.task.run(source) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<A, C, T, U, S> Propagate<S, T, U> for LiteralExecutor<A, C, S>
where
    T: Copy,
    S: Copy,
    A: CommandArgument<S, ()>,
    C: TaskLogic<S, T, Output = U>,
{
    fn propagate<'a>(&self, source: S, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, _) = self.argument.parse(source, input)?;
        match self.task.run(source, data) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<A, C, S> CommandArgument<S, ()> for LiteralExecutor<A, C, S>
where
    A: CommandArgument<S, ()>,
{
    fn parse<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, (), CommandError<'a>> { self.argument.parse(source, input) }
}

impl<A, C, S> IntoMultipleUsage for LiteralExecutor<A, C, S>
where
    A: IntoMultipleUsage,
{
    type Item = A::Item;

    fn usage_gen(&self) -> Self::Item { self.argument.usage_gen() }
}

impl<A, C, S> ChildUsage for LiteralExecutor<A, C, S>
where
    A: ChildUsage,
{
    type Child = A::Child;

    fn usage_child(&self) -> Self::Child { self.argument.usage_child() }
}
