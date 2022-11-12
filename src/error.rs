use std::fmt::Display;

use nom::error::{ContextError, ErrorKind, FromExternalError, ParseError};

/// Error returned when parsing a command.
///
/// This mimics `CommandSyntaxException` from the java version. This error type
/// can be used with all nom parsers. Anyhow is used to allow returning any
/// error in execution closures. The type is basically the combination of
/// remanining input and a [`CmdErrorKind`].
#[derive(Debug)]
pub struct CommandError<'a> {
    pub input: &'a str,
    pub kind: CmdErrorKind,
}

impl<'a> ParseError<&'a str> for CommandError<'a> {
    fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
        CommandError {
            input,
            kind: CmdErrorKind::Nom(kind),
        }
    }

    fn append(_: &'a str, _: ErrorKind, other: Self) -> Self { other }

    fn from_char(input: &'a str, c: char) -> Self {
        CommandError {
            input,
            kind: CmdErrorKind::Char(c),
        }
    }
}

impl<'a, E> FromExternalError<&'a str, E> for CommandError<'a>
where
    E: Into<CmdErrorKind>,
{
    fn from_external_error(input: &'a str, _: ErrorKind, e: E) -> Self {
        CommandError {
            input,
            kind: e.into(),
        }
    }
}

impl<'a> ContextError<&'a str> for CommandError<'a> {
    fn add_context(_input: &'a str, _ctx: &'static str, other: Self) -> Self { other }
}

/// Error kinds of [`CommandError`].
#[derive(Debug)]
pub enum CmdErrorKind {
    /// Nom parser expected a character but found a different one instead.
    Char(char),
    /// Error unknown to brigadier-rs.
    External(anyhow::Error),
    /// Any other nom error.
    Nom(ErrorKind),
    /// Can be returned when parsing number arguments
    OutOfBounds,
    /// Input is not empty
    NonEmpty,
}

impl Display for CmdErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmdErrorKind::Char(c) => write!(f, "Expected {}", c),
            CmdErrorKind::External(e) => write!(f, "{}", e.root_cause()),
            CmdErrorKind::Nom(e) => write!(f, "Parse error: {}", e.description()),
            CmdErrorKind::OutOfBounds => write!(f, "Number was out of bounds"),
            CmdErrorKind::NonEmpty => write!(f, "Unknown input"),
        }
    }
}

impl<E> From<E> for CmdErrorKind
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self { CmdErrorKind::External(e.into()) }
}

impl<'a> CommandError<'a> {
    pub fn convert(&self, input: &'a str, context_size: usize) -> String {
        let input_len = input.chars().count();
        let error_len = self.input.chars().count();
        let start = input
            .chars()
            .take((input_len - error_len).saturating_sub(context_size))
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let end = input.len() - self.input.len();
        let input = &input[start..end];

        let prefix = if input_len - error_len > context_size {
            "..."
        } else {
            ""
        };

        format!("{}: {}{}<--[HERE]", self.kind, prefix, input)
    }
}
