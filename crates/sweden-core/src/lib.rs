#![no_std]
#![forbid(unsafe_code)]
#![doc = "Shared, dependency-free contracts for Sweden agency crates."]
#![doc = ""]
#![doc = "Identifiers in this crate are validated descriptive values. They do"]
#![doc = "not prove source review, policy approval, registry membership, or"]
#![doc = "permission to execute an operation."]
#![doc = ""]
#![doc = "Dynamic identifiers remain usable without allocation:"]
#![doc = "```"]
#![doc = "use sweden_core::SourceId;"]
#![doc = "let source = SourceId::new(\"community-catalog\")?;"]
#![doc = "assert_eq!(source.as_str(), \"community-catalog\");"]
#![doc = "# Ok::<(), sweden_core::ValidationError>(())"]
#![doc = "```"]
#![doc = ""]
#![doc = "Downstream code cannot turn a descriptive ID into a reviewed source:"]
#![doc = "```compile_fail"]
#![doc = "use sweden_core::{ReviewedSourceId, SourceId};"]
#![doc = "if let Ok(source) = SourceId::new(\"trafikverket\") {"]
#![doc = "    let _reviewed = ReviewedSourceId(source);"]
#![doc = "}"]
#![doc = "```"]
#![doc = ""]
#![doc = "Execution authority is intentionally not a core capability:"]
#![doc = "```compile_fail"]
#![doc = "use sweden_core::AuthorizedExecution;"]
#![doc = "```"]

#[cfg(test)]
extern crate std;

mod budget;
mod identifier;
mod method;
mod source;
mod validation;
mod version;

pub use budget::ResponseBudget;
pub use identifier::{
    OperationId, PolicyId, ReviewedSourceId, SchemaId, SourceId, UpstreamId, reviewed_sources,
};
pub use method::Method;
pub use source::{AccessClass, IntegrationStatus, SourceDescriptor};
pub use validation::ValidationError;
pub use version::{PolicyVersion, SchemaVersion, UpstreamVersion};
