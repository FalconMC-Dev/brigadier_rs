use std::fmt::{Write, Error};

mod combine;
mod chain;
mod prefix;

pub use combine::*;
pub use chain::*;
pub use prefix::*;

pub trait SingleUsage {
    fn usage<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
}

pub trait MultipleUsage {
    fn usage_next<W: Write>(&mut self, writer: &mut W) -> Option<Result<(), Error>>;

    fn is_next(&self) -> bool;

    fn chain<U2>(self, other: U2) -> Chain<Self, U2>
    where
        Self: Sized,
    {
        Chain {
            left: self,
            right: other,
        }
    }
}

pub trait ChildUsage {
    type Child: SingleUsage;

    fn usage_child(&self) -> Self::Child;
}

pub trait IntoMultipleUsage {
    type Item: MultipleUsage;

    fn usage_gen(&self) -> Self::Item;
}

impl<'a> SingleUsage for &'a str {
    fn usage<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_str(self)
    }
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

impl<'a, I, U> MultipleUsage for I
where
    U: SingleUsage,
    I: ExactSizeIterator<Item = U>,
{
    fn usage_next<W: Write>(&mut self, writer: &mut W) -> Option<Result<(), Error>> {
        let next = self.next()?;
        Some(next.usage(writer))
    }

    fn is_next(&self) -> bool {
        self.len() > 0
    }
}

impl<U> IntoMultipleUsage for U
where
    U: SingleUsage + Clone,
{
    type Item = std::iter::Once<U>;

    fn usage_gen(&self) -> Self::Item {
        std::iter::once(self.clone())
    }
}
