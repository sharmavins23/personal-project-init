//! Context definitions for common errors when interacting with files.

use std::path::Path;

// ===== Error Context Definitions =============================================

/// Adds context on file read errors.
///
/// # Parameters
///
/// - `path`: The filepath to report.
///
/// # Returns
///
/// - `String`: The context, which can be used to form detailed error messages.
///
pub(super) fn file_read_error_context(path: &Path) -> String {
    let context: String = format!("Failed to read file at {}", path.display());

    context
}

/// Adds context on file write errors.
///
/// # Parameters
///
/// - `path`: The filepath to report.
///
/// # Returns
///
/// - `String`: The context, which can be used to form detailed error messages.
///
pub(super) fn file_write_error_context(path: &Path) -> String {
    let context: String = format!("Failed to write file at {}", path.display());

    context
}
