//! Context definitions for common errors when interacting with git.

use git2::Repository;
use indoc::formatdoc;
use std::path::Path;

// ===== Error Context Definitions =============================================

/// Adds context on git staging errors.
///
/// # Parameters
///
/// - `path`: The directory path to report.
///
/// # Returns
///
/// - `String`: The context, which can be used to form detailed error messages.
///
pub(in crate::system_accessors::git_accessors) fn git_add_all_error_context(
    repository: &Repository,
) -> String {
    let context: String = format!(
        "Failed to stage changes in the git repository at `{}`.",
        repository.path().display()
    );

    context
}

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
pub(in crate::system_accessors::git_accessors) fn git_init_error_context(path: &Path) -> String {
    let context: String = format!(
        "Failed to initialize git repository at `{}`.",
        path.display()
    );

    context
}

/// Adds context on git credential lookup errors.
///
/// # Parameters
///
/// - `path`: The repository path to report.
///
/// # Returns
///
/// - `String`: The context, which can be used to form detailed error messages.
///
pub(in crate::system_accessors::git_accessors) fn git_signature_error_context(
    repository: &Repository,
) -> String {
    let context: String = formatdoc! {"
        Failed to read git credentials for the repository at `{}`.

        You may need to set `user.name` and `user.email`. Example usage:

            git config --global user.name \"Your Name\"
            git config --global user.email \"you@example.com\"
    ", repository.path().display()};

    context
}
