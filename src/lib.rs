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

pub trait CommandParser<U>: Execute<U> + HelpUsage {}

impl<T, U> CommandParser<U> for T where T: Execute<U> + HelpUsage {}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::parsers::help::ThenHelp;
    use crate::{boolean, integer_i32, literal, BuildExecute, CommandParser, Execute, Then, UsagePrint};

    #[test]
    fn test_main() {
        let parser = literal("foo")
            .then(integer_i32("bar").max(10).build_exec(|i| {
                println!("Found integer {}", i);
                Ok::<(), Infallible>(())
            }))
            .build_exec(|| {
                println!("Didn't wanna give us a value aye?");
                Ok::<(), Infallible>(())
            });

        assert!(parser.execute("foo 13").is_err());
        assert_eq!(("", ()), parser.execute("foo").unwrap());
    }

    #[test]
    fn test_usage() {
        let parser: Box<dyn CommandParser<()>> = Box::new(
            literal("foo")
                .then(integer_i32("bar").max(10).build_exec(|i| {
                    println!("Found integer {}", i);
                    Ok::<(), Infallible>(())
                }))
                .then(boolean("buzz").build_exec(|_| Ok::<(), Infallible>(())))
                .build_exec(|| {
                    println!("Didn't wanna give us a value aye?");
                    Ok::<(), Infallible>(())
                })
                .help("Test description")
                .build_exec(|mut usages: UsagePrint<_>| {
                    assert_eq!("foo", usages.next().unwrap().unwrap());
                    assert_eq!("foo <bar>", usages.next().unwrap().unwrap());
                    assert_eq!("foo <buzz>", usages.next().unwrap().unwrap());
                    Ok::<(), Infallible>(())
                }),
        );

        let help = parser.help();
        println!("{:?}", help);

        assert_eq!(("", ()), parser.execute("foo").unwrap());
        assert_eq!(("", ()), parser.execute("foo true").unwrap());
        assert_eq!(("", ()), parser.execute("foo help").unwrap());
    }
}
