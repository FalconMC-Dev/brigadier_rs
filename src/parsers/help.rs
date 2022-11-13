use std::borrow::Cow;
use std::marker::PhantomData;

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::char;
use nom::error::{ErrorKind, FromExternalError};

use super::{LiteralExecutor, LiteralThen, LiteralThenExecutor};
use crate::{BuildExecute, ChildUsage, CommandArgument, CommandError, Execute, IntoMultipleUsage, TaskLogic, UsagePrint};

/// Parser that parses a root command followed by `" help"`.
///
/// This parser produces an iterator over all the different usages the root
/// parser can parse.
pub struct HelpArgument<S, E> {
    pub(crate) argument: E,
    pub(crate) description: &'static str,
    pub(crate) source: PhantomData<S>,
}

/// Type that can produce a usage list and help command.
///
/// This should conventionally only be implemented on literal types (types that
/// implement [`CommandArgument<()>`](crate::CommandArgument)).
pub trait ThenHelp<S> {
    /// Attach this parser to a [`HelpArgument`] and a description, the help
    /// name will be the usage returned by the root parser.
    fn help(self, description: &'static str) -> HelpArgument<S, Self>
    where
        Self: Sized,
    {
        HelpArgument {
            argument: self,
            description,
            source: PhantomData,
        }
    }
}

impl<S, E> CommandArgument<S, UsagePrint<E::Item>> for HelpArgument<S, E>
where
    E: CommandArgument<S, ()> + IntoMultipleUsage,
{
    fn parse<'a>(&self, source: S, input: &'a str) -> nom::IResult<&'a str, UsagePrint<E::Item>, crate::CommandError<'a>> {
        let (input, _) = self.argument.parse(source, input)?;
        let (input, _) = char(' ')(input)?;
        let (input, _) = tag_no_case("help")(input)?;
        Ok((input, UsagePrint {
            usage: self.argument.usage_gen(),
        }))
    }
}

impl<S, E, C> BuildExecute<C, HelpExecutor<S, E, C>> for HelpArgument<S, E>
where
    E: IntoMultipleUsage,
    C: TaskLogic<S, UsagePrint<E::Item>>,
{
    fn build_exec(self, task: C) -> HelpExecutor<S, E, C> { HelpExecutor { help: self, task } }
}

impl<E, S, U> Execute<S, U> for HelpArgument<S, E>
where
    E: Execute<S, U>,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> nom::IResult<&'a str, U, CommandError<'a>> { self.argument.execute(source, input) }
}

/// Executor for a custom help message.
///
/// Similar to [`DefaultExecutor`](crate::parsers::DefaultExecutor).
pub struct HelpExecutor<S, E, C> {
    pub(crate) help: HelpArgument<S, E>,
    pub(crate) task: C,
}

impl<E, C, U, S> Execute<S, U> for HelpExecutor<S, E, C>
where
    E: Execute<S, U> + CommandArgument<S, ()> + IntoMultipleUsage,
    C: TaskLogic<S, UsagePrint<E::Item>, Output = U>,
    S: Copy,
{
    fn execute<'a>(&self, source: S, input: &'a str) -> nom::IResult<&'a str, U, crate::CommandError<'a>> {
        alt((
            |i| {
                let (input, usage) = self.help.parse(source, i)?;
                match self.task.run(source, usage) {
                    Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, e))),
                    Ok(v) => Ok((input, v)),
                }
            },
            |i| self.help.argument.execute(source, i),
        ))(input)
    }
}

/// Name and description of a command.
///
/// This is primarily meant to generate quick overviews of available parser
/// commands.
#[derive(Debug, Clone)]
pub struct HelpEntry {
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
}

/// Type that returns a [`HelpEntry`].
pub trait HelpUsage {
    /// Returns name and description of this type in a [`HelpEntry`].
    fn help(&self) -> HelpEntry;
}

impl<S, E> HelpUsage for HelpArgument<S, E>
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

impl<S, E, C> HelpUsage for HelpExecutor<S, E, C>
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

impl<A, C, S> ThenHelp<S> for LiteralExecutor<A, C, S> {}

impl<A, E, C, S> ThenHelp<S> for LiteralThenExecutor<A, E, C, S> {}

impl<A, E, S> ThenHelp<S> for LiteralThen<A, E, S> {}
