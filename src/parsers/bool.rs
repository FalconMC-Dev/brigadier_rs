use std::marker::PhantomData;

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;

use super::CommandThen;
use crate::{ArgumentMarkerDefaultImpl, CommandArgument, CommandError, Then, IntoMultipleUsage, ChildUsage};

/// Create a boolean parser
pub fn boolean(name: &'static str) -> BoolArgument {
    BoolArgument { name }
}

/// Boolean argument parser.
///
/// This parser has no fields because it simply parses either `"true"` or
/// `"false`".
pub struct BoolArgument {
    name: &'static str,
}

impl CommandArgument<bool> for BoolArgument {
    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, bool, CommandError<'a>> {
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

impl ArgumentMarkerDefaultImpl for BoolArgument {}

impl<E> Then<E> for BoolArgument {
    type Output = CommandThen<Self, E, bool>;

    fn then(self, executor: E) -> Self::Output {
        CommandThen {
            argument: self,
            executor,
            output: PhantomData,
        }
    }
}

impl IntoMultipleUsage for BoolArgument {
    type Item = <[&'static str; 3] as IntoMultipleUsage>::Item;

    fn usage_gen(&self) -> Self::Item {
        self.usage_child().usage_gen()
    }
}

impl ChildUsage for BoolArgument {
    type Child = [&'static str; 3];

    fn usage_child(&self) -> Self::Child {
        ["<", self.name, ">"]
    }
}
