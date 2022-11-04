use nom::IResult;

use crate::CommandError;

mod argument_impl;

/// A parser for a command argument type.
///
/// This trait is implemented by all argument types. Implementors *must* not
/// account for starting or ending spaces. It is a good idea to use as many nom
/// combinators as possible here.
///
/// # Example
/// ```no_run
/// # use nom::IResult;
/// # use nom::branch::alt;
/// # use nom::bytes::complete::tag_no_case;
/// use brigadier_rs::CommandError;
/// use brigadier_rs::CommandArgument;
///
/// pub struct BoolArgument;
///
/// impl CommandArgument<bool> for BoolArgument {
///     fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, bool, CommandError<'a>> {
///         alt((
///             |i| {
///                 let (i, _) = tag_no_case("true")(i)?;
///                 Ok((i, true))
///             },
///             |i| {
///                 let (i, _) = tag_no_case("false")(i)?;
///                 Ok((i, false))
///             },
///         ))(input)
///     }
/// }
/// ```
pub trait CommandArgument<A> {
    /// Where the magic happens! This is what ends up extracting type `A` from
    /// the raw string input.
    ///
    /// # Note
    /// Do not let this function return a [`Failure`](::nom::Err::Failure)!!
    /// Doing so is not recommended and will most likely cause unwanted
    /// side-effects elsewhere.
    ///
    /// # Example
    /// ```rust
    /// # use brigadier_rs::CommandArgument;
    /// # use brigadier_rs::literal;
    /// let parser = literal("foo"); // new literal argument parser
    ///
    /// assert_eq!(("", ()), parser.parse("foo").unwrap()); // `LiteralArgument` implements `CommandArgument<()>`
    /// ```
    fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, A, CommandError<'a>>;
}

/// Chaining extension trait for argument type parsers ([`CommandArgument`]).
///
/// Implementors of this trait need to specify a branching parser. There are two
/// default branching parsers:
/// - [`LiteralThen`] supports any parser that returns the unit type `()` as its
///   argument type.
/// - [`CommandThen`] supports any other parser (parsers that return any type).
///
/// It is generally sufficient to use one of these two types.
///
/// # Note
/// Implementors should implement this type over any unbounded type `E`, type
/// bounds for this type `E` should be put on the [`Output`](Then::Output) type
/// instead.
///
/// [`LiteralThen`]: super::parsers::LiteralThen
/// [`CommandThen`]: super::parsers::CommandThen
pub trait Then<E> {
    /// Branching parser used by this type.
    type Output;

    /// Returns the braching parser [Self::Output] initialized with `self` and a
    /// child argument.
    ///
    /// See [`ThenWrapper`](super::parsers::ThenWrapper).
    fn then(self, executor: E) -> Self::Output;
}

/// Command logic builder type (without propagation).
///
/// This trait should be implemented by argument type parsers and branching
/// parsers.
///
/// Appends the logic that should be executed when the command parser reaches
/// the current parser block. This allows parsers to be fed with command input
/// as well as allow arguments to become optional.
pub trait BuildExecute<C, O> {
    fn build_exec(self, task: C) -> O;
}

/// Command logic builder type (with propagation).
///
/// This trait should be implemented by argument type parsers and branching
/// parsers.
///
/// Appends the logic that should be executed when the command parser reaches
/// the current parser, taking into account arguments that have been parsed
/// before. This allows the final closure to take all the arguments defined up
/// to this point and let the compiler infer their type. It also allows end
/// arguments to be optional.
///
/// # Note
/// The propagated type `T` is usually required to be `Copy`, note that any
/// reference is `Copy`.
pub trait BuildPropagate<C, T, O> {
    fn build_propagate(self, task: C) -> O;
}

/// Default argument marker trait.
///
/// Any command argument type implementing this trait will automatically
/// implement [`BuildExecute`] and [`BuildPropagate`].
pub trait ArgumentMarkerDefaultImpl {}

/// Command logic definition trait (without arguments).
///
/// Executes command logic.
///
/// `Fn`-closures implement this trait.
pub trait TaskLogicNoArgs {
    /// Error type this logic may return
    type Error: Into<anyhow::Error>;
    /// Return value upon success
    type Output;

    fn run(&self) -> Result<Self::Output, Self::Error>;
}

/// Command logic definition trait (with arguments).
///
/// Executes command logic. This type receives propagated arguments from parsers
/// upstream.
///
/// `Fn`-closures implement this trait.
pub trait TaskLogic<O> {
    /// Error type this logic may return
    type Error: Into<anyhow::Error>;
    /// Return value upon success
    type Output;

    fn run(&self, args: O) -> Result<Self::Output, Self::Error>;
}

/// Command parser execution entrypoint.
pub trait Execute<U> {
    fn execute<'a>(&self, input: &'a str) -> IResult<&'a str, U, CommandError<'a>>;
}

/// Command parser propagation entrypoint.
///
/// Generally not used by the end user.
pub trait Propagate<T, U> {
    fn propagate<'a>(&self, input: &'a str, data: T) -> IResult<&'a str, U, CommandError<'a>>;
}
