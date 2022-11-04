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

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::{integer_i32, literal, BuildExecute, Execute, Then};

    #[test]
    fn test_main() {
        let parser = literal("foo")
            .then(integer_i32().max(10).build_exec(|i| {
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
}
