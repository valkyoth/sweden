use core::{fmt, num::NonZeroU64};

use crate::ValidationError;

macro_rules! define_version {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(NonZeroU64);

        impl $name {
            /// Creates a version, rejecting zero as an invalid sentinel.
            pub const fn new(value: u64) -> Result<Self, ValidationError> {
                match NonZeroU64::new(value) {
                    Some(value) => Ok(Self(value)),
                    None => Err(ValidationError::Zero),
                }
            }

            /// Returns the non-zero numeric version.
            #[must_use]
            pub const fn get(self) -> u64 {
                self.0.get()
            }
        }

        impl TryFrom<u64> for $name {
            type Error = ValidationError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

define_version!(
    /// Monotonic non-zero version of a decoded schema contract.
    SchemaVersion
);
define_version!(
    /// Monotonic non-zero version of reviewed policy material.
    PolicyVersion
);
define_version!(
    /// Monotonic non-zero version assigned to an upstream contract.
    UpstreamVersion
);

#[cfg(test)]
mod tests {
    use super::{PolicyVersion, SchemaVersion, UpstreamVersion};
    use crate::ValidationError;

    #[test]
    fn every_version_rejects_zero() {
        assert_eq!(SchemaVersion::new(0), Err(ValidationError::Zero));
        assert_eq!(PolicyVersion::new(0), Err(ValidationError::Zero));
        assert_eq!(UpstreamVersion::new(0), Err(ValidationError::Zero));
    }

    #[test]
    fn versions_accept_full_non_zero_range() {
        for value in [1, 2, u64::MAX] {
            assert_eq!(SchemaVersion::new(value).map(SchemaVersion::get), Ok(value));
            assert_eq!(PolicyVersion::new(value).map(PolicyVersion::get), Ok(value));
            assert_eq!(
                UpstreamVersion::new(value).map(UpstreamVersion::get),
                Ok(value)
            );
        }
    }

    #[test]
    fn versions_compare_and_display_without_allocation_in_the_api() {
        let first = SchemaVersion::new(1);
        let second = SchemaVersion::new(2);
        assert!(first.is_ok());
        assert!(second.is_ok());
        if let (Ok(first), Ok(second)) = (first, second) {
            assert!(first < second);
            assert_eq!(std::format!("{second}"), "2");
        }
    }

    #[test]
    fn version_wrappers_have_no_storage_overhead() {
        assert_eq!(
            core::mem::size_of::<SchemaVersion>(),
            core::mem::size_of::<u64>()
        );
        assert_eq!(
            core::mem::size_of::<PolicyVersion>(),
            core::mem::size_of::<u64>()
        );
        assert_eq!(
            core::mem::size_of::<UpstreamVersion>(),
            core::mem::size_of::<u64>()
        );
    }
}
