//! Model for the high-level context shared across a single PPI execution.

use crate::models::argument_input::ArgumentInput;
use std::path::PathBuf;

// ===== Definitions ===========================================================

/// Holds the resolved inputs for a single PPI execution.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ExecutionContext {
    /// The raw (CLI) arguments.
    pub argument_input: ArgumentInput,

    /// The directory that PPI was invoked from.
    pub directory: PathBuf,

    /// The project's title, derived from the directory name.
    pub title: String,
}
