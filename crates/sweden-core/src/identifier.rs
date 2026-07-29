use core::fmt;

use crate::ValidationError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CanonicalId<'a>(&'a str);

impl<'a> CanonicalId<'a> {
    fn new(value: &'a str, maximum: usize) -> Result<Self, ValidationError> {
        validate(value, maximum)?;
        Ok(Self(value))
    }

    const fn from_project_constant(value: &'a str) -> Self {
        Self(value)
    }

    const fn as_str(self) -> &'a str {
        self.0
    }
}

fn validate(value: &str, maximum: usize) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.len() > maximum {
        return Err(ValidationError::TooLong);
    }

    let mut first = true;
    let mut previous_separator = false;
    for byte in value.bytes() {
        if first {
            if !byte.is_ascii_lowercase() {
                return Err(ValidationError::InvalidStart);
            }
            first = false;
            continue;
        }

        if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            previous_separator = false;
        } else if byte == b'-' || byte == b'.' {
            if previous_separator {
                return Err(ValidationError::InvalidSeparator);
            }
            previous_separator = true;
        } else {
            return Err(ValidationError::InvalidCharacter);
        }
    }

    if previous_separator {
        return Err(ValidationError::InvalidSeparator);
    }
    Ok(())
}

macro_rules! define_identifier {
    (
        $(#[$meta:meta])*
        $name:ident,
        $maximum:expr
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name<'a>(CanonicalId<'a>);

        impl<'a> $name<'a> {
            /// Maximum accepted identifier length in ASCII bytes.
            pub const MAX_LENGTH: usize = $maximum;

            /// Validates a borrowed canonical identifier.
            ///
            /// Canonical values begin with a lowercase ASCII letter. Remaining
            /// bytes are lowercase ASCII letters, digits, `-`, or `.`.
            /// Separators cannot be adjacent or final. Input is never silently
            /// normalized; non-canonical spelling is rejected.
            pub fn new(value: &'a str) -> Result<Self, ValidationError> {
                CanonicalId::new(value, Self::MAX_LENGTH).map(Self)
            }

            /// Returns the canonical borrowed spelling.
            #[must_use]
            pub const fn as_str(self) -> &'a str {
                self.0.as_str()
            }
        }

        impl AsRef<str> for $name<'_> {
            fn as_ref(&self) -> &str {
                self.0.0
            }
        }

        impl fmt::Display for $name<'_> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.0.0)
            }
        }

        impl<'a> TryFrom<&'a str> for $name<'a> {
            type Error = ValidationError;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

define_identifier!(
    /// Canonical descriptive identifier for an upstream Swedish data source.
    SourceId,
    63
);
define_identifier!(
    /// Canonical descriptive identifier for one source operation.
    OperationId,
    127
);
define_identifier!(
    /// Canonical descriptive identifier for a schema family.
    SchemaId,
    95
);
define_identifier!(
    /// Canonical descriptive identifier for a policy family.
    PolicyId,
    95
);
define_identifier!(
    /// Canonical descriptive identifier assigned by an upstream source.
    UpstreamId,
    127
);

/// A project-reviewed source spelling reserved for later registry binding.
///
/// This value proves only that Sweden reserved the canonical source name. It
/// does not prove dossier freshness, policy approval, operation membership, or
/// permission to execute. Downstream crates cannot construct this type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReviewedSourceId(SourceId<'static>);

impl ReviewedSourceId {
    const fn from_project_constant(value: &'static str) -> Self {
        Self(SourceId(CanonicalId::from_project_constant(value)))
    }

    /// Returns the reviewed canonical source spelling as a descriptive ID.
    #[must_use]
    pub const fn as_id(self) -> SourceId<'static> {
        self.0
    }

    /// Returns the reviewed canonical source spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0.as_str()
    }
}

impl AsRef<str> for ReviewedSourceId {
    fn as_ref(&self) -> &str {
        self.0.0.0
    }
}

impl fmt::Display for ReviewedSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.0.0)
    }
}

impl From<ReviewedSourceId> for SourceId<'static> {
    fn from(value: ReviewedSourceId) -> Self {
        value.as_id()
    }
}

/// Closed project-reviewed source spellings.
///
/// These constants reserve names for planned source crates. They are
/// descriptive only and cannot authorize operations or claim an integration
/// is implemented.
pub mod reviewed_sources {
    use super::ReviewedSourceId;

