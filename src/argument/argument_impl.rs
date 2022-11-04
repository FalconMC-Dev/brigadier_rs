use std::marker::PhantomData;

use nom::error::{ErrorKind, FromExternalError};
use nom::IResult;

use super::{ArgumentMarkerDefaultImpl, BuildExecute, BuildPropagate, CommandArgument, Execute, Propagate, TaskLogic, TaskLogicNoArgs};
use crate::parsers::DefaultExecutor;
use crate::CommandError;

impl<A, O, C> BuildExecute<C, DefaultExecutor<A, C, O>> for A
where
    A: ArgumentMarkerDefaultImpl + CommandArgument<O>,
    C: TaskLogic<O>,
{
    fn build_exec(self, task: C) -> DefaultExecutor<A, C, O> {
        DefaultExecutor {
            argument: self,
            task,
            output: PhantomData,
        }
    }
}

impl<A, O, C, T> BuildPropagate<C, T, DefaultExecutor<A, C, O>> for A
where
    A: ArgumentMarkerDefaultImpl + CommandArgument<O>,
    C: TaskLogic<(T, O)>,
{
    fn build_propagate(self, task: C) -> DefaultExecutor<A, C, O> {
        DefaultExecutor {
            argument: self,
            task,
            output: PhantomData,
        }
    }
}

impl<A, O, C, U> Execute<U> for DefaultExecutor<A, C, O>
where
    A: CommandArgument<O>,
    C: TaskLogic<O, Output = U>,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(input)?;
        match self.task.run(result) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<A, O, C, T, U> Propagate<T, U> for DefaultExecutor<A, C, O>
where
    T: Copy,
    A: CommandArgument<O>,
    C: TaskLogic<(T, O), Output = U>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>> {
        let (input, result) = self.argument.parse(input)?;
        match self.task.run((data, result)) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<E, F, O, U> TaskLogic<O> for F
where
    F: Fn(O) -> Result<U, E>,
    E: Into<anyhow::Error>,
{
    type Error = E;
    type Output = U;

    fn run(&self, args: O) -> Result<U, E> { self(args) }
}

impl<E, F, U> TaskLogicNoArgs for F
where
    F: Fn() -> Result<U, E>,
    E: Into<anyhow::Error>,
{
    type Error = E;
    type Output = U;

    fn run(&self) -> Result<U, Self::Error> { self() }
}
