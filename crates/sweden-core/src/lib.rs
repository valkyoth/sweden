#![no_std]
#![forbid(unsafe_code)]
#![doc = "Shared, dependency-free contracts for Sweden agency crates."]

/// Stable identifier for an upstream Swedish public-data source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceId(&'static str);

impl SourceId {
    /// Creates a source identifier from a reviewed static value.
    #[must_use]
    pub const fn reviewed(value: &'static str) -> Self {
        Self(value)
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Broad access class used before operation-specific policy is available.
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

/// Evidence-backed implementation state of an agency crate.
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

/// Static metadata shared by agency crates and the facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceDescriptor {
    /// Canonical source identifier.
    pub id: SourceId,
    /// Human-readable authority or platform name.
    pub display_name: &'static str,
    /// Broad source access class.
    pub access: AccessClass,
    /// Current evidence-backed implementation state.
    pub status: IntegrationStatus,
}

/// A transport-neutral HTTP method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Method {
    /// Retrieve a representation without changing upstream state.
    Get,
    /// Submit a bounded request representation.
    Post,
}

/// Explicit limits for an upstream response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseBudget {
    max_wire_bytes: u64,
    max_decoded_bytes: u64,
}

impl ResponseBudget {
    /// Creates a budget when both limits are non-zero and decoded bytes are
    /// at least as large as wire bytes.
    pub const fn new(max_wire_bytes: u64, max_decoded_bytes: u64) -> Option<Self> {
        if max_wire_bytes == 0 || max_decoded_bytes == 0 || max_decoded_bytes < max_wire_bytes {
            return None;
        }
        Some(Self {
            max_wire_bytes,
            max_decoded_bytes,
        })
    }

    /// Returns the maximum accepted wire bytes.
    #[must_use]
    pub const fn max_wire_bytes(self) -> u64 {
        self.max_wire_bytes
    }

    /// Returns the maximum accepted decoded bytes.
    #[must_use]
    pub const fn max_decoded_bytes(self) -> u64 {
        self.max_decoded_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{ResponseBudget, SourceId};

    #[test]
    fn source_id_preserves_reviewed_value() {
        let id = SourceId::reviewed("trafikverket");
        assert_eq!(id.as_str(), "trafikverket");
    }

    #[test]
    fn response_budget_rejects_zero_and_inverted_limits() {
        assert_eq!(ResponseBudget::new(0, 1), None);
        assert_eq!(ResponseBudget::new(1, 0), None);
        assert_eq!(ResponseBudget::new(2, 1), None);
    }

    #[test]
    fn response_budget_exposes_valid_limits() {
        let budget = ResponseBudget::new(1_024, 4_096);
        assert!(budget.is_some());
        if let Some(value) = budget {
            assert_eq!(value.max_wire_bytes(), 1_024);
            assert_eq!(value.max_decoded_bytes(), 4_096);
        }
    }
}
