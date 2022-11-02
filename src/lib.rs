mod argument;
mod error;
pub mod parsers;

pub use error::{CommandError, CmdErrorKind};
pub use argument::*;
pub use parsers::{literal::literal, bool::boolean, number::{integer_i8, integer_u8, integer_i16, integer_u16, integer_i32, integer_u32, integer_i64, integer_u64, float_32, float_64}};