    macro_rules! define_reviewed_sources {
        (
            $(
                $(#[$meta:meta])*
                $name:ident => $value:literal;
            )+
        ) => {
            $(
                $(#[$meta])*
                pub const $name: ReviewedSourceId =
                    ReviewedSourceId::from_project_constant($value);
            )+

            /// Complete closed inventory of project-reviewed source spellings.
            pub const ALL: &[ReviewedSourceId] = &[$($name),+];
        };
    }

    define_reviewed_sources! {
        /// Arbetsförmedlingen JobTech source spelling.
        JOBTECH => "jobtech";
        /// Statistics Sweden source spelling.
        SCB => "scb";
        /// Swedish Tax Agency source spelling.
        SKATTEVERKET => "skatteverket";
        /// Swedish Meteorological and Hydrological Institute source spelling.
        SMHI => "smhi";
        /// Swedish Transport Administration source spelling.
        TRAFIKVERKET => "trafikverket";
    }
}

#[cfg(test)]
mod tests {
    use super::{
        OperationId, PolicyId, SchemaId, SourceId, UpstreamId, reviewed_sources, validate,
    };
    use crate::ValidationError;

    #[test]
    fn every_identifier_kind_accepts_canonical_borrowed_input() {
        assert!(SourceId::new("trafikverket").is_ok());
        assert!(OperationId::new("traffic-message.list-v2").is_ok());
        assert!(SchemaId::new("traffic-message.v1").is_ok());
        assert!(PolicyId::new("public-read.v1").is_ok());
        assert!(UpstreamId::new("api.v2").is_ok());
    }

    #[test]
    fn empty_and_overlong_values_have_stable_categories() {
        assert_eq!(SourceId::new(""), Err(ValidationError::Empty));
        macro_rules! assert_length_boundary {
            ($kind:ident) => {{
                let at_limit = [b'a'; $kind::MAX_LENGTH];
                let over_limit = [b'a'; $kind::MAX_LENGTH + 1];
                let fixtures = (
                    core::str::from_utf8(&at_limit),
                    core::str::from_utf8(&over_limit),
                );
                assert!(fixtures.0.is_ok());
                assert!(fixtures.1.is_ok());
                if let (Ok(at_limit), Ok(over_limit)) = fixtures {
                    assert!($kind::new(at_limit).is_ok());
                    assert_eq!($kind::new(over_limit), Err(ValidationError::TooLong));
                }
            }};
        }
        assert_length_boundary!(SourceId);
        assert_length_boundary!(OperationId);
        assert_length_boundary!(SchemaId);
        assert_length_boundary!(PolicyId);
        assert_length_boundary!(UpstreamId);
    }

    #[test]
    fn starts_and_separators_are_canonical() {
        for value in ["1source", "-source", ".source", "Source"] {
            assert_eq!(SourceId::new(value), Err(ValidationError::InvalidStart));
        }
        for value in ["source-", "source.", "source--id", "source.-id"] {
            assert_eq!(SourceId::new(value), Err(ValidationError::InvalidSeparator));
        }
    }

    #[test]
    fn every_ascii_byte_has_the_expected_admission_result() {
        for byte in u8::MIN..=u8::MAX {
            let candidate = [b'a', byte, b'z'];
            if let Ok(value) = core::str::from_utf8(&candidate) {
                let accepted = byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'-'
                    || byte == b'.';
                assert_eq!(SourceId::new(value).is_ok(), accepted, "byte {byte}");
            }
        }
    }

    #[test]
    fn every_ascii_leading_byte_has_the_expected_admission_result() {
        for byte in u8::MIN..=u8::MAX {
            let candidate = [byte, b'a'];
            if let Ok(value) = core::str::from_utf8(&candidate) {
                assert_eq!(
                    SourceId::new(value).is_ok(),
                    byte.is_ascii_lowercase(),
                    "leading byte {byte}"
                );
            }
        }
    }

    #[test]
    fn non_ascii_and_noncanonical_ascii_are_rejected() {
        for value in [
            "källa",
            "source_name",
            "source/name",
            "source name",
            "a\u{0}b",
        ] {
            assert_eq!(SourceId::new(value), Err(ValidationError::InvalidCharacter));
        }
    }

    #[test]
    fn all_reviewed_constants_are_valid_and_closed() {
        assert_eq!(reviewed_sources::ALL.len(), 5);
        for &reviewed in reviewed_sources::ALL {
            assert_eq!(
                SourceId::new(reviewed.as_str()),
                Ok(reviewed.as_id()),
                "{}",
                reviewed
            );
        }
    }

    #[test]
    fn comparison_and_display_use_canonical_bytes() {
        let earlier = SourceId::new("scb");
        let later = SourceId::new("smhi");
        assert!(earlier.is_ok());
        assert!(later.is_ok());
        if let (Ok(earlier), Ok(later)) = (earlier, later) {
            assert!(earlier < later);
            assert_eq!(std::format!("{earlier}"), "scb");
            assert_eq!(earlier.as_ref(), "scb");
        }
    }

    #[test]
    fn validator_boundaries_match_each_public_ceiling() {
        for maximum in [
            SourceId::MAX_LENGTH,
            OperationId::MAX_LENGTH,
            SchemaId::MAX_LENGTH,
            PolicyId::MAX_LENGTH,
            UpstreamId::MAX_LENGTH,
        ] {
            assert_eq!(validate("", maximum), Err(ValidationError::Empty));
            assert!(validate("a", maximum).is_ok());
        }
    }

    #[test]
    fn borrowed_identifiers_have_only_borrowed_slice_storage() {
        assert_eq!(
            core::mem::size_of::<SourceId<'_>>(),
            core::mem::size_of::<&str>()
        );
        assert_eq!(
            core::mem::size_of::<OperationId<'_>>(),
            core::mem::size_of::<&str>()
        );
    }
}
