use crate::ValidationError;

/// Explicit limits for an upstream response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ResponseBudget {
    max_wire_bytes: u64,
    max_decoded_bytes: u64,
}

impl ResponseBudget {
    /// Creates a budget when both limits are non-zero and decoded bytes are
    /// at least as large as wire bytes.
    pub const fn new(max_wire_bytes: u64, max_decoded_bytes: u64) -> Result<Self, ValidationError> {
        if max_wire_bytes == 0 || max_decoded_bytes == 0 {
            return Err(ValidationError::Zero);
        }
        if max_decoded_bytes < max_wire_bytes {
            return Err(ValidationError::InconsistentLimits);
        }
        Ok(Self {
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
    use super::ResponseBudget;
    use crate::ValidationError;

    #[test]
    fn rejects_zero_and_inverted_limits() {
        assert_eq!(ResponseBudget::new(0, 1), Err(ValidationError::Zero));
        assert_eq!(ResponseBudget::new(1, 0), Err(ValidationError::Zero));
        assert_eq!(
            ResponseBudget::new(2, 1),
            Err(ValidationError::InconsistentLimits)
        );
    }

    #[test]
    fn exposes_valid_limits() {
        let budget = ResponseBudget::new(1_024, 4_096);
        assert!(budget.is_ok());
        if let Ok(value) = budget {
            assert_eq!(value.max_wire_bytes(), 1_024);
            assert_eq!(value.max_decoded_bytes(), 4_096);
        }
    }
}
