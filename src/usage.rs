use std::fmt::{Error, Write};

pub(crate) mod chain;
pub(crate) mod combine;
pub(crate) mod display;
pub(crate) mod prefix;

pub use chain::*;
pub use combine::*;
pub use display::UsagePrint;
pub use prefix::*;

/// A single usage able to write itself to a [`Write`](std::fmt::Write).
pub trait SingleUsage {
    /// Write self to the provided writer.
    fn usage<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
}

/// One or more usages that can be iterated through.
///
/// This trait is similar to the [`Iterator`] trait.
pub trait MultipleUsage {
    /// Write the next usage to the writer and advance the internal iterator.
    ///
    /// If `None` is returned, there are no more usages. The `Result<(), Error>`
    /// can be returned by writing to the writer.
    fn usage_next<W: Write>(&mut self, writer: &mut W) -> Option<Result<(), Error>>;

    /// Returns true if there are usages left, otherwise returns false.
    fn is_next(&self) -> bool;

    /// Chains this `MultipleUsage` with another MultipleUsage.
    fn chain<U2>(self, other: U2) -> Chain<Self, U2>
    where
        Self: Sized,
    {
        Chain {
            left: self,
            right: other,
        }
    }

    /// Returns an iterator of [`String`].
    ///
    /// This allocates as many strings as there are usages.
    /// Convenience method.
    fn string_iter(self) -> UsagePrint<Self>
    where
        Self: Sized,
    {
        UsagePrint { usage: self }
    }
}

/// Implemented on `MultipleUsage` that want to return a single "root" usage
/// in the parser tree.
pub trait ChildUsage {
    /// Usage that is returned.
    type Child: SingleUsage;

    /// Return a new `SingleUsage`, this represents the path from the root up
    /// until this parser.
    fn usage_child(&self) -> Self::Child;
}

/// Type that can build a `MultipleUsage`.
pub trait IntoMultipleUsage {
    /// The returned `MultipleUsage` iterator.
    type Item: MultipleUsage;

    /// Return a new `MultipleUsage` based on self (without consuming self).
    fn usage_gen(&self) -> Self::Item;
}

impl<'a> SingleUsage for &'a str {
    fn usage<W: Write>(&self, writer: &mut W) -> Result<(), Error> { writer.write_str(self) }
}

impl<U> SingleUsage for Option<U>
where
    U: SingleUsage,
{
    fn usage<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        if let Some(usage) = self {
            usage.usage(writer)?;
        }
        Ok(())
    }
}

impl<I, U> MultipleUsage for I
where
    U: SingleUsage,
    I: ExactSizeIterator<Item = U>,
{
    fn usage_next<W: Write>(&mut self, writer: &mut W) -> Option<Result<(), Error>> {
        let next = self.next()?;
        Some(next.usage(writer))
    }

    fn is_next(&self) -> bool { self.len() > 0 }
}

impl<U> IntoMultipleUsage for U
where
    U: SingleUsage + Clone,
{
    type Item = std::iter::Once<U>;

    fn usage_gen(&self) -> Self::Item { std::iter::once(self.clone()) }
}
