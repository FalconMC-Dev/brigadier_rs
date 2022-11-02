use std::marker::PhantomData;

use nom::{IResult, character::complete::char, combinator::{map_res, recognize, opt}, sequence::{preceded, terminated, separated_pair}, bytes::complete::tag, multi::{many1, many0}, character::complete::one_of};

use crate::{CommandError, CommandArgument, error::CmdErrorKind, Then, ArgumentMarkerDefaultImpl};

use super::CommandThen;

pub struct IntegerArgument<N> {
    pub(crate) min: N,
    pub(crate) max: N,
    pub(crate) parse: fn(&str) -> IResult<&str, N, CommandError>,
}

impl<N> IntegerArgument<N>
where
    N: PartialOrd,
{
    pub fn min(mut self, min: N) -> Self {
        debug_assert!(self.max >= min);
        self.min = min;
        self
    }

    pub fn max(mut self, max: N) -> Self {
        debug_assert!(self.min <= max);
        self.max = max;
        self
    }
}

impl<N> CommandArgument<N> for IntegerArgument<N>
where
    N: PartialOrd,
{
    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, N, CommandError<'a>> {
        map_res(
            self.parse,
            |out| {
                if out > self.max || out < self.min {
                    return Err(CmdErrorKind::OutOfBounds);
                }
                Ok(out)
            }
        )(input)
    }
}

impl<E, N> Then<E> for IntegerArgument<N> {
    type Output = CommandThen<Self, E, N>;

    fn then(self, executor: E) -> Self::Output {
        CommandThen {
            argument: self,
            executor,
            output: PhantomData,
        }
    }
}

impl<N> ArgumentMarkerDefaultImpl for IntegerArgument<N> {}

fn decimal(input: &str) -> IResult<&str, &str, CommandError> {
    recognize(
        preceded(
            opt(tag("-")),
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )
        )
    )(input)
}

fn float(input: &str) -> IResult<&str, &str, CommandError> {
    recognize(
        preceded(
            opt(tag("-")),
            separated_pair(
                many1(terminated(one_of("0123456789"), many0(char('_')))),
                opt(char('.')),
                opt(many1(terminated(one_of("0123456789"), many0(char('_'))))),
            )
        )
    )(input)
}

macro_rules! impl_num {
    ($num:ty => $name:ident = $parse:ident) => {
        impl_num!($num => $name = $parse + decimal);
    };
    ($num:ty => $name:ident = $parse:ident + $num_parse:ident) => {
        pub fn $name() -> IntegerArgument<$num> {
            IntegerArgument {
                min: <$num>::MIN,
                max: <$num>::MAX,
                parse: $parse,
            }
        }

        fn $parse(input: &str) -> IResult<&str, $num, CommandError> {
            map_res($num_parse, |out: &str| ::std::str::FromStr::from_str(out))(input)
        }
    };
}

impl_num!(i8 => integer_i8 = parse_i8);
impl_num!(i16 => integer_i16 = parse_i16);
impl_num!(i32 => integer_i32 = parse_i32);
impl_num!(i64 => integer_i64 = parse_i64);
impl_num!(u8 => integer_u8 = parse_u8);
impl_num!(u16 => integer_u16 = parse_u16);
impl_num!(u32 => integer_u32 = parse_u32);
impl_num!(u64 => integer_u64 = parse_u64);
impl_num!(f32 => float_32 = parse_f32 + float);
impl_num!(f64 => float_64 = parse_f64 + float);
