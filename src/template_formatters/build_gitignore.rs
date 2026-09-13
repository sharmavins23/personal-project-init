//! Template builder for a project's `.gitignore` file.

// ===== Template Data =========================================================

/// String for `.gitignore` contents.
const GITIGNORE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/.gitignore"));

// ===== Builder Functions =====================================================

/// Builds the contents of a generated `.gitignore` file.
///
/// # Returns
///
/// - [`str`]: The contents of the `.gitignore` file.
///
#[allow(dead_code)]
#[must_use]
pub(crate) const fn build_gitignore() -> &'static str {
    GITIGNORE_TEMPLATE
}
