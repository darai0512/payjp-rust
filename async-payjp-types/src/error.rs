use std::fmt::{Display, Formatter};

/// Error representing a failure to parse a enum
#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unrecognized enum variant")
    }
}

impl std::error::Error for ParseError {}
