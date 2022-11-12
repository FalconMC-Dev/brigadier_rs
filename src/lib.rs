mod argument;
mod error;
pub mod parsers;
mod usage;

pub use argument::*;
pub use error::{CmdErrorKind, CommandError};
pub use parsers::bool::boolean;
pub use parsers::help::{HelpEntry, HelpUsage, ThenHelp};
pub use parsers::literal::literal;
pub use parsers::number::{
    float_32, float_64, integer_i16, integer_i32, integer_i64, integer_i8, integer_u16, integer_u32, integer_u64, integer_u8,
};
pub use usage::*;

/// Parser trait combination of `Execute` and `HelpUsage`.
///
/// This is the result of combining one or more parsers.
pub trait CommandParser<S, U>: Execute<S, U> + HelpUsage + Send + Sync {}

impl<S, T, U> CommandParser<S, U> for T where T: Execute<S, U> + HelpUsage + Send + Sync {}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::parsers::help::ThenHelp;
    use crate::{boolean, integer_i32, literal, BuildExecute, CommandParser, Execute, Then, UsagePrint};

    #[test]
    fn test_main() {
        let parser = literal("foo")
            .then(integer_i32("bar").max(10).build_exec(|_, i| {
                println!("Found integer {}", i);
                Ok::<(), Infallible>(())
            }))
            .build_exec(|_| {
                println!("Didn't wanna give us a value aye?");
                Ok::<(), Infallible>(())
            });

        assert!(parser.execute((), "foo 13").is_err());
        assert_eq!(("", ()), parser.execute((), "foo").unwrap());
    }

    #[test]
    fn test_usage() {
        let parser: Box<dyn CommandParser<i32, ()>> = Box::new(
            literal("foo")
                .then(integer_i32("bar").max(10).build_exec(|x, i| {
                    println!("Found integer {} for source {}", i, x);
                    Ok::<(), Infallible>(())
                }))
                .then(boolean("buzz").build_exec(|_, _| Ok::<(), Infallible>(())))
                .build_exec(|_| {
                    println!("Didn't wanna give us a value aye?");
                    Ok::<(), Infallible>(())
                })
                .help("Test description")
                .build_exec(|_, mut usages: UsagePrint<_>| {
                    assert_eq!("foo", usages.next().unwrap().unwrap());
                    assert_eq!("foo <bar>", usages.next().unwrap().unwrap());
                    assert_eq!("foo <buzz>", usages.next().unwrap().unwrap());
                    assert!(usages.next().is_none());
                    Ok::<(), Infallible>(())
                }),
        );

        let help = parser.help();
        println!("{:?}", help);

        assert_eq!(("", ()), parser.execute(10, "foo").unwrap());
        assert_eq!(("", ()), parser.execute(12, "foo -456").unwrap());
        assert_eq!(("", ()), parser.execute(12, "foo true").unwrap());
        assert_eq!(("", ()), parser.execute(15, "foo help").unwrap());
    }
}
