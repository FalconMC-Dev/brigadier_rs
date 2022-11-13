//! Minecraft command parsing crate inspired by [`Brigadier`](https://github.com/Mojang/Brigadier)
//! and [`nom`](https://crates.io/crates/nom). This crate tries to combine the easy syntax from
//! `Brigadier` with the performant parsing concepts of `nom`.
//!
//! ### Creating a parser
//!
//! Using the builder pattern, different parsers can be chained together to
//! create a logical tree a command can be propagated through. This is done by
//! using the appropiate parser functions in the root of the crate. Custom
//! parsers can be implemented using [`CommandArgument`] and most other traits.
//! It is recommended to look at parser structs in [`parsers`](self::parsers)
//! for inspiration.
//!
//! ## Example
//!
//! ```no_run
//! # use brigadier_rs::{literal, integer_i32, Then, BuildExecute, ThenHelp};
//! # use std::convert::Infallible;
//! let parser = literal("foo")
//!     .then(
//!         integer_i32("bar")
//!             .build_exec(|ctx: (), bar| {
//!                 println!("Bar is {}", bar);
//!                 Ok::<(), Infallible>(())
//!             })
//!     ).build_exec(|ctx: ()| {
//!         println!("Called foo with no arguments");
//!         Ok::<(), Infallible>(())
//!     });
//! ```
//!
//! This code creates a new parser that can parse commands in the forms of `foo`
//! and `foo <bar>` and can be represented in a tree like this:
//!
//! ```ditaa
//!               +-----------+       +---------+
//!               | i32 (bar) +-----> | Execute |
//!               +-----+-----+       +---------+
//!                     ^
//!                     |
//!                 +---+------+
//! +-----------+   |    (foo) |      +---------+
//! | lit (foo) +-->| Then     +----> | Execute |
//! +-----------+   +----------+      +---------+
//! ```
//!
//! The parser first expects a literal string "foo" as denoted by the
//! `literal("foo")`. After this literal value, an optional integer can be
//! provided. Note that the second argument is optional due to the `Execute`
//! attached to the `Then` that branches to that argument.
//!
//! Unlike Mojang's brigadier, arguments are not collected in a `Context`
//! object. They are instead fed directly into the provided closures. A generic
//! context however is provided so dependents can pass data to the closures
//! after parsing (`ctx` in the example).
//!
//! ### Command help
//!
//! A `HelpArgument` is provided to easily integrate a command into a help
//! system. This is done by calling `help()` on a command parser like this:
//! ```no_run
//! # use brigadier_rs::{literal, integer_i32, Then, BuildExecute, ThenHelp, UsagePrint};
//! # use std::convert::Infallible;
//! let parser = literal("foo")
//!     .then(
//!         integer_i32("bar")
//!             .build_exec(|ctx: (), bar| {
//!                 println!("Bar is {}", bar);
//!                 Ok::<(), Infallible>(())
//!             })
//!     ).build_exec(|ctx| {
//!         println!("Called foo with no arguments");
//!         Ok::<(), Infallible>(())
//!     })
//!     .help("Short description of foo")
//!     .build_exec(|ctx: (), usages: UsagePrint<_>| {
//!         println!("'foo help' was called");
//!         Ok::<(), Infallible>(())
//!     });
//! ```
//!
//! The parser can now return `foo` and `Short description of foo` when queried
//! using [`HelpUsage`], this is useful for collecting a list of commands. This
//! also automatically chains a [`HelpArgument`](self::parsers::HelpArgument)
//! for `foo help`. The `usages` variable is an iterator over all the different
//! syntaxes this parser understands ([`UsagePrint`]). In this example, that
//! would be:
//!
//! - `foo`
//! - `foo <bar>`
//!
//! There will be as many syntaxes as action points (`build_exec`
//! or`build_propagate`) defined. Note that `foo help` is ignored.

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
pub trait CommandParser<S, U>: Execute<S, U> + HelpUsage {}

impl<S, T, U> CommandParser<S, U> for T where T: Execute<S, U> + HelpUsage {}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use nom::Finish;

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
        assert_eq!(
            "Unknown input: /foo true<--[HERE]",
            parser
                .execute(10, "foo true hahah")
                .finish()
                .unwrap_err()
                .convert("/foo true hahah", 10)
        );
        assert_eq!(
            "number too large to fit in target type: ...8945645620<--[HERE]",
            parser
                .execute(10, "foo 12345678945645620")
                .finish()
                .unwrap_err()
                .convert("/foo 12345678945645620", 10)
        );
        assert_eq!(("", ()), parser.execute(12, "foo true").unwrap());
        assert_eq!(("", ()), parser.execute(15, "foo help").unwrap());
    }
}
