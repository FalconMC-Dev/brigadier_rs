mod argument;
mod error;
pub mod parsers;

pub use argument::*;
pub use error::{CmdErrorKind, CommandError};
pub use parsers::bool::boolean;
pub use parsers::literal::literal;
pub use parsers::number::{
    float_32, float_64, integer_i16, integer_i32, integer_i64, integer_i8, integer_u16, integer_u32, integer_u64, integer_u8,
};
