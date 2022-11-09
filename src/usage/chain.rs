use crate::MultipleUsage;

/// A chain of two `MultipleUsage` iterators.
///
/// When iterating over this type, it will return all usages from
/// both contained `MultipleUsage` iterators.
#[derive(Debug, Clone, Copy)]
pub struct Chain<U1, U2> {
    pub(crate) left: U1,
    pub(crate) right: U2,
}

impl<U1, U2> MultipleUsage for Chain<U1, U2>
where
    U1: MultipleUsage,
    U2: MultipleUsage,
{
    fn usage_next<W: std::fmt::Write>(&mut self, writer: &mut W) -> Option<Result<(), std::fmt::Error>> {
        match self.left.usage_next(writer) {
            Some(result) => Some(result),
            None => self.right.usage_next(writer),
        }
    }

    fn is_next(&self) -> bool { self.left.is_next() || self.right.is_next() }
}
