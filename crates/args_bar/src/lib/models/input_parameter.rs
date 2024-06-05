use super::Encoding;
use std::sync::Arc;

// ANCHOR: model
/// Represents some piece of information
/// that could be collected from the args bar
#[derive(Debug, Clone)]
pub struct InputParameter {
    /// simple string identifier
    pub(crate) name: Arc<str>,

    /// more detailed explication of the
    /// data that is being collected
    pub(crate) desc: Arc<str>,

    /// serialized string of a value to
    /// use if not collected
    pub(crate) default: Arc<str>,

    /// reports on if the data being
    /// requested is absolutely needed
    pub(crate) required: bool,

    /// describes the type of value collected
    pub(crate) encoding: Encoding,
}
// ANCHOR_END: model
