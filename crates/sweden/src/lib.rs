#![no_std]
#![forbid(unsafe_code)]
#![doc = "Facade for independently publishable Sweden crates."]

/// Shared transport-neutral contracts.
pub use sweden_core as core;

#[cfg(test)]
mod tests {
    #[test]
    fn facade_always_exposes_core() {
        let id = crate::core::SourceId::reviewed("sweden");
        assert_eq!(id.as_str(), "sweden");
    }
}
