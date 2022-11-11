use std::borrow::Cow;

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::char;
use nom::error::{ErrorKind, FromExternalError};

use super::{LiteralExecutor, LiteralThen, LiteralThenExecutor};
use crate::{BuildExecute, ChildUsage, CommandArgument, CommandError, Execute, IntoMultipleUsage, TaskLogic, UsagePrint};

pub struct HelpArgument<E> {
    pub(crate) argument: E,
    pub(crate) description: &'static str,
}

pub trait ThenHelp {
    fn help(self, description: &'static str) -> HelpArgument<Self>
    where
        Self: Sized,
    {
        HelpArgument {
            argument: self,
            description,
        }
    }
}

impl<E> CommandArgument<UsagePrint<E::Item>> for HelpArgument<E>
where
    E: CommandArgument<()> + IntoMultipleUsage,
{
    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, UsagePrint<E::Item>, crate::CommandError<'a>> {
        let (input, _) = self.argument.parse(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, _) = tag_no_case("help")(input)?;
        Ok((input, UsagePrint {
            usage: self.argument.usage_gen(),
        }))
    }
}

impl<E, C> BuildExecute<C, HelpExecutor<E, C>> for HelpArgument<E>
where
    E: IntoMultipleUsage,
    C: TaskLogic<UsagePrint<E::Item>>,
{
    fn build_exec(self, task: C) -> HelpExecutor<E, C> { HelpExecutor { help: self, task } }
}

pub struct HelpExecutor<E, C> {
    pub(crate) help: HelpArgument<E>,
    pub(crate) task: C,
}

impl<E, C, U> Execute<U> for HelpExecutor<E, C>
where
    E: Execute<U> + CommandArgument<()> + IntoMultipleUsage,
    C: TaskLogic<UsagePrint<E::Item>, Output = U>,
{
    fn execute<'a>(&self, input: &'a str) -> nom::IResult<&'a str, U, crate::CommandError<'a>> {
        alt((
            |i| {
                let (input, usage) = self.help.parse(i)?;
                match self.task.run(usage) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
            |i| self.help.argument.execute(i),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub struct HelpEntry {
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
}

pub trait HelpUsage {
    fn help(&self) -> HelpEntry;
}

impl<E> HelpUsage for HelpArgument<E>
where
    E: ChildUsage<Child = &'static str>,
{
    fn help(&self) -> HelpEntry {
        HelpEntry {
            name: self.argument.usage_child().into(),
            description: self.description.into(),
        }
    }
}

impl<E, C> HelpUsage for HelpExecutor<E, C>
where
    E: ChildUsage<Child = &'static str>,
{
    fn help(&self) -> HelpEntry {
        HelpEntry {
            name: self.help.argument.usage_child().into(),
            description: self.help.description.into(),
        }
    }
}

impl<A, C> ThenHelp for LiteralExecutor<A, C> {}

impl<A, E, C> ThenHelp for LiteralThenExecutor<A, E, C> {}

impl<A, E> ThenHelp for LiteralThen<A, E> {}
