use nom::{IResult, bytes::complete::tag_no_case, error::{ErrorKind, FromExternalError}};

use crate::{CommandArgument, CommandError, Then, BuildExecute, TaskLogicNoArgs, BuildPropagate, TaskLogic, Execute, Propagate};

use super::LiteralThen;

pub fn literal(literal: &'static str) -> LiteralArgument {
    LiteralArgument { literal, }
}

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

pub struct LiteralExecutor<A, C> {
    argument: A,
    task: C,
}

impl<A, C> Execute for LiteralExecutor<A, C>
where
    A: CommandArgument<()>,
    C: TaskLogicNoArgs,
{
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, bool, CommandError<'a>> {
        let (input, _) = self.argument.parse(input)?;
        match self.task.run() {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v))
        }
    }
}

impl<A, C, T> Propagate<T> for LiteralExecutor<A, C>
where
    T: Copy,
    A: CommandArgument<()>,
    C: TaskLogic<T>,
{
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, bool, CommandError<'a>> {
        let (input, _) = self.argument.parse(input)?;
        match self.task.run(data) {
            Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
            Ok(v) => Ok((input, v))
        }
    }
}
