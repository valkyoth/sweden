/// A transport-neutral HTTP method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Method {
    /// Retrieve a representation without changing upstream state.
    Get,
    /// Submit a bounded request representation.
    Post,
}
