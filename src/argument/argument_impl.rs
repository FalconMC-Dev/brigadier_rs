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

impl<A, O, C> Execute for DefaultExecutor<A, C, O>
where
    A: CommandArgument<O>,
    C: TaskLogic<O>,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, bool, CommandError<'a>> {
        let (input, result) = self.argument.parse(input)?;
        match self.task.run(result) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<A, O, C, T> Propagate<T> for DefaultExecutor<A, C, O>
where
    T: Copy,
    A: CommandArgument<O>,
    C: TaskLogic<(T, O)>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, bool, CommandError<'a>> {
        let (input, result) = self.argument.parse(input)?;
        match self.task.run((data, result)) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v)),
        }
    }
}

impl<E, F, O> TaskLogic<O> for F
where
    F: Fn(O) -> Result<bool, E>,
    E: Into<anyhow::Error>,
{
    type Error = E;

    fn run(&self, args: O) -> Result<bool, E> { self(args) }
}

impl<E, F> TaskLogicNoArgs for F
where
    F: Fn() -> Result<bool, E>,
    E: Into<anyhow::Error>,
{
    type Error = E;

    fn run(&self) -> Result<bool, Self::Error> { self() }
}
