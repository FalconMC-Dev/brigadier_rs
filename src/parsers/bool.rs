use std::marker::PhantomData;

use nom::{branch::alt, bytes::complete::tag_no_case};

use crate::{CommandArgument, CommandError, ArgumentMarkerDefaultImpl, Then};

use super::CommandThen;

#[doc(hidden)]
pub fn boolean() -> BoolArgument {
    BoolArgument
}

pub struct BoolArgument;

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
            }
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
