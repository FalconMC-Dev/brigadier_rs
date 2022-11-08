use std::fmt::Write;

use crate::{MultipleUsage, SingleUsage};

pub fn prefix<P, U>(prefix: P, usage: U) -> Prefix<P, U>
{
    Prefix {
        prefix,
        usage,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Prefix<P, U> {
    prefix: P,
    usage: U,
}

impl<P, U> MultipleUsage for Prefix<P, U>
where
    P: SingleUsage,
    U: MultipleUsage,
{
    fn usage_next<W: Write>(&mut self, writer: &mut W) -> Option<Result<(), std::fmt::Error>> {
        if self.usage.is_next() {
            if let Err(e) = self.prefix.usage(writer) {
                return Some(Err(e));
            }
        }
        self.usage.usage_next(writer)
    }

    fn is_next(&self) -> bool {
        self.usage.is_next()
    }
}
