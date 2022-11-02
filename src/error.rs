use nom::error::{ErrorKind, FromExternalError, ParseError};

#[derive(Debug)]
pub struct CommandError<'a> {
    input: &'a str,
    kind: CmdErrorKind,
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

#[derive(Debug)]
pub enum CmdErrorKind {
    Char(char),
    External(anyhow::Error),
    Nom(ErrorKind),
    OutOfBounds,
}

impl<E> From<E> for CmdErrorKind
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self { CmdErrorKind::External(e.into()) }
}
