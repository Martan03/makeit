use crate::err::args_err::ArgsErr;

/// Struct for parsing arguments
pub struct Args {}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, ArgsErr> {
        Ok(Self {})
    }
}
