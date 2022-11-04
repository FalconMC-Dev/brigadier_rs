use std::marker::PhantomData;

use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{opt, recognize};
use nom::error::{ErrorKind, FromExternalError};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use super::CommandThen;
use crate::error::CmdErrorKind;
use crate::{ArgumentMarkerDefaultImpl, CommandArgument, CommandError, Then};

/// Numeric argument parser.
///
/// This type can be bounded between a minimum and maximum value (standard MIN
/// and MAX of type `N`). A parse method for type `N` must be provided also.
pub struct NumberArgument<N> {
    pub(crate) min: N,
    pub(crate) max: N,
    pub(crate) parse: fn(&str) -> IResult<&str, N, CommandError>,
}

impl<N> NumberArgument<N>
where
    N: PartialOrd,
{
    /// Set a minimum value for this argument (inclusive).
    pub fn min(mut self, min: N) -> Self {
        debug_assert!(self.max >= min);
        self.min = min;
        self
    }

    /// Set a maximum value for this argument (inclusive).
    pub fn max(mut self, max: N) -> Self {
        debug_assert!(self.min <= max);
        self.max = max;
        self
    }
}

impl<N> CommandArgument<N> for NumberArgument<N>
where
    N: PartialOrd,
{
    /// This implementation may return a [`Failure`](nom::Err::Failure) when the
    /// parsed number is outside of the bounds.
    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, N, CommandError<'a>> {
        let (input, out) = (self.parse)(input)?;
        if out > self.max || out < self.min {
            Err(nom::Err::Failure(CommandError::from_external_error(input, ErrorKind::MapRes, CmdErrorKind::OutOfBounds)))
        } else {
            Ok((input, out))
        }
    }
}

impl<E, N> Then<E> for NumberArgument<N> {
    type Output = CommandThen<Self, E, N>;

    fn then(self, executor: E) -> Self::Output {
        CommandThen {
            argument: self,
            executor,
            output: PhantomData,
        }
    }
}

impl<N> ArgumentMarkerDefaultImpl for NumberArgument<N> {}

fn decimal(input: &str) -> IResult<&str, &str, CommandError> {
    recognize(preceded(opt(tag("-")), many1(terminated(one_of("0123456789"), many0(char('_'))))))(input)
}

fn float(input: &str) -> IResult<&str, &str, CommandError> {
    recognize(preceded(
        opt(tag("-")),
        separated_pair(
            many1(terminated(one_of("0123456789"), many0(char('_')))),
            opt(char('.')),
            opt(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        ),
    ))(input)
}

macro_rules! impl_num {
    ($num:ty => $name:ident = $parse:ident) => {
        impl_num!($num => $name = $parse + decimal);
    };
    ($num:ty => $name:ident = $parse:ident + $num_parse:ident) => {
        #[doc = stringify!(Create a $num argument parser.)]
        pub fn $name() -> NumberArgument<$num> {
            NumberArgument {
                min: <$num>::MIN,
                max: <$num>::MAX,
                parse: $parse,
            }
        }

        fn $parse(input: &str) -> IResult<&str, $num, CommandError> {
            let (input, number) = $num_parse(input)?;
            match ::std::str::FromStr::from_str(number) {
                Err(e) => Err(nom::Err::Failure(CommandError::from_external_error(input, ::nom::error::ErrorKind::MapRes, e))),
                Ok(v) => Ok((input, v)),
            }
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
