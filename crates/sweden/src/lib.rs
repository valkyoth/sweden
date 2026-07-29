#![no_std]
#![forbid(unsafe_code)]
#![doc = "Facade for independently publishable Sweden crates."]

/// Shared transport-neutral contracts.
pub use sweden_core as core;

#[cfg(test)]
mod tests {
    #[test]
    fn facade_always_exposes_core() {
        let id = crate::core::SourceId::new("sweden");
        assert!(id.is_ok());
        if let Ok(value) = id {
            assert_eq!(value.as_str(), "sweden");
        }
    }
}
