use std::fmt::Error;

use crate::MultipleUsage;

/// Wrapper of `MultipleUsage` iterator.
///
/// This iterator produces [`String`]s.
pub struct UsagePrint<U> {
    pub(crate) usage: U,
}

impl<U> Iterator for UsagePrint<U>
where
    U: MultipleUsage,
{
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::new();
        Some(match self.usage.usage_next(&mut result)? {
            Ok(_) => Ok(result),
            Err(e) => Err(e),
        })
    }
}
