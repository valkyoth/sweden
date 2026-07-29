use crate::{ReviewedSourceId, ValidationError};

/// Broad descriptive access class used before operation policy is available.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccessClass {
    /// No caller registration is normally required.
    OpenAnonymous,
    /// The source requires caller registration or an API key.
    OpenRegistered,
    /// Use requires a separate agreement.
    PartnerAgreement,
    /// Access is limited to specifically authorized callers.
    TargetedAuthorization,
    /// The integration has not completed source review.
    ReviewRequired,
}

/// Evidence-backed descriptive implementation state of an agency crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum IntegrationStatus {
    /// Only the repository and API boundary are present.
    Foundation,
    /// The integration is experimental and not production-ready.
    Experimental,
    /// The documented operation set has passed its release gates.
    Stable,
}

/// Static descriptive metadata shared by agency crates and the facade.
///
/// A descriptor is not an operation registration or execution permit. Future
/// executable behavior requires a generated `sweden-registry` entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SourceDescriptor {
    id: ReviewedSourceId,
    display_name: &'static str,
    access: AccessClass,
    status: IntegrationStatus,
}

impl SourceDescriptor {
    /// Creates source metadata when its descriptive fields are consistent.
    ///
    /// A stable integration cannot retain the fail-closed
    /// [`AccessClass::ReviewRequired`] access state.
    pub const fn new(
        id: ReviewedSourceId,
        display_name: &'static str,
        access: AccessClass,
        status: IntegrationStatus,
    ) -> Result<Self, ValidationError> {
        if display_name.is_empty() {
            return Err(ValidationError::Empty);
        }
        if matches!(status, IntegrationStatus::Stable)
            && matches!(access, AccessClass::ReviewRequired)
        {
            return Err(ValidationError::ContradictoryMetadata);
        }
        Ok(Self {
            id,
            display_name,
            access,
            status,
        })
    }

    /// Returns the project-reviewed source spelling.
    #[must_use]
    pub const fn id(self) -> ReviewedSourceId {
        self.id
    }

    /// Returns the reviewed human-readable authority or platform name.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        self.display_name
    }

    /// Returns the broad descriptive source access class.
    #[must_use]
    pub const fn access(self) -> AccessClass {
        self.access
    }

    /// Returns the evidence-backed descriptive integration state.
    #[must_use]
    pub const fn status(self) -> IntegrationStatus {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessClass, IntegrationStatus, SourceDescriptor};
    use crate::{ValidationError, reviewed_sources};

    #[test]
    fn descriptor_rejects_empty_display_name() {
        assert_eq!(
            SourceDescriptor::new(
                reviewed_sources::TRAFIKVERKET,
                "",
                AccessClass::ReviewRequired,
                IntegrationStatus::Foundation,
            ),
            Err(ValidationError::Empty)
        );
    }

    #[test]
    fn descriptor_rejects_stable_unreviewed_access() {
        assert_eq!(
            SourceDescriptor::new(
                reviewed_sources::TRAFIKVERKET,
                "Trafikverket",
                AccessClass::ReviewRequired,
                IntegrationStatus::Stable,
            ),
            Err(ValidationError::ContradictoryMetadata)
        );
    }

    #[test]
    fn descriptor_exposes_consistent_metadata() {
        let descriptor = SourceDescriptor::new(
            reviewed_sources::TRAFIKVERKET,
            "Trafikverket",
            AccessClass::ReviewRequired,
            IntegrationStatus::Foundation,
        );
        assert!(descriptor.is_ok());
        if let Ok(value) = descriptor {
            assert_eq!(value.id(), reviewed_sources::TRAFIKVERKET);
            assert_eq!(value.display_name(), "Trafikverket");
            assert_eq!(value.access(), AccessClass::ReviewRequired);
            assert_eq!(value.status(), IntegrationStatus::Foundation);
        }
    }

    #[test]
    fn stable_reviewed_metadata_is_descriptively_valid() {
        assert!(
            SourceDescriptor::new(
                reviewed_sources::TRAFIKVERKET,
                "Trafikverket",
                AccessClass::OpenRegistered,
                IntegrationStatus::Stable,
            )
            .is_ok()
        );
    }
}
