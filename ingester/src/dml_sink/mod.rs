//! A composable abstraction for processing write requests.

mod r#trait;
pub(crate) use r#trait::*;

#[cfg(test)]
pub(crate) mod mock_sink;
