/// The `ParseError` enum is a collection of all the possible reasons
/// an enum can fail to parse from a string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ParseError {
    VariantNotFound,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        // We could use our macro here, but this way we don't take a dependency on the
        // macros crate.
        match self {
            ParseError::VariantNotFound => write!(f, "Matching variant not found"),
        }
    }
}
