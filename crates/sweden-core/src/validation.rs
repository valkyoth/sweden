/// Stable, payload-free validation failure categories.
///
/// The variants intentionally retain no rejected input, which keeps errors
/// bounded and avoids reflecting caller-controlled identifiers into logs.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ValidationError {
    /// A required value was empty.
    Empty,
    /// A value exceeded its type's documented byte ceiling.
    TooLong,
    /// An identifier did not begin with a lowercase ASCII letter.
    InvalidStart,
    /// An identifier contained a non-canonical byte.
    InvalidCharacter,
    /// An identifier ended with a separator or contained adjacent separators.
    InvalidSeparator,
    /// A version or limit was zero where zero has no valid meaning.
    Zero,
    /// Two individually valid limits had an invalid relationship.
    InconsistentLimits,
    /// Descriptive metadata contained a fail-open contradiction.
    ContradictoryMetadata,
}

#[cfg(test)]
mod tests {
    use super::ValidationError;

    #[test]
    fn categories_carry_no_input_payload() {
        assert_eq!(core::mem::size_of::<ValidationError>(), 1);
    }
}
