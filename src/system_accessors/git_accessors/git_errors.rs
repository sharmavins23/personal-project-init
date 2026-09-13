//! Context definitions for common errors when interacting with git.

use std::path::Path;

// ===== Error Context Definitions =============================================

/// Adds context on git initialization errors.
///
/// # Parameters
///
/// - `path`: The directory path to report.
///
/// # Returns
///
/// - `String`: The context, which can be used to form detailed error messages.
///
pub(super) fn git_init_error_context(path: &Path) -> String {
    let context: String = format!("Failed to initialize git repository at {}", path.display());

    context
}
