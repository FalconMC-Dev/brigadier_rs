use std::marker::PhantomData;

use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::{ArgumentMarkerDefaultImpl, BuildExecute, BuildPropagate, CommandArgument, Execute, Propagate, TaskLogic, TaskLogicNoArgs};
use crate::parsers::DefaultExecutor;
use crate::CommandError;

impl<A, O, C, S> CommandArgument<S, O> for DefaultExecutor<A, O, C, S>
where
    A: CommandArgument<S, O>,
{
    fn parse<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, O, CommandError<'a>> { self.argument.parse(source, input) }
}

impl<S, A, O, C> BuildExecute<C, DefaultExecutor<A, C, O, S>> for A
where
    A: ArgumentMarkerDefaultImpl + CommandArgument<S, O>,
    C: TaskLogic<S, O>,
{
    fn build_exec(self, task: C) -> DefaultExecutor<A, C, O, S> {
        DefaultExecutor {
            argument: self,
            task,
            output: PhantomData,
            source: PhantomData,
        }
    }
}

impl<S, A, O, C, T> BuildPropagate<C, T, DefaultExecutor<A, C, O, S>> for A
where
    A: ArgumentMarkerDefaultImpl + CommandArgument<S, O>,
    C: TaskLogic<S, (T, O)>,
{
    fn build_propagate(self, task: C) -> DefaultExecutor<A, C, O, S> {
        DefaultExecutor {
            argument: self,
            task,
            output: PhantomData,
            source: PhantomData,
        }
    }
}

impl<A, O, C, U, S> Execute<S, U> for DefaultExecutor<A, C, O, S>
where
    A: CommandArgument<S, O>,
    C: TaskLogic<S, O, Output = U>,
    S: Copy,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(source, input)?;
        match self.task.run(source, result) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<A, O, C, T, U, S> Propagate<S, T, U> for DefaultExecutor<A, C, O, S>
where
    T: Copy,
    S: Copy,
    A: CommandArgument<S, O>,
    C: TaskLogic<S, (T, O), Output = U>,
{
    fn propagate<'a>(&self, source: S, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(source, input)?;
        match self.task.run(source, (data, result)) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<E, F, S, O, U> TaskLogic<S, O> for F
where
    F: Fn(S, O) -> Result<U, E>,
    E: Into<anyhow::Error>,
{
    type Error = E;
    type Output = U;

    fn run(&self, source: S, args: O) -> Result<U, E> { self(source, args) }
}

impl<E, F, U, S> TaskLogicNoArgs<S> for F
where
    F: Fn(S) -> Result<U, E>,
    E: Into<anyhow::Error>,
{
    type Error = E;
    type Output = U;

    fn run(&self, source: S) -> Result<U, Self::Error> { self(source) }
}
