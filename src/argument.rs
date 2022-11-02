use nom::IResult;

use crate::CommandError;

mod argument_impl;

pub trait CommandArgument<A> {
    fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, A, CommandError<'a>>;
}

pub trait Then<E> {
    type Output;

    fn then(self, executor: E) -> Self::Output;
}

pub trait BuildExecute<C, O> {
    fn build_exec(self, task: C) -> O;
}

pub trait BuildPropagate<C, T, O> {
    fn build_propagate(self, task: C) -> O;
}

pub(crate) trait ArgumentMarkerDefaultImpl {}

pub trait TaskLogicNoArgs {
    type Error: Into<anyhow::Error>;

    fn run(&self) -> Result<bool, Self::Error>;
}

pub trait TaskLogic<O> {
    type Error: Into<anyhow::Error>;

    fn run(&self, args: O) -> Result<bool, Self::Error>;
}

pub trait Execute {
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, bool, CommandError<'a>>;
}

pub trait Propagate<T> {
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, bool, CommandError<'a>>;
}
