use std::marker::PhantomData;

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;

use super::CommandThen;
use crate::{ArgumentMarkerDefaultImpl, ChildUsage, CommandArgument, CommandError, IntoMultipleUsage, Then};

/// Create a boolean parser
pub fn boolean<S>(name: &'static str) -> BoolArgument<S> {
    BoolArgument {
        name,
        source: PhantomData,
    }
}

/// Boolean argument parser.
///
/// This parser has no fields because it simply parses either `"true"` or
/// `"false`".
pub struct BoolArgument<S> {
    name: &'static str,
    source: PhantomData<S>,
}

impl<S> CommandArgument<S, bool> for BoolArgument<S> {
    fn parse<'a>(&self, _source: S, input: &'a str) -> nom::IResult<&'a str, bool, CommandError<'a>> {
        alt((
            |i| {
                let (i, _) = tag_no_case("true")(i)?;
                Ok((i, true))
            },
            |i| {
                let (i, _) = tag_no_case("false")(i)?;
                Ok((i, false))
            },
        ))(input)
    }
}

impl<S> ArgumentMarkerDefaultImpl for BoolArgument<S> {}

impl<S, E> Then<E> for BoolArgument<S> {
    type Output = CommandThen<Self, E, bool, S>;

    fn then(self, executor: E) -> Self::Output {
        CommandThen {
            argument: self,
            executor,
            output: PhantomData,
            source: PhantomData,
        }
    }
}

impl<S> IntoMultipleUsage for BoolArgument<S> {
    type Item = <[&'static str; 3] as IntoMultipleUsage>::Item;

    fn usage_gen(&self) -> Self::Item { self.usage_child().usage_gen() }
}

impl<S> ChildUsage for BoolArgument<S> {
    type Child = [&'static str; 3];

    fn usage_child(&self) -> Self::Child { ["<", self.name, ">"] }
}
