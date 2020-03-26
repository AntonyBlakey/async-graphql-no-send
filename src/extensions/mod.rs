//! Extensions for schema
use crate::QueryPath;

mod tracing;

pub use tracing::ApolloTracing;

pub(crate) type BoxExtension = Box<dyn Extension>;

/// Parameters for `Extension::resolve_field_start`
pub struct ResolveInfo<'a> {
    /// Because resolver is concurrent, `Extension::resolve_field_start` and `Extension::resolve_field_end` are
    /// not strictly ordered, so each pair is identified by an id.
    pub resolve_id: usize,

    /// Current path.
    pub path: &'a QueryPath,

    /// Parent type
    pub parent_type: &'a str,

    /// Current return type, is qualified name.
    pub return_type: &'a str,
}

/// Represents a GraphQL extension
#[allow(unused_variables)]
pub trait Extension: Sync + Send + 'static {
    /// Extension name.
    fn name(&self) -> &'static str;

    /// Called at the begin of the parse.
    fn parse_start(&self, query_source: &str) {}

    /// Called at the end of the parse.
    fn parse_end(&self) {}

    /// Called at the begin of the validation.
    fn validation_start(&self) {}

    /// Called at the end of the validation.
    fn validation_end(&self) {}

    /// Called at the begin of the execution.
    fn execution_start(&self) {}

    /// Called at the end of the execution.
    fn execution_end(&self) {}

    /// Called at the begin of the resolve field.
    fn resolve_field_start(&self, info: &ResolveInfo<'_>) {}

    /// Called at the end of the resolve field.
    fn resolve_field_end(&self, resolve_id: usize) {}

    /// Get the results
    fn result(&self) -> serde_json::Value;
}
